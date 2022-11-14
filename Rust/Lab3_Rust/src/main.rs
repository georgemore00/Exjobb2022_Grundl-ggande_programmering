use array_init::array_init;

#[derive(Debug, Clone)]
struct Medicine {
    name: String,
    sizes: [i32; 10],
    balances: [i32; 10],
    nr_of_sizes: i32
}

/*Todo - ändra till 10000
Måste vara global variabel annars måste man skicka variabeln som input-parameter i flera metoder
eller hårkoda värdet 10000 i metoderna

usize då värdet måste vara unsigned(positivt) om man vill sätta array storlek med MAX_SIZE
*/

const MAX_SIZE: usize = 10;

fn main() {
    let mut nr_of_medicines: i32 = 0;
    let mut nr_of_found_in_search: i32 = 0;
    let mut found_index: i32 = -1;

    /*array_init pga att det inte går att initiera en array av structar om structen har en datatyp som inte implementerar Copy Traiten
    name: String implementerar inte Copy, vilket är metoden som Rust kommer att använda 
    då den itererar över varje Index och kallar på Copy när vi skapar arrayen*/
    let mut medicines: [Medicine; MAX_SIZE] = array_init(|_i: usize| 
        Medicine { name: String::new(), sizes: [0; 10], balances: [0;10], nr_of_sizes: 0 });

    //todo läs in från fil

    println!("Apoteks verktyg 2.0");

    loop {
        println!("RPSDQ? ");
        let mut input  = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        match input.trim(){
            "q"=> {
                println!("Stänger Apoteks verktyg");
                 //todo skriv till fil
                break;
            },
            "r" => register_medicine(&mut medicines, &mut nr_of_medicines),
            "p" => print!("{}", print_medicines(&medicines, &nr_of_medicines)),
            "s" => {
                let found : [Medicine; MAX_SIZE] = search(&medicines, &nr_of_medicines, &mut nr_of_found_in_search, &mut found_index);
                println!("{}", print_medicines(&found, &nr_of_found_in_search))
            },
            "d" => println!("not implemented"),
            _ => println!("Invalid input, try again."),
        };
    }

}

fn register_medicine(medicines: &mut[Medicine; MAX_SIZE], nr_of_medicines : &mut i32){
    if *nr_of_medicines == MAX_SIZE as i32 {
        println!("Finns inget mer utrymme för fler mediciner.");
        return;
    }

    let mut m : Medicine = Medicine{ 
        name: String::new(),
        sizes: [0;10], 
        balances: [0;10], 
        nr_of_sizes: 0 };

    println!("Registrera läkemedel");

    loop{
        print!("Ange namn: ");

        // för att reseta strängen annars kommer den bli appended mellan varje read_line
        m.name = String::new();

        /*Vi kan inte scanna in ett värde till en temp variabel och sedan sätta m.name till den variabeln
        Detta då temp variabeln endast lever i detta scope och kommer att försvinna när metoden är klar
        Då kommer vår structs name variabel peka mot något som längre inte finns vilket Rust inte tillåter.
        */
        std::io::stdin().read_line(&mut m.name).unwrap();
        m.name =  m.name.trim().to_string();
        if !medicine_already_exists(medicines, *nr_of_medicines, m.name.clone()) {
             break;
        }
       println!("Medicin med namn {} existerar redan, prova igen", m.name)
    }

    for n in 0..10 as usize{
        println!("Ange storlek (0 för att avsluta): ");

        let mut input  = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let size: i32 = input.trim().parse::<i32>().unwrap();  
        if size == 0 {
            break;
        } 
        m.sizes[n] = size;
        m.nr_of_sizes +=1;
    }
    medicines[*nr_of_medicines as usize] = m;
    *nr_of_medicines += 1;
    
}


fn medicine_already_exists(medicines : &mut[Medicine; MAX_SIZE], nr_of_medicines : i32, name : String) -> bool {
    for n in 0..nr_of_medicines as usize {
        if medicines[n].name == name {
            return true;
        }
    }
    return false;
}

fn print_medicines(medicines : &[Medicine; MAX_SIZE], nr_of_medicines : &i32) -> String {
    let mut s : String = String::from("Läkemedel \t Storlekar \t Saldo\n");
    s += "__________________________________________________\n";
    
    //String kan endast plussas ihop eller appendas med str
    // om två str ska plussas ihop eller appendas med varandra måste en av dem vara owned

    for n in 0..*nr_of_medicines as usize {

       // String s += str(owned String.str + &str)
       s += &(medicines[n].name.clone().as_str().to_owned() + "\t\t");

       for j in 0..medicines[n].nr_of_sizes as usize {
        s+= &(medicines[n].sizes[j].to_string().as_str().to_owned() + ", ");
       }

       s += "\t";

       for j in 0..medicines[n].nr_of_sizes as usize {
        s+= &(medicines[n].balances[j].to_string().as_str().to_owned() + ", ");
       }
       s += "\n";
    }
    return s;
}

fn search(medicines : &[Medicine; MAX_SIZE], nr_of_medicines : &i32, nr_of_found_in_search : &mut i32, found_index : &mut i32) 
-> [Medicine; MAX_SIZE] {

    let mut search_word = String::new();
    let mut found : [Medicine; MAX_SIZE] = array_init(|_i: usize| 
        Medicine { name: String::new(), sizes: [0; 10], balances: [0;10], nr_of_sizes: 0 });

    *nr_of_found_in_search = 0;

    println!("Söka läkemedel");
    println!("Ange namn: ");
    std::io::stdin().read_line(&mut search_word).unwrap();

    for n in 0.. *nr_of_medicines as usize {
        if medicines[n].name.contains(&search_word.trim()) {
            found[*nr_of_found_in_search as usize] = medicines[n].clone();
            *nr_of_found_in_search += 1;
            *found_index = n as i32;
        }
    }
    return found;
}
