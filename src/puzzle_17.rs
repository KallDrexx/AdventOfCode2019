use crate::intcode::Machine;
use std::char;
use std::collections::HashSet;

#[derive(Clone)]
enum FaceDirection { North, South, East, West }

#[derive(Eq, PartialEq, Debug)]
enum TurnDirection { Left, Right }

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Position {
    x: u32,
    y: u32,
}

#[derive(Clone)]
struct Robot {
    position: Position,
    facing: FaceDirection,
}

struct Map {
    height: u32,
    width: u32,
    scaffolding: HashSet<Position>,
    robot: Robot,
}

#[derive(Eq, PartialEq, Debug)]
struct Command {
    direction: TurnDirection,
    units: u32,
}

pub fn run() {
    let mut machine = Machine::new_from_file("src/inputs/17.txt");

    //machine.memory[0] = 2; // allow input
    machine.run_program();
    let map = get_map(&mut machine);
    let route = get_route(&map);

    let mut raw_commands = String::new();
    for x in 0..route.len() {
        if x > 0 {
            raw_commands.push(',');
        }

        match route[x].direction {
            TurnDirection::Right => raw_commands.push('R'),
            TurnDirection::Left => raw_commands.push('L'),
        }

        raw_commands.push(',');
        raw_commands.push_str(&route[x].units.to_string());
    }

    println!("Commands: {}", raw_commands);
}

fn get_map(machine: &mut Machine) -> Map {
    let mut width = 0;
    let mut from_left = 0;
    let mut from_top = 0;
    let mut blocks = HashSet::new();
    let mut robot = None;
    for output in machine.output_buffer.drain(..) {
        let ch = char::from_u32(output as u32).unwrap();
        match &ch {
            '.' => from_left = from_left + 1,
            '#' => {
                blocks.insert(Position {x: from_left, y: from_top});
                from_left = from_left + 1;
            }

            '\n' => {
                from_top = from_top + 1;
                width = from_left;
                from_left = 0;
            }

            '^' => {
                robot = Some(Robot {
                    position: Position { x: from_left, y: from_top },
                    facing: FaceDirection::North,
                });

                from_left = from_left + 1;
            }

            _ => from_left = from_left + 1,
        }

        print!("{}", ch);
    }

    let height = from_top;
    println!();

    Map { height, width, scaffolding: blocks, robot: robot.unwrap() }
}

fn render_output(machine: &mut Machine) {
    for output in machine.output_buffer.drain(..) {
        let ch = char::from_u32(output as u32).unwrap();
        print!("{}", ch);
    }
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

fn get_route(map: &Map) -> Vec<Command> {
    let mut commands = Vec::new();
    let mut robot = map.robot.clone();
    loop {
        // Assume we exhausted forward motion
        let (turn, now_facing) = match get_turn_direction(map, &robot) {
            None => break, // end of the line
            Some(x) => x,
        };

        robot.facing = now_facing;
        let spaces_moved = move_robot(map, &mut robot);
        commands.push(Command {
            direction: turn,
            units: spaces_moved,
        });
    }

    commands
}

fn get_turn_direction(map: &Map, robot: &Robot) -> Option<(TurnDirection, FaceDirection)> {
    let north = get_north(&robot.position);
    let south  = get_south(&robot.position);
    let east = get_east(&robot.position);
    let west = get_west(&robot.position);

    match robot.facing {
        FaceDirection::North => {
            if east.is_some() && map.scaffolding.contains(&east.unwrap()) {
                Some((TurnDirection::Right, FaceDirection::East))
            } else if west.is_some() && map.scaffolding.contains(&west.unwrap()) {
                Some((TurnDirection::Left, FaceDirection::West))
            } else {
                None
            }
        }

        FaceDirection::South => {
            if east.is_some() && map.scaffolding.contains(&east.unwrap()) {
                Some((TurnDirection::Left, FaceDirection::East))
            } else if west.is_some() && map.scaffolding.contains(&west.unwrap()) {
                Some((TurnDirection::Right, FaceDirection::West))
            } else {
                None
            }
        }

        FaceDirection::East => {
            if north.is_some() && map.scaffolding.contains(&north.unwrap()) {
                Some((TurnDirection::Left, FaceDirection::North))
            } else if south.is_some() && map.scaffolding.contains(&south.unwrap()) {
                Some((TurnDirection::Right, FaceDirection::South))
            } else {
                None
            }
        }

        FaceDirection::West => {
            if north.is_some() && map.scaffolding.contains(&north.unwrap()) {
                Some((TurnDirection::Left, FaceDirection::North))
            } else if south.is_some() && map.scaffolding.contains(&south.unwrap()) {
                Some((TurnDirection::Right, FaceDirection::South))
            } else {
                None
            }
        }
    }
}

fn move_robot(map: &Map, robot: &mut Robot) -> u32 {
    let mut steps = 0;
    loop {
        let next_position = match robot.facing {
            FaceDirection::North => get_north(&robot.position),
            FaceDirection::South => get_south(&robot.position),
            FaceDirection::East => get_east(&robot.position),
            FaceDirection::West => get_west(&robot.position),
        };

        if next_position.is_none() {
            break;
        }

        let position = next_position.unwrap();
        if map.scaffolding.contains(&position) {
            robot.position = position;
        } else {
            break;
        }

        steps = steps + 1;
    }

    steps
}

fn get_north(position: &Position) -> Option<Position> {
    if position.y == 0 {
        None
    } else {
        Some(Position { x: position.x, y: position.y - 1 })
    }
}

fn get_south(position: &Position) -> Option<Position> {
    Some(Position { x: position.x, y: position.y + 1 })
}

fn get_east(position: &Position) -> Option<Position> {
    Some(Position { x: position.x + 1, y: position.y })
}

fn get_west(position: &Position) -> Option<Position> {
    if position.x == 0 {
        None
    } else {
        Some(Position { x: position.x - 1, y: position.y })
    }
}

