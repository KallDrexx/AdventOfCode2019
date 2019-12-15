use crate::intcode::Machine;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
enum Direction { North, South, East, West }
enum CellContents { Empty, Wall, OxygenTank }

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Position {
    x: i32,
    y: i32,
}

pub fn run() {
    let mut machine = Machine::new_from_file("src/inputs/15A.txt");
    let mut found_positions = HashMap::new();
    let mut current_position = Position {x: 0, y: 0};
    let mut history = Vec::new();
    let mut tank_position = None;

    machine.run_program();
    found_positions.insert(current_position.clone(), CellContents::Empty);
    loop {
        if let Some((next_direction, next_position)) = next_unexplored(&current_position, &found_positions) {

            let input = direction_to_input(&next_direction);
            machine.input_buffer.push_back(input);
            machine.run_program();

            if machine.output_buffer.len() != 1 {
                panic!("Expected 1 outputs, found {:?}", machine.output_buffer);
            }

            match machine.output_buffer.pop_front().unwrap() {
                0 => {
                    // wall, didn't move
                    found_positions.insert(next_position.clone(), CellContents::Wall);
                }

                1 => {
                    // Moved, no tank
                    found_positions.insert(next_position.clone(), CellContents::Empty);
                    history.push((next_direction, current_position.clone()));
                    current_position = next_position;
                }

                2 => {
                    // Moved, with tank
                    found_positions.insert(next_position.clone(), CellContents::OxygenTank);
                    history.push((next_direction, current_position.clone()));
                    current_position = next_position.clone();
                    tank_position = Some(next_position);
                }

                x => panic!("Unknown output code {}", x),
            }
        } else {
            // Dead end, so backtrack
            if history.len() == 0 {
                // Whole map has been mapped
                break;
            }

            let (direction, position) = history.pop().unwrap();
            let opposite_direction = opposite_direction(&direction);

            current_position = position;
            let input = direction_to_input(&opposite_direction);
            machine.input_buffer.push_back(input);
            machine.run_program();

            if machine.output_buffer.len() != 1 {
                panic!("Expected 1 outputs, found {:?}", machine.output_buffer);
            }

            let output = machine.output_buffer.pop_front().unwrap();
            if output != 1 {
                panic!("Backtrack expected output of 1, instead had {}", output);
            }
        }
    }

    let tank_at = tank_position.unwrap();
    println!("Tank at {:?}", tank_at);

    let steps_to_tank = steps_to_tank(&found_positions);
    println!("Steps: {}", steps_to_tank);

    let steps_to_fill = steps_to_fill(&tank_at, &found_positions);
    println!("Filled in: {}", steps_to_fill);
}

fn next_unexplored(current_position: &Position, found_positions: &HashMap<Position, CellContents>) -> Option<(Direction, Position)> {
    let north = Position {x: current_position.x, y: current_position.y + 1};
    let south = Position {x: current_position.x, y: current_position.y - 1};
    let east = Position {x: current_position.x + 1, y: current_position.y};
    let west = Position {x: current_position.x - 1, y: current_position.y};

    if !found_positions.contains_key(&north) {
        Some((Direction::North, north))
    } else if !found_positions.contains_key(&east) {
        Some((Direction::East, east))
    } else if !found_positions.contains_key(&south) {
        Some((Direction::South, south))
    } else if !found_positions.contains_key(&west) {
        Some((Direction::West, west))
    } else {
        None
    }
}

fn opposite_direction(direction: &Direction) -> Direction {
    match direction {
        Direction::North => Direction::South,
        Direction::South => Direction::North,
        Direction::East => Direction::West,
        Direction::West => Direction::East,
    }
}

fn direction_to_input(direction: &Direction) -> i128 {
    match direction {
        Direction::North => 1,
        Direction::South => 2,
        Direction::West => 3,
        Direction::East => 4,
    }
}

fn steps_to_tank(map: &HashMap<Position, CellContents>) -> i32 {
    let mut active_positions = vec![Position {x: 0, y: 0}];
    let mut found_positions = HashSet::new();
    let mut step_count = 0;
    while active_positions.len() > 0 {
        let mut new_positions = Vec::new();
        for next_position in active_positions.drain(..) {
            found_positions.insert(next_position.clone());

            let north = Position {x: next_position.x, y: next_position.y + 1};
            let south = Position {x: next_position.x, y: next_position.y - 1};
            let east = Position {x: next_position.x + 1, y: next_position.y};
            let west = Position {x: next_position.x - 1, y: next_position.y};

            let positions_to_try = vec![north, south, east, west];
            for try_position in positions_to_try {
                match map.get(&try_position) {
                    None => (),
                    Some(x) => match x {
                        CellContents::Empty => {
                            if !found_positions.contains(&try_position) {
                                new_positions.push(try_position.clone());
                            }
                        },
                        CellContents::Wall => (),
                        CellContents::OxygenTank => return step_count + 1,
                    }
                }
            }
        }

        active_positions = new_positions;
        step_count = step_count + 1;
    }

    unreachable!();
}

fn steps_to_fill(tank_position: &Position, map: &HashMap<Position, CellContents>) -> i32 {
    let mut active_positions = vec![tank_position.clone()];
    let mut found_positions = HashSet::new();
    let mut step_count = 0;
    while active_positions.len() > 0 {
        let mut new_positions = Vec::new();
        for next_position in active_positions.drain(..) {
            found_positions.insert(next_position.clone());

            let north = Position {x: next_position.x, y: next_position.y + 1};
            let south = Position {x: next_position.x, y: next_position.y - 1};
            let east = Position {x: next_position.x + 1, y: next_position.y};
            let west = Position {x: next_position.x - 1, y: next_position.y};

            let positions_to_try = vec![north, south, east, west];
            for try_position in positions_to_try {
                match map.get(&try_position) {
                    None => (),
                    Some(x) => match x {
                        CellContents::Empty => {
                            if !found_positions.contains(&try_position) {
                                new_positions.push(try_position.clone());
                            }
                        },
                        CellContents::Wall => (),
                        CellContents::OxygenTank => (), // should never get hit
                    }
                }
            }
        }

        active_positions = new_positions;
        step_count = step_count + 1;
    }

    step_count - 1 // -1 because we used an extra step to notice that no more steps were needed
}