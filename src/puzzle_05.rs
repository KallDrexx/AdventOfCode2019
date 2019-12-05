use std::io;
use std::fs::File;
use std::io::Read;

pub fn run() {
    let memory = read_initial_memory();
    //display_values(&memory);
    run_program(memory);
}

#[derive(Debug, Clone)]
enum ParameterMode { Position, Immediate }

#[derive(Debug)]
struct Instruction {
    pub op_code: i32,
    pub param1_mode: ParameterMode,
    pub param2_mode: ParameterMode,
    pub param3_mode: ParameterMode,
}

fn read_initial_memory() -> Vec<i32> {
    let mut file = File::open("src/inputs/05A.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let mut output = Vec::new();
    for code in content.trim().split(",") {
        let code_as_int = code.parse::<i32>().unwrap();
        output.push(code_as_int);
    }

    output
}

fn read_param_value(memory: &Vec<i32>, param_value: i32, param_type: &ParameterMode) -> i32 {
    match param_type {
        ParameterMode::Position => memory[param_value as usize],
        ParameterMode::Immediate => param_value,
    }
}

fn parse_instruction(value: i32) -> Instruction {
    let op_code = value % 100;
    let param1 = value % 1000 / 100;
    let param2 = value % 10000 / 1000;
    let param3 = value % 100000 / 10000;

    Instruction {
        op_code,
        param1_mode: if param1 == 0 { ParameterMode::Position } else { ParameterMode::Immediate },
        param2_mode: if param2 == 0 { ParameterMode::Position } else { ParameterMode::Immediate },
        param3_mode: if param3 == 0 { ParameterMode::Position } else { ParameterMode::Immediate },
    }
}

fn display_values(values: &Vec<i32>) {
    for x in values {
        print!("{} ", x);
    }

    println!();
}

fn run_program(mut memory: Vec<i32>) -> Vec<i32> {
    let mut instruction_pointer = 0;
    loop {
        let instruction = parse_instruction(memory[instruction_pointer]);
        //println!("{} {:?}", memory[instruction_pointer], instruction);
        match &instruction.op_code {
            1 => {
                // Add
                let left_param_val = memory[instruction_pointer as usize + 1];
                let right_param_val = memory[instruction_pointer as usize + 2];
                let result_param_val = memory[instruction_pointer as usize + 3];

                let left = read_param_value(&memory, left_param_val, &instruction.param1_mode);
                let right = read_param_value(&memory, right_param_val, &instruction.param2_mode);
                let output = left + right;

                memory[result_param_val as usize] = output;
                instruction_pointer = instruction_pointer + 4;
            }

            2 => {
                // Multiply
                let left_param_val = memory[instruction_pointer as usize + 1];
                let right_param_val = memory[instruction_pointer as usize + 2];
                let result_param_val = memory[instruction_pointer as usize + 3];

                let left = read_param_value(&memory, left_param_val, &instruction.param1_mode);
                let right = read_param_value(&memory, right_param_val, &instruction.param2_mode);
                let output = left * right;

                memory[result_param_val as usize] = output;
                instruction_pointer = instruction_pointer + 4;
            }

            3 => {
                println!("Input:");
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();

                let value = input.trim().parse().unwrap();
                let output_address = memory[instruction_pointer + 1];
                memory[output_address as usize] = value;
                instruction_pointer = instruction_pointer + 2;
            }

            4 => {
                let param_val = memory[instruction_pointer  + 1];
                let output = read_param_value(&memory, param_val, &instruction.param1_mode);
                println!("Output {}", output);
                instruction_pointer = instruction_pointer + 2;
            }

            5 => {
                // Jump if true
                let param1_val = memory[instruction_pointer + 1];
                let param2_val = memory[instruction_pointer + 2];
                let check_value = read_param_value(&memory, param1_val, &instruction.param1_mode);
                let jump_address = read_param_value(&memory, param2_val, &instruction.param2_mode);

                instruction_pointer = if check_value != 0 {
                    jump_address as usize
                } else {
                    instruction_pointer + 3
                }
            }

            6 => {
                // Jump if false
                let param1_val = memory[instruction_pointer + 1];
                let param2_val = memory[instruction_pointer + 2];
                let check_value = read_param_value(&memory, param1_val, &instruction.param1_mode);
                let jump_address = read_param_value(&memory, param2_val, &instruction.param2_mode);

                instruction_pointer = if check_value == 0 {
                    jump_address as usize
                } else {
                    instruction_pointer + 3
                }
            }

            7 => {
                // less than
                let param1_val = memory[instruction_pointer + 1];
                let param2_val = memory[instruction_pointer + 2];
                let param3_val = memory[instruction_pointer + 3];

                let left = read_param_value(&memory, param1_val, &instruction.param1_mode);
                let right = read_param_value(&memory, param2_val, &instruction.param2_mode);
                let store_pos = param3_val;

                memory[store_pos as usize] = if left < right { 1 } else { 0 };
                instruction_pointer = instruction_pointer + 4;
            }

            8 => {
                // equals
                let param1_val = memory[instruction_pointer + 1];
                let param2_val = memory[instruction_pointer + 2];
                let param3_val = memory[instruction_pointer + 3];

                let left = read_param_value(&memory, param1_val, &instruction.param1_mode);
                let right = read_param_value(&memory, param2_val, &instruction.param2_mode);
                let store_pos = param3_val;

                memory[store_pos as usize] = if left == right { 1 } else { 0 };
                instruction_pointer = instruction_pointer + 4;
            }

            99 => {
                break;
            }

            x => panic!("Unknown opcode {}", x)
        }

        //display_values(&memory);
    }

    memory
}