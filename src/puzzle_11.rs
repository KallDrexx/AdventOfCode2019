use crate::intcode::{Machine, MachineState};
use std::collections::{HashSet, HashMap};

enum Direction { Up, Down, Left, Right }

#[derive(Clone)]
enum Color { Black, White }

#[derive(Eq, PartialEq, Hash, Clone)]
struct Point {
    x: i32,
    y: i32,
}

pub fn run() {
    let mut machine = Machine::new_from_file("src/inputs/11A.txt");
    let mut current_direction = Direction::Up;
    let mut visited_points = HashSet::new();
    let mut point_colors = HashMap::new();
    let mut current_point = Point { x: 0, y: 0 };
    let mut has_moved_once = false;
    let mut max_coords = Point { x: 0, y: 0 };
    let mut min_coords = Point { x: 0, y: 0 };

    loop {
        let state = machine.run_program();
        if !machine.output_buffer.is_empty() {
            let color = match machine.output_buffer.remove(0).unwrap() {
                0 => Color::Black,
                1 => Color::White,
                x => panic!("Invalid color code {}", x),
            };

            let direction_code = machine.output_buffer.remove(0).unwrap();
            if direction_code != 0 && direction_code != 1 {
                panic!("Invalid direction code {}", direction_code);
            }

            current_direction = match current_direction {
                Direction::Up => if direction_code == 0 { Direction::Left } else { Direction:: Right },
                Direction::Down => if direction_code == 0 { Direction::Right } else { Direction::Left },
                Direction::Left => if direction_code == 0 { Direction::Down } else { Direction::Up },
                Direction::Right => if direction_code == 0 { Direction::Up } else { Direction::Down },
            };

            // Paint the tile
            visited_points.insert(current_point.clone());
            point_colors.insert(current_point.clone(), color);

            // Move
            current_point = match current_direction {
                Direction::Up => Point { x: current_point.x, y: current_point.y + 1 },
                Direction::Down => Point { x: current_point.x, y: current_point.y - 1 },
                Direction::Left => Point { x: current_point.x - 1, y: current_point.y },
                Direction::Right => Point { x: current_point.x + 1, y: current_point.y },
            };

            if current_point.x > max_coords.x {
                max_coords = Point { x: current_point.x, y: max_coords.y }
            }

            if current_point.y > max_coords.y {
                max_coords = Point { x: max_coords.x, y: current_point.y }
            }

            if current_point.x < min_coords.x {
                min_coords = Point { x: current_point.x, y: min_coords.y }
            }

            if current_point.y < min_coords.y {
                min_coords = Point { x: min_coords.x, y: current_point.y }
            }
        }

        match state {
            MachineState::Halted => break,
            MachineState::WaitingForInput => {
                let input_value = match point_colors.get(&current_point) {
                    None => match has_moved_once {
                        true => 0,
                        false => 1, // first tile starts white
                    },
                    Some(color) => match color {
                        Color::Black => 0,
                        Color::White => 1,
                    }
                };

                machine.input_buffer.push_back(input_value);
            }
        }

        has_moved_once = true;
    }

    println!("Visited {} points", visited_points.len());
    println!();

    render(min_coords, max_coords, point_colors);
}

fn render(min_coords: Point, max_coords: Point, color_map: HashMap<Point, Color>) {
    for y in (min_coords.y..=max_coords.y).rev() {
        for x in min_coords.x..=max_coords.x {
            let point = Point { x, y };
            let color = match color_map.get(&point) {
                None => Color::Black,
                Some(color) => color.clone(),
            };

            match color {
                Color::Black => print!(" "),
                Color::White => print!("#"),
            }
        }

        println!();
    }
}