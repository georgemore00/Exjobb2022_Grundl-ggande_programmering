use array_init::array_init;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::fs::OpenOptions;


#[derive(Debug, Clone)]
struct Medicine {
    name: String,
    sizes: [i32; 10],
    balances: [i32; 10],
    nr_of_sizes: i32,
}

/*Todo - ändra till 10000
Måste vara global variabel annars måste man skicka variabeln som input-parameter i flera metoder
eller hårkoda värdet "10000" i metoderna

usize då värdet måste vara unsigned (positivt) om man vill sätta array storlek med MAX_SIZE
*/
const MAX_SIZE: usize = 10;

fn main() {
    /*Array_init pga att det inte går att initiera en array av structar om structen har en datatyp som inte implementerar Copy Traiten
    name: String implementerar inte Copy, vilket är metoden som Rust kommer att använda
    då den itererar över varje Index och kallar på Copy när vi skapar arrayen*/
    let mut medicines: [Medicine; MAX_SIZE] = array_init(|_i: usize| Medicine {
        name: String::new(),
        sizes: [0; 10],
        balances: [0; 10],
        nr_of_sizes: 0,
    });
    let mut nr_of_medicines: i32 = 0;
    let mut nr_of_found_in_search: i32 = 0;
    let mut found_index: i32 = -1;
    let mut file_path: String = String::new();

    println!("Apoteks verktyg 2.0");
    println!("\nVänligen ange filnamn: ");
    std::io::stdin().read_line(&mut file_path).unwrap();
    read_from_file(&mut file_path, &mut medicines, &mut nr_of_medicines);

    loop {
        println!("\nRPSDQ?");
        let mut input: String = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "r" => register_medicine(&mut medicines, &mut nr_of_medicines),
            "p" => print!("\n{}", print_medicines(&medicines, &nr_of_medicines)),
            "s" => {
                let found: [Medicine; MAX_SIZE] = search(
                    &medicines,
                    &nr_of_medicines,
                    &mut nr_of_found_in_search,
                    &mut found_index,
                );
                println!("{}", print_medicines(&found, &nr_of_found_in_search))
            }
            "d" => delete_medicine(&mut medicines, &mut nr_of_medicines, &mut nr_of_found_in_search, &mut found_index),
            "q" => {
                write_to_file(&mut file_path, &mut medicines, &mut nr_of_medicines);
                println!("Stänger Apoteks verktyg 2.0");
                break;
            }
            _ => println!("Invalid inmatning, försök igen."),
        };
    }
}

fn register_medicine(medicines: &mut [Medicine; MAX_SIZE], nr_of_medicines: &mut i32) {
    if *nr_of_medicines == MAX_SIZE as i32 {
        println!("Finns inget mer utrymme för fler mediciner.");
        return;
    }

    let mut m: Medicine = Medicine {
        name: String::new(),
        sizes: [0; 10],
        balances: [0; 10],
        nr_of_sizes: 0,
    };

    println!("\nRegistrera läkemedel");

    loop {
        println!("Ange namn: ");

        //För att reseta strängen annars kommer den bli appended mellan varje read_line
        m.name = String::new();

        /* Vi kan inte scanna in ett värde till en temp variabel och sedan sätta m.name till den variabeln
        Detta då temp variabeln endast lever i detta scope och kommer att försvinna när metoden är klar
        Då kommer vår structs name variabel peka mot något som längre inte finns vilket Rust inte tillåter.
        */
        std::io::stdin().read_line(&mut m.name).unwrap();
        m.name = m.name.trim().to_string();
        if !medicine_already_exists(medicines, *nr_of_medicines, m.name.clone()) {
            break;
        }
        println!("Medicin med namn {} existerar redan, prova igen", m.name)
    }

    for n in 0..10 as usize {
        println!("Ange storlek (0 för att avsluta): ");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let size: i32 = input.trim().parse::<i32>().unwrap();
        if size == 0 {
            break;
        }
        m.sizes[n] = size;
        m.nr_of_sizes += 1;
    }
    medicines[*nr_of_medicines as usize] = m;
    *nr_of_medicines += 1;
}

fn medicine_already_exists(medicines: &mut [Medicine; MAX_SIZE], nr_of_medicines: i32, name: String) -> bool {
    for n in 0..nr_of_medicines as usize {
        if medicines[n].name == name {
            return true;
        }
    }
    return false;
}

fn delete_medicine(medicines: &mut [Medicine; MAX_SIZE], nr_of_medicines: &mut i32, nr_of_found_in_search: &mut i32, found_index: &mut i32) {
    let mut answer = String::new();
    *nr_of_found_in_search = 0;
    while *nr_of_found_in_search != 1 {
        let found: [Medicine; MAX_SIZE] = search(&medicines, &nr_of_medicines, nr_of_found_in_search, found_index);
        println!("{}", print_medicines(&found, nr_of_found_in_search));
        if *nr_of_found_in_search == 1 {
            println!("Vill du avregistrera {} (j/n)?", medicines[*found_index as usize].name);
            std::io::stdin().read_line(&mut answer).unwrap();
            if answer.trim() == "j" {
                println!("{} avregistreras", medicines[*found_index as usize].name);
                // Vet inte hur jag ska ta bort elementet i arrayen
                /*for i in *found_index..*nr_of_medicines {
                    medicines[*found_index as usize] = medicines[*found_index+1];
                }
                *nr_of_medicines -= 1;*/
            }
            else if answer.trim() == "n" {
                break;
            }
            else {
                println!("Var vanlig och skriv in antingen 'j' eller 'n' \n");
            }
        }
        else {
            if *nr_of_medicines == 0 {
                println!("Det finns för nuvarande inga läkemedel i apoteket.");
                break;
            }
            println!("Du fick inte endast ett alternativ. Vänligen gör en ny sökning.");
        }
    }
}

fn print_medicines(medicines: &[Medicine; MAX_SIZE], nr_of_medicines: &i32) -> String {
    let mut s: String = String::from("Läkemedel \t Storlekar \t Saldo\n");
    s += "__________________________________________________\n";

    //String kan endast plussas ihop eller appendas med str
    //om två str ska plussas ihop eller appendas med varandra måste en av dem vara owned
    for n in 0..*nr_of_medicines as usize {
        // String s += str(owned String.str + &str)
        s += &(medicines[n].name.clone().as_str().to_owned() + "\t\t");

        for j in 0..medicines[n].nr_of_sizes as usize {
            s += &(medicines[n].sizes[j].to_string().as_str().to_owned() + ", ");
        }

        s += "\t";

        for j in 0..medicines[n].nr_of_sizes as usize {
            s += &(medicines[n].balances[j].to_string().as_str().to_owned() + ", ");
        }
        s += "\n";
    }
    return s;
}

fn search(medicines: &[Medicine; MAX_SIZE], nr_of_medicines: &i32, nr_of_found_in_search: &mut i32, found_index: &mut i32,) -> [Medicine; MAX_SIZE] {
    let mut search_word = String::new();
    let mut found: [Medicine; MAX_SIZE] = array_init(|_i: usize| Medicine {
        name: String::new(),
        sizes: [0; 10],
        balances: [0; 10],
        nr_of_sizes: 0,
    });

    *nr_of_found_in_search = 0;

    println!("\nSökning av läkemedel...");
    println!("Ange namn: ");
    std::io::stdin().read_line(&mut search_word).unwrap();

    for n in 0..*nr_of_medicines as usize {
        if medicines[n].name.contains(&search_word.trim()) {
            found[*nr_of_found_in_search as usize] = medicines[n].clone();
            *nr_of_found_in_search += 1;
            *found_index = n as i32;
        }
    }
    return found;
}

fn write_to_file(file_path: &mut String, medicines: &mut [Medicine; MAX_SIZE], nr_of_medicines: &mut i32) {
    let mut file = File::create(&file_path).expect("\nKunde inte skapa filen");
    println!("\nSparar filen: {}", file_path);

    //Lägger in nr_of_medicines högst upp i filen
    file.write_all(&(nr_of_medicines.clone().to_string().to_owned() + "\n").as_bytes()).expect("Write nr_of_medicines misslyckades");

    for i in 0..*nr_of_medicines as usize{
        //Lägger in name i filen
        file.write_all((medicines[i].name.clone().to_string().to_owned() + " ").as_bytes()).expect("Write name misslyckades");

        //Lägger in sizes i filen
        for j in 0..MAX_SIZE {
            file.write_all((medicines[i].sizes[j].to_string().to_owned() + ",").as_bytes()).expect("Write sizes misslyckades");
        }
        file.write_all((" ").as_bytes()).expect("Write misslyckades");

        //Lägger in balances i filen
        for j in 0..MAX_SIZE {
            file.write_all((medicines[i].balances[j].to_string().to_owned() + ",").as_bytes()).expect("Write balances misslyckades");
        }
        file.write_all((" ").as_bytes()).expect("Write misslyckades");

        //Lägger in nr_of_sizes i filen
        file.write_all((medicines[i].nr_of_sizes.clone().to_string().to_owned()).as_bytes()).expect("Write nr_of_sizes misslyckades");
        file.write_all(("\n").as_bytes()).expect("Write misslyckades");
    }
}

//Kan bara hämta och öppna en fil, skapar om det inte finns någon. Saknar hämtning av data för tillfället
fn read_from_file(file_path: &mut String, medicines: &mut [Medicine; MAX_SIZE], nr_of_medicines: &mut i32) {
    let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path);
    /*if !file.is_err(){
        println!("\nÖppnar filen: {}", file_path);
    }
    else {
        println!("\nKunde inte hitta filen: {}Filen kommer skapas vid programslut.", file_path);
    }*/
}