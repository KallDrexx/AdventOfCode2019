use crate::intcode::Machine;

pub fn run () {
    //let input = vec![104,1125899906842624,99];
    //let mut machine = Machine::new_from_memory(input);
    let mut machine = Machine::new_from_file("src/inputs/09A.txt");
    machine.input_buffer.push_front(2);
    let stop_reason = machine.run_program();

    println!("{:?}", stop_reason);
    println!("Output: {:?}", machine.output_buffer);
}