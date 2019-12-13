use crate::intcode::{Machine, MachineState};
use console::{Term};
use std::io::Write;
use std::collections::{HashSet};
use std::{thread, time};

pub fn run() {
    let mut machine = Machine::new_from_file("src/inputs/13A.txt");
    machine.memory[0] = 2;
    let mut term = Term::stdout();
    term.clear_screen().unwrap();

    let mut last_y_coord: usize = 0;
    let mut score = 0;
    let mut blocks = HashSet::new();
    let mut paddle_position = Position {from_left: 0, from_top: 0 };
    let mut ball_position = Position {from_left: 0, from_top: 0 };
    let mut ball_direction = Direction::Right;

    render_score(&mut term, score);

    loop {
        let state = machine.run_program();
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

                let position = Position {
                    from_left: values[0],
                    from_top: values[1],
                };

                let tile_type = match values[2] {
                    0 => Tile::Empty,
                    1 => Tile::Wall,
                    2 => Tile::Block,
                    3 => Tile::Paddle,
                    4 => Tile::Ball,
                    x => panic!("Unknown tile type {}", x),
                };

                render_tile(&mut term, &tile_type, &position);

                if tile_type == Tile::Block {
                    blocks.insert(position);
                } else if tile_type == Tile::Empty {
                    blocks.remove(&position);
                } else if tile_type == Tile::Paddle {
                    paddle_position = position
                } else if tile_type == Tile::Ball {
                    ball_direction = if position.from_left < ball_position.from_left {
                        Direction::Left
                    } else {
                        Direction::Right
                    };

                    ball_position = position;
                }
            }
        }

        let distance = paddle_position.from_top - ball_position.from_top - 1;
        let target = match ball_direction {
            Direction::Left => ball_position.from_left - distance,
            Direction::Right => ball_position.from_left + distance,
        };

        let next_input = if paddle_position.from_left < target {
            1 // go right
        } else if paddle_position.from_left > target {
            -1 // go left
        } else {
            0
        };

        //term.move_cursor_to(0, last_y_coord + 4).unwrap();
        //println!("Blocks: {}", blocks.len());
        //println!("Ball Position: {:?}", ball_position);
        //println!("Ball Direction: {:?}", ball_direction);
        //println!("Paddle Position: {:?}", paddle_position);
        //println!("Target: {:?} ({})", target, distance);

        machine.input_buffer.push_front(next_input);
        thread::sleep(time::Duration::from_millis(5));

        if state == MachineState::Halted {
            break;
        }
    }

    term.move_cursor_to(0, last_y_coord + 4).unwrap();
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

#[derive(Hash, Eq, PartialEq, Debug)]
struct Position {
    from_left: i128,
    from_top: i128,
}

#[derive(Eq, PartialEq, Debug)]
enum Direction { Left, Right }

fn render_score(term: &mut Term, score: i128) {
    term.move_cursor_to(0, 1).unwrap();
    term.write(format!("Score: {}", score).as_bytes()).unwrap();
}

fn render_tile(term: &mut Term, tile_type: &Tile, position: &Position) {
    term.move_cursor_to(position.from_left as usize, (position.from_top + 2) as usize).unwrap();

    match tile_type {
        Tile::Empty => term.write(" ".as_bytes()).unwrap(),
        Tile::Wall => term.write("█".as_bytes()).unwrap(),
        Tile::Block => term.write("X".as_bytes()).unwrap(),
        Tile::Paddle => term.write("_".as_bytes()).unwrap(),
        Tile::Ball => term.write("O".as_bytes()).unwrap(),
    };
}