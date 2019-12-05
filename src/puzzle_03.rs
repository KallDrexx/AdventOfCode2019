use std::collections::{HashSet};
use std::fs::File;
use std::io::Read;

const START_X: i32 = 0;
const START_Y: i32 = 0;

#[derive(Eq, PartialEq, Clone, Hash)]
struct Point {
    pub x: i32,
    pub y: i32,
}

struct Wire {
    pub set: HashSet<Point>,
    pub list: Vec<Point>,
}

pub fn run() {
    let (line1, line2) = read_inputs();
    let wire1 = build_points(line1);
    let wire2 = build_points(line2);

    let distance = calc_distance_of_closest_intersection(wire1, wire2);
    println!("Closest distance: {}", distance);
}

fn read_inputs() -> (String, String) {
    let mut file = File::open("src/inputs/03A.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let mut lines = content.lines();
    let first = lines.next().unwrap().to_owned();
    let second = lines.next().unwrap().to_owned();
    (first, second)
}

fn build_points(raw_path: String) -> Wire {
    let mut current_point = Point {x: START_X, y: START_Y};
    let mut point_set = HashSet::new();
    let mut point_list = Vec::new();

    for path in raw_path.split(",") {
        let (direction, raw_count) = path.split_at(1);
        let spaces_count = raw_count.parse::<i32>().unwrap();

        for _ in 1..=spaces_count {
            let new_point = match direction {
                "U" => Point {x: current_point.x, y: current_point.y + 1},
                "D" => Point {x: current_point.x, y: current_point.y - 1},
                "R" => Point {x: current_point.x + 1, y: current_point.y},
                "L" => Point {x: current_point.x - 1, y: current_point.y},
                x => panic!("Unknown direction {}", x),
            };

            point_set.insert(new_point.clone());
            point_list.push(new_point.clone());
            current_point = new_point;
        }
    }

    Wire {set: point_set, list: point_list}
}

fn calc_distance_of_closest_intersection(wire1: Wire, wire2: Wire) -> i32 {
    let mut closest_distance = Option::None;

    for point in &wire1.set {
        if wire2.set.contains(point) {
            //let distance = calc_manhattan_distance(START_X, START_Y, point.x, point.y);
            let distance = calc_walk_distance(point, &wire1, &wire2);
            if let Some(prev_distance) = closest_distance {
                if prev_distance > distance {
                    closest_distance = Some(distance);
                }
            } else {
                closest_distance = Some(distance);
            }
        }
    }

    closest_distance.unwrap()
}

fn calc_manhattan_distance(first_x: i32, first_y: i32, second_x: i32, second_y: i32) -> i32 {
    (first_x - second_x).abs() + (first_y - second_y).abs()
}

fn calc_walk_distance(point: &Point, wire1: &Wire, wire2: &Wire) -> i32 {
    let mut first_distance = 0;
    let mut second_distance = 0;

    for x in 0..wire1.list.len() {
        if wire1.list[x] == *point {
            first_distance = x as i32 + 1;
            break;
        }
    }

    for x in 0..wire2.list.len() {
        if wire2.list[x] == *point {
            second_distance = x as i32 + 1;
            break;
        }
    }

    first_distance + second_distance
}