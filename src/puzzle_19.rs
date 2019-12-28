use crate::intcode::{Machine};
use std::collections::{HashMap};
use std::i128;

const BOX_SIZE: i128 = 100;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct Position {
    x: i128,
    y: i128,
}

struct RowData {
    width: i128,
    start_x: i128,
}

pub fn run() {
    let mut rows = HashMap::new();
    let mut beam_count = 0;

    let mut min_x = 0;
    let mut y = 0;
    let mut box_pos = None;
    while box_pos.is_none() {
        let row = get_data_for_row(y, min_x);
        beam_count = beam_count + row.width;
        min_x = row.start_x;
        println!("Row {} width: {}", y, row.width);

        if row.width < BOX_SIZE + 50 {
            y = y + 50;
            continue;
        }

        rows.insert(y, row);
        match get_box_position(&rows, y) {
            None => (),
            Some(pos) => { box_pos = Some(pos); }
        }

        y = y + 1;
    }

    println!("{} beam positions", beam_count);

    let final_box_pos = box_pos.unwrap();

    println!("Position: {:?}", final_box_pos);
    println!("Answer: {}", final_box_pos.x * 10000 + final_box_pos.y);
}

fn render_area(rows: &HashMap<i128, RowData>, start_y: i128, end_y: i128) {
    let mut y = start_y - 1;
    let _first_row = rows.get(&start_y).unwrap();
    let last_row = rows.get(&end_y).unwrap();
    while y <= end_y {
        print!("{}: ", y);
        for x in 0..(last_row.width + last_row.start_x) {
            let mut machine = Machine::new_from_file("src/inputs/19.txt");
            machine.input_buffer.push_back(x);
            machine.input_buffer.push_back(y);
            machine.run_program();

            match machine.output_buffer.pop_front() {
                None => panic!("Position ({}, {}) did not return any output value", x, y),
                Some(val) => match val {
                    0 => print!("."),
                    1 => print!("#"),
                    v => panic!("Output of {} received", v),
                }
            }
        }

        println!(" ({})", rows.get(&y).unwrap().width);
        y = y + 1;
    }
}

fn get_data_for_row(row_num: i128, start_x: i128) -> RowData {
    let mut min_x = None;

    let mut current_x = start_x;
    loop {
        let mut machine = Machine::new_from_file("src/inputs/19.txt");

        machine.input_buffer.push_back(current_x);
        machine.input_buffer.push_back(row_num);

        machine.run_program();

        match machine.output_buffer.pop_front() {
            None => panic!("Position ({}, {}) did not return any output value", current_x, row_num),
            Some(val) => match val {
                0 => {
                    // stationary
                    if min_x.is_some() {
                        // End of beam cone
                        break;
                    }
                },
                1 => {
                    if min_x.is_none() {
                        min_x = Some(current_x);
                    }
                },
                v => panic!("Output of {} received", v),
            }
        }

        if min_x.is_none() && current_x >= 2000 {
            // Most likely an empty row
            println!("{} {}", current_x, start_x);
            break;
        }

        current_x = current_x + 1;
    }

    let mut width = 0;
    let mut row_begin_x = 0;
    if let Some(val) = min_x {
        row_begin_x = val;
        width = current_x - val;
    }

    RowData {
        start_x: row_begin_x,
        width,
    }
}

fn get_box_position(rows: &HashMap<i128, RowData>, row_num: i128) -> Option<Position> {
    if row_num < BOX_SIZE {
        return None;
    }

    let current_row = match rows.get(&row_num) {
        None => {return None },
        Some(row) => row,
    };

    let first_row_num = row_num - BOX_SIZE + 1;
    let first_row = match rows.get(&first_row_num) {
        None => {return None },
        Some(row) => row,
    };

    if current_row.width < BOX_SIZE {
        return None;
    }

    let x_diff = current_row.start_x - first_row.start_x;
    if first_row.width < x_diff + BOX_SIZE {
        return None;
    }

    return Some(Position {
        x: current_row.start_x,
        y: first_row_num,
    })
}