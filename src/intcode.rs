use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;

pub struct Machine {
    pub memory: Vec<i32>,
    pub input_buffer: VecDeque<i32>,
    pub output_buffer: VecDeque<i32>,
    instruction_pointer: usize,
}

#[derive(Debug)]
pub enum MachineState {
    WaitingForInput,
    Halted,
}

impl Machine {
    pub fn new_from_memory(memory: Vec<i32>) -> Self {
        Machine {
            memory,
            input_buffer: VecDeque::new(),
            output_buffer: VecDeque::new(),
            instruction_pointer: 0,
        }
    }

    pub fn new_from_file(filename: &str) -> Self {
        let mut file = File::open(filename).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();

        let mut memory = Vec::new();
        for code in content.trim().split(",") {
            let code_as_int = code.parse::<i32>().unwrap();
            memory.push(code_as_int);
        }

        Machine::new_from_memory(memory)
    }
    
    pub fn run_program(&mut self) -> MachineState {
        loop {
            let instruction = parse_instruction(self.memory[self.instruction_pointer]);
            match &instruction.op_code {
                1 => {
                    // Add
                    let left_param_val = self.memory[self.instruction_pointer as usize + 1];
                    let right_param_val = self.memory[self.instruction_pointer as usize + 2];
                    let result_param_val = self.memory[self.instruction_pointer as usize + 3];

                    let left = read_param_value(&self.memory, left_param_val, &instruction.param1_mode);
                    let right = read_param_value(&self.memory, right_param_val, &instruction.param2_mode);
                    let output = left + right;

                    self.memory[result_param_val as usize] = output;
                    self.instruction_pointer = self.instruction_pointer + 4;
                }

                2 => {
                    // Multiply
                    let left_param_val = self.memory[self.instruction_pointer as usize + 1];
                    let right_param_val = self.memory[self.instruction_pointer as usize + 2];
                    let result_param_val = self.memory[self.instruction_pointer as usize + 3];

                    let left = read_param_value(&self.memory, left_param_val, &instruction.param1_mode);
                    let right = read_param_value(&self.memory, right_param_val, &instruction.param2_mode);
                    let output = left * right;

                    self.memory[result_param_val as usize] = output;
                    self.instruction_pointer = self.instruction_pointer + 4;
                }

                3 => {
                    let input = match self.input_buffer.pop_front() {
                        None => return MachineState::WaitingForInput,
                        Some(x) => x,
                    };

                    //println!("Input: {}", input);

                    let output_address = self.memory[self.instruction_pointer + 1];
                    self.memory[output_address as usize] = input;
                    self.instruction_pointer = self.instruction_pointer + 2;
                }

                4 => {
                    let param_val = self.memory[self.instruction_pointer  + 1];
                    let output = read_param_value(&self.memory, param_val, &instruction.param1_mode);
                    self.output_buffer.push_back(output);
                    self.instruction_pointer = self.instruction_pointer + 2;
                }

                5 => {
                    // Jump if true
                    let param1_val = self.memory[self.instruction_pointer + 1];
                    let param2_val = self.memory[self.instruction_pointer + 2];
                    let check_value = read_param_value(&self.memory, param1_val, &instruction.param1_mode);
                    let jump_address = read_param_value(&self.memory, param2_val, &instruction.param2_mode);

                    self.instruction_pointer = if check_value != 0 {
                        jump_address as usize
                    } else {
                        self.instruction_pointer + 3
                    }
                }

                6 => {
                    // Jump if false
                    let param1_val = self.memory[self.instruction_pointer + 1];
                    let param2_val = self.memory[self.instruction_pointer + 2];
                    let check_value = read_param_value(&self.memory, param1_val, &instruction.param1_mode);
                    let jump_address = read_param_value(&self.memory, param2_val, &instruction.param2_mode);

                    self.instruction_pointer = if check_value == 0 {
                        jump_address as usize
                    } else {
                        self.instruction_pointer + 3
                    }
                }

                7 => {
                    // less than
                    let param1_val = self.memory[self.instruction_pointer + 1];
                    let param2_val = self.memory[self.instruction_pointer + 2];
                    let param3_val = self.memory[self.instruction_pointer + 3];

                    let left = read_param_value(&self.memory, param1_val, &instruction.param1_mode);
                    let right = read_param_value(&self.memory, param2_val, &instruction.param2_mode);
                    let store_pos = param3_val;

                    self.memory[store_pos as usize] = if left < right { 1 } else { 0 };
                    self.instruction_pointer = self.instruction_pointer + 4;
                }

                8 => {
                    // equals
                    let param1_val = self.memory[self.instruction_pointer + 1];
                    let param2_val = self.memory[self.instruction_pointer + 2];
                    let param3_val = self.memory[self.instruction_pointer + 3];

                    let left = read_param_value(&self.memory, param1_val, &instruction.param1_mode);
                    let right = read_param_value(&self.memory, param2_val, &instruction.param2_mode);
                    let store_pos = param3_val;

                    self.memory[store_pos as usize] = if left == right { 1 } else { 0 };
                    self.instruction_pointer = self.instruction_pointer + 4;
                }

                99 => {
                    return MachineState::Halted;
                }

                x => panic!("Unknown opcode {}", x)
            }
        }
    }
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

fn read_param_value(memory: &Vec<i32>, param_value: i32, param_type: &ParameterMode) -> i32 {
    match param_type {
        ParameterMode::Position => memory[param_value as usize],
        ParameterMode::Immediate => param_value,
    }
}