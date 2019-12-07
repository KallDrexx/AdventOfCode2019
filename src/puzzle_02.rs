use crate::intcode::{Machine};

pub fn run() {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut machine = Machine::new_from_file("src/inputs/02A.txt");
            machine.memory[1] = noun;
            machine.memory[2] = verb;

            machine.run_program();
            if machine.memory[0] == 19690720 {
                println!("Result {}", 100 * noun + verb);
                break;
            }
        }
    }

    println!("Finished");
}