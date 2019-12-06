use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::ops::Index;

pub fn run() {
    let input = read_input();
    let objects = build_object_map(input);

    let count: i32 = objects.names.iter()
        .map(|x| get_orbit_counts(x, 0, &objects.orbit_info))
        .sum();

    println!("Total orbits {}", count);

    let transfers = get_transfer_count(&objects.orbit_info);
    println!("Transfers: {}", transfers);
}

struct Object {
    pub id: String,
    pub orbits: Option<String>,
}

struct Objects {
    pub names: HashSet<String>,
    pub orbit_info: HashMap<String, Object>,
}

fn build_object_map(input: String) -> Objects {
    let mut map = HashMap::new();
    let mut set = HashSet::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split(')').collect();
        if parts.len() != 2 {
            panic!("Incorrect number of parts for object: {}", parts.len());
        }

        let inner = parts[0].to_owned();
        let outer = parts[1].to_owned();

        if !map.contains_key(&inner) {
            map.insert(inner.clone(), Object { id: inner.clone(), orbits: None });
        }

        map.insert(outer.clone(), Object { id: outer.clone(), orbits: Some(inner.clone())});
        set.insert(inner.clone());
        set.insert(outer.clone());
    }

    Objects { names: set, orbit_info: map }
}

fn get_orbit_counts(obj_name: &String, depth: i32, map: &HashMap<String, Object>) -> i32 {
    match map.get(obj_name) {
        None => panic!("Obj {} wasn't found in the map"),
        Some(obj) => {
            match &obj.orbits {
                None => depth,
                Some(inner) => get_orbit_counts(inner, depth + 1, map)
            }
        }
    }
}

fn get_orbit_transfers(obj_name: &String, map: &HashMap<String, Object>, mut list: Vec<String>) -> Vec<String> {
    match map.get(obj_name) {
        None => panic!("Obj {} wasn't found in the map"),
        Some(obj) => {
            match &obj.orbits {
                None => list,
                Some(inner) => {
                    list.push(inner.clone());
                    get_orbit_transfers(inner, map, list)
                }
            }
        }
    }
}

fn get_transfer_count(map: &HashMap<String, Object>) -> i32 {
    let you_transfers = get_orbit_transfers(&"YOU".to_owned(), map, Vec::new());
    let san_transfers = get_orbit_transfers(&"SAN".to_owned(), map, Vec::new());

    println!("length {}", you_transfers.len());

    for x in 0..san_transfers.len() {
        for y in 0..you_transfers.len() {
            println!("{} {}", san_transfers[x], you_transfers[y]);
            if san_transfers[x] == you_transfers[y] {
                return (x + y) as i32;
            }
        }
    }

    panic!("No common objects found!");
}

fn read_input() -> String {
    let mut file = File::open("src/inputs/06A.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    content
}