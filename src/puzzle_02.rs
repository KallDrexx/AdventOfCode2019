use std::fs::File;
use std::io::Read;

pub fn run() {
    let original_memory = read_initial_memory();

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut memory = original_memory.clone();
            memory[1] = noun;
            memory[2] = verb;

            let output = run_program(memory);
            if output[0] == 19690720 {
                println!("Result {}", 100 * noun + verb);
                break;
            }
        }
    }

    println!("Finished");
}

fn display_values(values: &Vec<i32>) {
    for x in values {
        print!("{} ", x);
    }

    println!();
}

fn read_initial_memory() -> Vec<i32> {
    let mut file = File::open("src/inputs/02A.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let mut output = Vec::new();
    for code in content.trim().split(",") {
        let code_as_int = code.parse::<i32>().unwrap();
        output.push(code_as_int);
    }

    output
}

fn run_program(mut memory: Vec<i32>) -> Vec<i32> {
    let mut instruction_pointer = 0;
    loop {
        let op_code = memory[instruction_pointer];
        match op_code {
            1 => {
                // Add
                let left_pos = memory[instruction_pointer as usize + 1];
                let right_pos = memory[instruction_pointer as usize + 2];
                let result_pos = memory[instruction_pointer as usize + 3];

                let left = memory[left_pos as usize];
                let right = memory[right_pos as usize];
                let output = left + right;

                //println!("{} + {} ({}, {}) = {} ({})", left, right, left_pos, right_pos, output, right_pos);
                memory[result_pos as usize] = output;
            }

            2 => {
                // Multiply
                let left_pos = memory[instruction_pointer as usize + 1];
                let right_pos = memory[instruction_pointer as usize + 2];
                let result_pos = memory[instruction_pointer as usize + 3];

                let left = memory[left_pos as usize];
                let right = memory[right_pos as usize];
                let output = left * right;

                //println!("{} * {} ({}, {}) = {} ({})", left, right, left_pos, right_pos, output, right_pos);
                memory[result_pos as usize] = output;
            }

            99 => {
                //println!("Finished");
                break;
            }

            _ => panic!("Unknown opcode {}", op_code)
        }

        instruction_pointer = instruction_pointer + 4;

        //display_values(&memory);
    }

    memory
}