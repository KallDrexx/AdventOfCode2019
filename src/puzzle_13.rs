use crate::intcode::{Machine, MachineState};
use console::{Term, Key};
use std::io::Write;

pub fn run() {
    let mut machine = Machine::new_from_file("src/inputs/13A.txt");
    machine.memory[0] = 2;
    let mut term = Term::stdout();
    term.clear_screen().unwrap();

    let mut last_y_coord: usize = 0;
    let mut score = 0;
    render_score(&mut term, score);

    loop {
        match machine.run_program() {
            MachineState::Halted => break,
            MachineState::WaitingForInput => {
                while machine.output_buffer.len() >= 3 {
                    if machine.output_buffer.len() < 3 {
                        break;
                    }

                    let values = machine.output_buffer.drain(0..3).collect::<Vec<i128>>();
                    if values[0] == -1 && values[1] == 0 {
                        // score
                        score = values[2];
                        render_score(&mut term, score);
                    } else {
                        if values[1] as usize > last_y_coord {
                            last_y_coord = values[1] as usize;
                        }

                        render_tile(&mut term, values[2], values[0], values[1]);
                    }
                }

                term.move_cursor_to(0, last_y_coord + 4).unwrap();
                let next_input = match term.read_key().unwrap() {
                    Key::ArrowLeft => -1,
                    Key::ArrowRight => 1,
                    _ => 0,
                };

                machine.input_buffer.push_front(next_input);
            }
        }
    }

    term.move_cursor_to(0, last_y_coord + 3).unwrap();
    term.write_line("Program halted").unwrap();
}

#[derive(Eq, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

struct Position {
    from_left: i128,
    from_top: i128,
}

fn render_score(term: &mut Term, score: i128) {
    term.move_cursor_to(0, 1).unwrap();
    term.write(format!("Score: {}", score).as_bytes()).unwrap();
}

fn render_tile(term: &mut Term, tile_code: i128, x: i128, y: i128) {
    term.move_cursor_to(x as usize, (y + 2) as usize).unwrap();

    let tile_type = match tile_code {
        0 => Tile::Empty,
        1 => Tile::Wall,
        2 => Tile::Block,
        3 => Tile::Paddle,
        4 => Tile::Ball,
        x => panic!("Unknown tile type {}", x),
    };

    match tile_type {
        Tile::Empty => term.write(" ".as_bytes()).unwrap(),
        Tile::Wall => term.write("█".as_bytes()).unwrap(),
        Tile::Block => term.write("X".as_bytes()).unwrap(),
        Tile::Paddle => term.write("_".as_bytes()).unwrap(),
        Tile::Ball => term.write("O".as_bytes()).unwrap(),
    };
}