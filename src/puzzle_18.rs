use std::collections::{HashSet, HashMap};

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

#[derive(Debug)]
struct Edge {
    length: i32,
    target: char,
    doors: HashSet<char>,
    keys_in_the_way: Vec<char>,
}

pub fn run() {
    let map = read_map();
    let steps = calculate_steps_to_all_keys(&map);
    println!("Completed in {:?} steps", steps);
}

fn read_map() -> Map {
    let input = "#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################".to_owned();

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

fn get_edges_to_keys(start_position: &Position, map: &Map) -> Vec<Edge> {
    #[derive(Clone)]
    struct Agent {
        current: Position,
        doors_encountered: HashSet<char>,
        keys_encountered: Vec<char>,
    }

    let mut edges = Vec::new();
    let mut steps = 0;
    let mut visited = HashSet::new();
    let mut agents = Vec::new();
    agents.push(Agent {
        current: start_position.clone(),
        doors_encountered: HashSet::new(),
        keys_encountered: Vec::new(),
    });

    loop {
        steps = steps + 1;

        if agents.len() == 0 {
            break;
        }

        let mut step_agents = Vec::new();
        for agent in agents {
            let candidates = vec![
                get_north(&agent.current),
                get_south(&agent.current),
                get_east(&agent.current),
                get_west(&agent.current),
            ];

            for candidate in candidates {
                if visited.contains(&candidate) {
                    continue;
                }

                if !map.walkable.contains(&candidate) {
                    continue;
                }

                let mut new_agent = agent.clone();
                if let Some(door) = map.doors.get(&candidate) {
                    new_agent.doors_encountered.insert(door.clone());
                }

                if let Some(key) = map.keys.get(&candidate) {
                    edges.push(Edge {
                        length: steps,
                        doors: agent.doors_encountered.clone(),
                        keys_in_the_way: agent.keys_encountered.clone(),
                        target: key.clone(),
                    });

                    new_agent.keys_encountered.push(key.clone());
                }

                visited.insert(candidate.clone());
                new_agent.current = candidate.clone();
                step_agents.push(new_agent);
            }
        }

        agents = step_agents;
    }

    edges
}

fn calculate_steps_to_all_keys(map: &Map) -> i32 {
    #[derive(Clone)]
    struct Agent {
        steps_taken: i32,
        current_key: char,
        keys_collected: HashSet<char>,
        keys_in_order: Vec<char>,
    }

    #[derive(Hash, Eq, PartialEq)]
    struct CacheItem {
        keys_in_alpha_order: Vec<char>,
        last_key: char,
    }

    let mut key_edges = HashMap::new();
    key_edges.insert('@', get_edges_to_keys(&map.start_at, &map));
    for (position, key) in &map.keys {
        key_edges.insert(key.clone(), get_edges_to_keys(&position, &map));
    }

    let mut active_agents = Vec::new();
    active_agents.push(Agent {
        current_key: '@',
        keys_collected: HashSet::new(),
        steps_taken: 0,
        keys_in_order: Vec::new(),
    });

    let mut length_cache = HashMap::new();
    let mut finished_agent: Option<Agent> = None;
    loop {
        if active_agents.len() == 0 {
            break;
        }

        let _start_count = active_agents.len();
        let agent = active_agents.remove(0);
        //println!("Agent starting at {} at {} steps ({:?})", agent.current_key, agent.steps_taken, agent.keys_in_order);

        let edges = &key_edges[&agent.current_key];
        for edge_index in 0..edges.len() {
            let edge = edges.get(edge_index).unwrap();
            if agent.keys_collected.contains(&edge.target) {
                continue; // We've already collected this key
            }

            let mut has_all_required_keys = true;
            for door in &edge.doors {
                if !agent.keys_collected.contains(&door.to_ascii_lowercase()) {
                    has_all_required_keys = false;
                    break;
                }
            }

            if !has_all_required_keys {
                continue; // We can't traverse this edge yet until we get more keys
            }

            // Make sure this edge isn't better handled by another edge (e.g. other edge picks this
            // edge's target up in it's path)
            let mut ignore_this_edge = false;
            for other_edge_index in 0..edges.len() {
                if other_edge_index == edge_index {
                    continue;
                }

                let other_edge = edges.get(other_edge_index).unwrap();
                if other_edge.keys_in_the_way.contains(&edge.target) {
                    let mut has_all_required_keys = true;
                    for door in &other_edge.doors {
                        if !agent.keys_collected.contains(&door.to_ascii_lowercase()) {
                            has_all_required_keys = false;
                            break;
                        }
                    }

                    if has_all_required_keys {
                        ignore_this_edge = true;
                        break;
                    }
                }
            }

            if ignore_this_edge {
                continue; // Since this edge is better taken care by another edge, ignore this one
            }

            let mut new_agent = agent.clone();
            new_agent.steps_taken = agent.steps_taken + edge.length;

            for middle_key in &edge.keys_in_the_way {
                if !new_agent.keys_collected.contains(middle_key) {
                    new_agent.keys_collected.insert(middle_key.clone());
                    new_agent.keys_in_order.push(middle_key.clone());
                }
            }

            new_agent.keys_collected.insert(edge.target.clone());
            new_agent.keys_in_order.push(edge.target.clone());
            new_agent.current_key = edge.target.clone();

            //println!("Taking {} steps to {} ({} total)", edge.length, edge.target, new_agent.steps_taken);
            if finished_agent.is_some() && finished_agent.as_ref().unwrap().steps_taken <= new_agent.steps_taken {
                // An agent has finished and has less number of steps than this current agent.
                // Therefore there's no point in going forward.
                //println!("At more steps than finished agent, ignoring this agent");
                continue;
            }

            let mut keys_in_alpha_order: Vec<char> = new_agent.keys_collected
                .iter()
                .map(|x| x.clone())
                .collect();

            keys_in_alpha_order.sort();

            let cache_key = CacheItem {
                keys_in_alpha_order,
                last_key: edge.target.clone(),
            };

            match length_cache.get(&cache_key) {
                None => {length_cache.insert(cache_key, new_agent.steps_taken);},
                Some(length) => {
                    if *length < new_agent.steps_taken {
                        continue; // We already have a shorter path for this segment
                    } else {
                        length_cache.insert(cache_key, new_agent.steps_taken);
                    }
                }
            }

            if new_agent.keys_collected.len() == map.keys.len() {
                println!("Agent finished {}: {:?} ({} agents remaining)", new_agent.steps_taken, new_agent.keys_in_order, active_agents.len());
                finished_agent = Some(new_agent);
            } else {
                let mut add_at = None;
                for x in 0..active_agents.len() {
                    if active_agents[x].steps_taken >= new_agent.steps_taken {
                        add_at = Some(x);
                        break;
                    }
                }

                if let Some(x) = add_at {
                    active_agents.insert(x, new_agent);
                } else {
                    active_agents.push(new_agent);
                }
            }
        }

        //println!("Agent count went from {} to {}", _start_count, active_agents.len());

        let mut _input = String::new();
        //io::stdin().read_line(&mut _input).unwrap();
    }

    finished_agent.unwrap().steps_taken
}

fn get_north(current: &Position) -> Position {Position {x: current.x, y: current.y - 1}}
fn get_south(current: &Position) -> Position {Position {x: current.x, y: current.y + 1}}
fn get_east(current: &Position) -> Position {Position {x: current.x + 1, y: current.y}}
fn get_west(current: &Position) -> Position {Position {x: current.x - 1, y: current.y}}