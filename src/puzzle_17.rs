use crate::intcode::Machine;
use std::collections::HashSet;
use std::char;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Position {
    x: u32,
    y: u32,
}

struct Map {
    height: u32,
    width: u32,
    blocks: HashSet<Position>,
}

pub fn run() {
    let mut machine = Machine::new_from_file("src/inputs/17.txt");
    machine.run_program();

    let map = get_map(&mut machine);
    let intersections = get_intersections(&map.blocks);

    let mut total = 0;
    for intersection in intersections {
        total = total + (intersection.x * intersection.y);
    }

    println!("Total: {}", total);
}

fn get_map(machine: &mut Machine) -> Map {
    let mut width = 0;
    let mut from_left = 0;
    let mut from_top = 0;
    let mut blocks = HashSet::new();
    for output in machine.output_buffer.drain(..) {
        let ch = char::from_u32(output as u32).unwrap();
        match &ch {
            '.' => from_left = from_left + 1,
            '#' => {
                blocks.insert(Position {x: from_left, y: from_top});
                from_left = from_left + 1;
            },
            '\n' => {
                from_top = from_top + 1;
                width = from_left;
                from_left = 0;
            }

            _ => from_left = from_left + 1,
        }

        print!("{}", ch);
    }

    let height = from_top;
    println!();

    Map { height, width, blocks }
}

fn get_intersections(blocks: &HashSet<Position>) -> Vec<Position> {
    let mut results = Vec::new();

    for block in blocks {
        if block.x == 0 || block.y == 0 {
            continue;
        }

        let north = Position {x: block.x, y: block.y - 1};
        let south = Position {x: block.x, y: block.y + 1};
        let east = Position {x: block.x + 1, y: block.y};
        let west = Position {x: block.x - 1, y: block.y};

        if blocks.contains(&north) &&
            blocks.contains(&south) &&
            blocks.contains(&east) &&
            blocks.contains(&west) {
            results.push(block.clone());
        }
    }

    results
}