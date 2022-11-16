/*
MIN SEKVENS FÖR ATT LYCKAS MED LANDNINGEN
0
0
0
0
0
0
100
100
100
100
0
0
30
*/

fn main() {
    let mut time: i32 = 0;
    let mut height: f64 = 250.0;
	let mut velocity: f64 = -25.0;
	let mut throttle: f64 = 0.0;

    println!("{}", get_initial_message());

    while height > 0.0 { 
        println!("{} \t {} \t\t {} \t\t {}", time, height, velocity, throttle);

        let mut input  = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        throttle = input.trim().parse::<f64>().unwrap();
        
        if throttle < 0.0 || throttle > 100.0 {
			println!("Scan for throttle failed, input must be between 0-100.");
			break;
		}

        height = calc_new_height(height, velocity, throttle);
        velocity = calc_new_velocity(velocity, throttle);
        time += 1;
    }

    if velocity >= -2.0 {
		print!("SUCCESS! Landed at {} m/s", velocity);
	} else {
		print!("FAILED! Crash landed at {} m/s", velocity);
	}
}

fn calc_new_height(current_height : f64, current_velocity : f64, throttle : f64 ) -> f64{
    return current_height + current_velocity + ((throttle*0.1 - 1.5) / 2.0)
}

fn calc_new_velocity(current_velocity : f64, throttle: f64) -> f64 {
    return current_velocity + (throttle*0.1 - 1.5);
}

fn get_initial_message() -> String {
    return String::from("
Lunar decent challenge!
You will pilot a lunar decent the last 250m.
Each turn represent 1-second decent time.
Set the throttle for each turn (0-100)
Time \t Height \t Velocity \t Throttle? \t")
}
