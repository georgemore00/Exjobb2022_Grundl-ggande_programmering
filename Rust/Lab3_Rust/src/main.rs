#[derive(Copy, Clone)]
struct Medicine<'a> {
    name: &'a str,
    sizes: [i32; 10],
    balances: [i32; 10],
    nrOfSizes: i32,
}

fn main() {
    const MAX_SIZE: usize = 10000;
    let mut nrOfMedicines: i32 = 0;
    let mut nrOfFoundInSearch: i32 = 0;
    let mut foundIndex: i32 = -1;

    let mut medicines: [Medicine; MAX_SIZE] = [Medicine {
        name: "",
        sizes: [0; 10],
        balances: [0; 10],
        nrOfSizes: 0,
    }; MAX_SIZE];

    register_medicine(medicines);
}

fn register_medicine(mut medicines: [Medicine; 10000]) {
    medicines[0].name = "Alvedon";
    medicines[0].sizes = 5;
    println!("{}, {}", medicines[0].name, medicines[0].sizes);
}
