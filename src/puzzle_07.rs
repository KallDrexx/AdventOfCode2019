use std::fs::File;
use std::io::{Read};
use crate::intcode::{Machine, MachineState};
use std::collections::VecDeque;

pub fn run() {
    let program = read_initial_memory();
    let mut permutations = get_permutations(&vec![5, 6, 7,8, 9], 0);
    permutations.sort();
    permutations.dedup();

    let mut max_signal = 0;
    for settings in permutations {
        let signal = run_amps(program.clone(), settings);
        if signal > max_signal {
            max_signal = signal;
        }
    }

    println!("Final signal: {}", max_signal);
}

fn run_amps(program: Vec<i32>, phase_settings: Vec<i32>) -> i32 {
    let signal;
    let mut amp_index = 0;

    let mut amps: Vec<Machine> = (0..phase_settings.len())
        .map(|x| {
            let mut machine = Machine::new_from_memory(program.clone());
            machine.input_buffer.push_front(phase_settings[x]);
            machine
        })
        .collect();

    {
        let first_amp = amps.get_mut(0).unwrap();
        first_amp.input_buffer.push_back(0);
    }

    let mut new_outputs = VecDeque::new();
    loop {
        let current_amp_index = amp_index % amps.len();
        let next_amp_index = (amp_index + 1) % amps.len();

        let current_amp = amps.get_mut(current_amp_index).unwrap();
        current_amp.input_buffer.append(&mut new_outputs);

        match current_amp.run_program() {
            MachineState::WaitingForInput => {
                //println!("Amp {} waiting for input", current_amp_index);
                new_outputs = current_amp.output_buffer.drain(..).collect();
                //println!("Outputs: {:?}", new_outputs);
            },

            MachineState::Halted => {
                //println!("Amp {} halted!", current_amp_index);

                new_outputs = current_amp.output_buffer.drain(..).collect();
                if current_amp_index == amps.len() - 1 {
                    // Last amp
                    signal = new_outputs[0];
                    break;
                }
            }
        };
        amp_index = next_amp_index;
    }

    signal
}

fn read_initial_memory() -> Vec<i32> {
    let mut file = File::open("src/inputs/07A.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let mut output = Vec::new();
    for code in content.trim().split(",") {
        let code_as_int = code.parse::<i32>().unwrap();
        output.push(code_as_int);
    }

    output
}

fn get_permutations(initial: &Vec<i32>, item_index: usize) -> Vec<Vec<i32>> {
    let mut results = Vec::new();

    if item_index < initial.len() {
        for swap_index in 0..initial.len() {
            if swap_index == item_index {
                continue;
            }

            let mut swapped = initial.clone();
            swapped.swap(swap_index, item_index);

            let mut recursive_results = get_permutations(&swapped, item_index + 1);

            results.push(swapped);
            results.append(&mut recursive_results);
        }
    }

    results
}