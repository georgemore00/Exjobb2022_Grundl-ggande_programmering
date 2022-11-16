fn main() {
    
    let mut values : [i32; 10] = [0; 10];
    let mut size : i32 = 0;

    println!("Measurement tool 2.0");

    loop {
        println!("VECRQ? ");
        let mut option: String = String::new();
        std::io::stdin().read_line(&mut option).unwrap();

        match option.trim() {
            "v" => println!("{}", view(values, size)),
            "e" => enter_measurements(&mut values, &mut size),
            "c" => compute(values, size),
            "r" => {
                values = [0; 10];
                size = 0;
                println!("Reseted values.")
            },
            "q" => { 
                println!("Exit Measurement tool");
                break;
            },
            _ => println!("Invalid input, try again.")
        };
    }

}

fn view(arr : [i32; 10], size : i32) -> String{
    if size == 0 {
        return String::from("No measurements");
    }

    let mut s : String = String::from("[");
    for n in 0..size as usize{
        s += &(arr[n].to_string() + " ");
    }
    s += "]";

    return s;
}

fn enter_measurements(arr : &mut [i32; 10], size: &mut i32) {
    let mut input: String = String::new();

    for n in *size..10{
        println!("Enter measurment #{} (or q to quit): ", *size+1);
        std::io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "q"{
            return;
        }
        let measurment : i32 = input.trim().parse::<i32>().unwrap();
        arr[n as usize] = measurment;
        *size += 1;
        input = "".to_string();
    }
}

fn compute(arr : [i32; 10], size : i32) {
    if size == 0 {
        println!("Measurments empty.");
        return;
    }

    println!("Min value: {}", find_min(arr, size));
    println!("Max value: {}", find_max(arr, size));
    println!("Avg value: {:0.2}", find_average(arr, size));
    println!("{}", view(calc_normalized_values(arr, size), size))
}

fn find_min(arr : [i32; 10], size : i32) -> i32 {
    let mut min : i32 = arr[0];

    for n in 0..size-1{
        if arr[(n +1 ) as usize] < min{
            min = arr[(n+1) as usize];
        }
    }
    return min;
}

fn find_max(arr : [i32; 10], size : i32) -> i32 {
    let mut max : i32 = arr[0];

    for n in 0..size-1 {
        if arr[(n +1 ) as usize] > max{
            max = arr[(n +1 ) as usize];
        }
    }
    return max;
}

fn find_average(arr : [i32; 10], size : i32) -> f32 {
    let mut total : f32 = 0.0;
    for n in 0..size{
        total += arr[n as usize] as f32
    }
    return total/(size as f32);
}

fn calc_normalized_values(arr : [i32; 10], size : i32) -> [i32; 10] {
    let average : f32 = find_average(arr, size);
    let mut normalized_values : [i32; 10] = [0; 10];

    for n in 0..size as usize {
        let val : i32 = arr[n] - (average.round() as i32);
        normalized_values[n] = val;
    }
    return normalized_values;
}

