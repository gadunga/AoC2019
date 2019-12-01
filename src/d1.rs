/*Specifically, to find the fuel required for a module, 
take its mass, divide by three, round down, and subtract 2.*/

pub fn solve(data: &str) {
    let mut fuel_req = 0.0;
    data.lines().for_each(|s| {
        let mass = s.parse::<f32>().unwrap();
        fuel_req += calculate_fuel_requirement(mass);
    });

    println!("Part 1: {}", fuel_req);

    data.lines().for_each(|s| {
        let mass = s.parse::<f32>().unwrap();
        let temp_fuel_req = calculate_fuel_requirement(mass);

        let mut current_fuel = temp_fuel_req;

        loop {
            let fuel_fuel = calculate_fuel_requirement(current_fuel);
            if (fuel_fuel > 0.0) {
                fuel_req += fuel_fuel;
                current_fuel = fuel_fuel;
            } else {
                break;
            }
        }
    });

    println!("Part 2: {}", fuel_req);
}

fn calculate_fuel_requirement(mass: f32) -> f32 {
    (mass / 3.0).floor() - 2.0
}