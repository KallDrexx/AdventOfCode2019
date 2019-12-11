use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;

pub struct Machine {
    pub memory: Vec<i128>,
    pub input_buffer: VecDeque<i128>,
    pub output_buffer: VecDeque<i128>,
    instruction_pointer: usize,
    relative_base: i128,
}

#[derive(Debug)]
pub enum MachineState {
    WaitingForInput,
    Halted,
}

impl Machine {
    pub fn new_from_memory(memory: Vec<i128>) -> Self {
        Machine {
            memory,
            input_buffer: VecDeque::new(),
            output_buffer: VecDeque::new(),
            instruction_pointer: 0,
            relative_base: 0,
        }
    }

    pub fn new_from_file(filename: &str) -> Self {
        let mut file = File::open(filename).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();

        let mut memory = Vec::new();
        for code in content.trim().split(",") {
            let code_as_int = code.parse::<i128>().unwrap();
            memory.push(code_as_int);
        }

        Machine::new_from_memory(memory)
    }
    
    pub fn run_program(&mut self) -> MachineState {
        loop {
            let instruction = parse_instruction(self.memory[self.instruction_pointer]);
            //println!("{:?}", instruction);
            match &instruction.op_code {
                1 => {
                    // Add
                    let left_param_val = self.memory[self.instruction_pointer as usize + 1];
                    let right_param_val = self.memory[self.instruction_pointer as usize + 2];
                    let mut result_param_val = self.memory[self.instruction_pointer as usize + 3];

                    let left = self.read_param_value(left_param_val, &instruction.param1_mode);
                    let right = self.read_param_value(right_param_val, &instruction.param2_mode);
                    let output = left + right;

                    if instruction.param3_mode == ParameterMode::Relative {
                        result_param_val = self.relative_base + result_param_val;
                    }

                    self.resize_if_needed(result_param_val);
                    self.memory[result_param_val as usize] = output;
                    self.instruction_pointer = self.instruction_pointer + 4;
                }

                2 => {
                    // Multiply
                    let left_param_val = self.memory[self.instruction_pointer as usize + 1];
                    let right_param_val = self.memory[self.instruction_pointer as usize + 2];
                    let mut result_param_val = self.memory[self.instruction_pointer as usize + 3];

                    let left = self.read_param_value(left_param_val, &instruction.param1_mode);
                    let right = self.read_param_value(right_param_val, &instruction.param2_mode);
                    let output = left * right;

                    if instruction.param3_mode == ParameterMode::Relative {
                        result_param_val = self.relative_base + result_param_val;
                    }

                    self.resize_if_needed(result_param_val);
                    self.memory[result_param_val as usize] = output;
                    self.instruction_pointer = self.instruction_pointer + 4;
                }

                3 => {
                    let input = match self.input_buffer.pop_front() {
                        None => return MachineState::WaitingForInput,
                        Some(x) => x,
                    };

                    //println!("Input: {}", input);
                    let mut output_address = self.memory[self.instruction_pointer + 1];
                    //println!("Original Address {}, base: {}", output_address, self.relative_base);

                    if instruction.param1_mode == ParameterMode::Relative {
                        output_address = self.relative_base + output_address;
                    }

                    self.resize_if_needed(output_address);
                    self.memory[output_address as usize] = input;
                    self.instruction_pointer = self.instruction_pointer + 2;
                }

                4 => {
                    let param_val = self.memory[self.instruction_pointer  + 1];
                    let output = self.read_param_value(param_val, &instruction.param1_mode);
                    self.output_buffer.push_back(output);
                    self.instruction_pointer = self.instruction_pointer + 2;
                }

                5 => {
                    // Jump if true
                    let param1_val = self.memory[self.instruction_pointer + 1];
                    let param2_val = self.memory[self.instruction_pointer + 2];
                    let check_value = self.read_param_value(param1_val, &instruction.param1_mode);
                    let jump_address = self.read_param_value(param2_val, &instruction.param2_mode);

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
                    let check_value = self.read_param_value(param1_val, &instruction.param1_mode);
                    let jump_address = self.read_param_value(param2_val, &instruction.param2_mode);

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

                    let left = self.read_param_value(param1_val, &instruction.param1_mode);
                    let right = self.read_param_value(param2_val, &instruction.param2_mode);
                    let mut store_pos = param3_val;

                    if instruction.param3_mode == ParameterMode::Relative {
                        store_pos = self.relative_base + store_pos;
                    }

                    self.resize_if_needed(store_pos);
                    self.memory[store_pos as usize] = if left < right { 1 } else { 0 };
                    self.instruction_pointer = self.instruction_pointer + 4;
                }

                8 => {
                    // equals
                    let param1_val = self.memory[self.instruction_pointer + 1];
                    let param2_val = self.memory[self.instruction_pointer + 2];
                    let param3_val = self.memory[self.instruction_pointer + 3];

                    let left = self.read_param_value(param1_val, &instruction.param1_mode);
                    let right = self.read_param_value(param2_val, &instruction.param2_mode);
                    let mut store_pos = param3_val;

                    if instruction.param3_mode == ParameterMode::Relative {
                        store_pos = self.relative_base + store_pos;
                    }

                    self.resize_if_needed(store_pos);
                    self.memory[store_pos as usize] = if left == right { 1 } else { 0 };
                    self.instruction_pointer = self.instruction_pointer + 4;
                }

                9 => {
                    // adjust relative base
                    let param_val = self.memory[self.instruction_pointer + 1];
                    let change = self.read_param_value(param_val, &instruction.param1_mode);

                    //println!("Relative base change by {} + {} = {}", self.relative_base, change, self.relative_base + change);

                    self.relative_base = self.relative_base + change;
                    self.instruction_pointer = self.instruction_pointer + 2;
                }

                99 => {
                    return MachineState::Halted;
                }

                x => panic!("Unknown opcode {}", x)
            }
        }
    }

    fn read_param_value(&mut self, param_value: i128, param_type: &ParameterMode) -> i128 {
        match param_type {
            ParameterMode::Position => self.read_memory_loc(param_value),
            ParameterMode::Immediate => param_value,
            ParameterMode::Relative => self.read_memory_loc(param_value + self.relative_base),
        }
    }

    fn read_memory_loc(&mut self, location: i128) -> i128 {
        self.resize_if_needed(location);
        self.memory[location as usize]
    }

    fn resize_if_needed(&mut self, location: i128) {
        if location < 0 {
            panic!("Negaive memory location provided {}", location);
        }

        if (location + 1) as usize > self.memory.len() {
            self.memory.resize((location + 1) as usize, 0)
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum ParameterMode { Position, Immediate, Relative }

#[derive(Debug)]
struct Instruction {
    pub op_code: i128,
    pub param1_mode: ParameterMode,
    pub param2_mode: ParameterMode,
    pub param3_mode: ParameterMode,
}

fn parse_instruction(value: i128) -> Instruction {
    let op_code = value % 100;
    let param1 = value % 1000 / 100;
    let param2 = value % 10000 / 1000;
    let param3 = value % 100000 / 10000;

    Instruction {
        op_code,
        param1_mode: get_param_mode(param1),
        param2_mode: get_param_mode(param2),
        param3_mode: get_param_mode(param3),
    }
}

fn get_param_mode(value: i128) -> ParameterMode {
    match value {
        0 => ParameterMode::Position,
        1 => ParameterMode::Immediate,
        2 => ParameterMode::Relative,
        x => panic!("Parameter mode {} is not valid", x),
    }
}