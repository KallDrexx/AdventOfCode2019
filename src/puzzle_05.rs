use crate::intcode::Machine;

pub fn run() {
    let mut machine = Machine::new_from_file("src/inputs/05A.txt");
    machine.input_buffer.push_front(5);
    machine.run_program();

    println!("Output:");
    for output in machine.output_buffer.drain(..) {
        println!("{}", output);
    }
}