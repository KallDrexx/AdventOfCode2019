use std::collections::{HashSet, HashMap};
//use std::io;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct Position {
    x: i32,
    y: i32,
}

struct Map {
    walkable: HashSet<Position>,
    doors: HashMap<Position, char>,
    keys: HashMap<Position, char>,
    max_x: i32,
    max_y: i32,
    start_at: Position,
}

#[derive(Clone)]
struct Instance {
    current: Position,
    //previous: HashSet<Position>,
    previous: Option<Position>,
    collected_keys: HashSet<char>,
}

pub fn run() {
    let map = read_map();
    let steps = calculate_steps_to_all_keys(&map);
    println!("Steps: {}", steps);
}

fn read_map() -> Map {
    let input = "########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################".to_owned();

    let mut map = Map {
        walkable: HashSet::new(),
        doors: HashMap::new(),
        keys: HashMap::new(),
        max_x: 0,
        max_y: 0,
        start_at: Position {x: 0, y: 0},
    };

    let mut y = 0;
    for line in input.lines() {
       let mut x = 0;
        for ch in line.chars() {
            if ch != '#' { // ignore walls
                let position = Position { x, y };
                map.walkable.insert(position.clone());

                if ch == '@' {
                    map.start_at = position;
                } else if ch.is_alphabetic() && ch.is_uppercase() {
                    map.doors.insert(position, ch.clone());
                } else if ch.is_alphabetic() && ch.is_lowercase() {
                    map.keys.insert(position, ch.clone());
                }
            }

            if map.max_x < x {
                map.max_x = x;
            }

            x = x + 1;
        }

        if map.max_y < y {
            map.max_y = y;
        }

        y = y + 1;
    }

    map
}

fn calculate_steps_to_all_keys(map: &Map) -> i32 {
    let mut instances = Vec::new();
    instances.push(Instance {
        current: map.start_at.clone(),
        //previous: HashSet::new(),
        previous: None,
        collected_keys: HashSet::new(),
    });

    let mut steps = 0;
    loop {
        steps = steps + 1;
        let mut step_instances: Vec<Instance> = Vec::new();

        //print!("{}[2J", 27 as char);
        if instances.len() == 0 {
            panic!("No instances left");
        }

        for instance in instances {
            //println!("Agent starting at {:?}", instance.current);
            let next_positions = get_valid_next_positions(&instance, &map);
            for position in next_positions {
                //println!("Moving from {:?} to {:?}", instance.current, position);

                let mut new_instance = instance.clone();
                //new_instance.previous.insert(instance.current.clone());
                new_instance.previous = Some(instance.current.clone());
                new_instance.current = position.clone();

                if let Some(key) = map.keys.get(&position) {
                    if !new_instance.collected_keys.contains(key) {
                        //println!("Key {} collected", key);
                        new_instance.collected_keys.insert(key.clone());

                        // Since we collected a key, we need to allow moving to a previous
                        // position so that we can use that key
                        //new_instance.previous.clear();
                        new_instance.previous = None;
                    }
                }

                if new_instance.collected_keys.len() == map.keys.len() {
                    // All keys collected
                    return steps;
                }

                step_instances.push(new_instance);
            }
        }

        instances = step_instances;

        println!("Step: {} with {} instances", steps, instances.len());
//        render(map, &instances);
//        let mut input = String::new();
//        io::stdin().read_line(&mut input).unwrap();
    }
}

fn get_valid_next_positions(instance: &Instance, map: &Map) -> Vec<Position> {
    let mut positions = Vec::new();

    let candidates = vec![
        get_north(&instance.current),
        get_south(&instance.current),
        get_east(&instance.current),
        get_west(&instance.current)
    ];

    for candidate in candidates {
        //if instance.previous.contains(&candidate) {
        if instance.previous.is_some() && instance.previous.as_ref().unwrap() == &candidate {
            // We came from the candidate, so it's not valid to move back there
            continue;
        }

        if !map.walkable.contains(&candidate) {
            // Not a walkable space so not a valid next position
            continue;
        }

        if let Some(door) = map.doors.get(&candidate) {
            if !instance.collected_keys.contains(&door.to_ascii_lowercase()) {
                // We don't have the key, therefore this is not valid
                //println!("Door {} found but we don't have the key", door);
                continue;
            }
        }

        // iF we got here then it's a valid space we can walk to
        positions.push(candidate);
    }

    positions
}

fn get_north(current: &Position) -> Position {Position {x: current.x, y: current.y - 1}}
fn get_south(current: &Position) -> Position {Position {x: current.x, y: current.y + 1}}
fn get_east(current: &Position) -> Position {Position {x: current.x + 1, y: current.y}}
fn get_west(current: &Position) -> Position {Position {x: current.x - 1, y: current.y}}

fn render(map: &Map, instances: &Vec<Instance>) {
    for y in 0..=map.max_y {
        for x in 0..=map.max_x {
            let position = Position {x,y};
            if map.walkable.contains(&position) {
                let mut is_instance_position = false;
                for instance in instances {
                    if instance.current == position {
                        is_instance_position = true;
                        break;
                    }
                }

                if is_instance_position {
                    print!("@");
                } else if let Some(key) = map.keys.get(&position) {
                    print!("{}", key);
                } else if let Some(door) = map.doors.get(&position) {
                    print!("{}", door);
                } else {
                    print!(".");
                }
            } else {
                print!("#");
            }
        }

        println!();
    }
}