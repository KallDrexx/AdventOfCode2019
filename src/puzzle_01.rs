use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn run() {
    let file = File::open("src/inputs/01A.txt").unwrap();
    let reader = BufReader::new(file);
    let mut total_fuel = 0;
    for line in reader.lines() {
        let mass = line.unwrap().parse::<i32>().unwrap();
        let mut fuel = calculate_fuel(mass);
        total_fuel = total_fuel + fuel;

        loop {
            fuel = calculate_fuel(fuel);
            if fuel <= 0 {
                break;
            }

            total_fuel = total_fuel + fuel;
        }
    }

    println!("Fuel for modules: {}", total_fuel);
}

fn calculate_fuel(mass: i32) -> i32 {
    mass / 3 - 2
}