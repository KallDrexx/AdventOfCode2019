use std::f32;
use std::collections::HashSet;

pub fn run() {
    let input = read_input();
    let points = get_points(input);

    let mut candidate = None;
    let mut visible_count = 0;

    for point1 in &points {
        let mut references: Vec<Reference> = Vec::new();
        for point2 in &points {
            if point1 == point2 {
                continue;
            }

            let angle = get_angle(point1, point2);

            let mut reference_index = None;
            for x in 0..references.len() {
                let difference = (references[x].angle - angle).abs();
                if difference < f32::EPSILON {
                    reference_index = Some(x);
                    break;
                }
            }

            let reference = if let Some(x) = reference_index {
                references.get_mut(x).unwrap()
            } else {
                references.push(Reference {
                    origin: point1.clone(),
                    angle,
                    points: Vec::new(),
                });

                let index = references.len() - 1;
                references.get_mut(index).unwrap()
            };

            reference.points.push(point2.clone());
        }

        if visible_count < references.len() {
            visible_count = references.len();
            candidate = Some(references);
        }
    }

    println!("total: {}", visible_count);

    let references = sort_refs(candidate.unwrap());
    let mut first_index = 0;
    for x in 0..references.len() {
        if references[x].angle >= 270_f32 {
            first_index = x;
            break;
        }
    }

    let mut destroyed_points = HashSet::new();
    let mut hit_count = 0;
    let mut current_index = first_index;
    while hit_count < visible_count {
        let reference = references.get(current_index).unwrap();
        for point in &reference.points {
            if !destroyed_points.contains(point) {
                destroyed_points.insert(point.clone());
                hit_count = hit_count + 1;

                println!("{}: {:?}", hit_count, point);
                break;
            }
        }

        current_index = current_index + 1;
        if current_index >= references.len() {
            current_index = 0;
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Reference {
    origin: Point,
    angle: f32,
    points: Vec<Point>,
}

fn read_input() -> String {
    "#..#....#...#.#..#.......##.#.####
#......#..#.#..####.....#..#...##.
.##.......#..#.#....#.#..#.#....#.
###..#.....###.#....##.....#...#..
...#.##..#.###.......#....#....###
.####...##...........##..#..#.##..
..#...#.#.#.###....#.#...##.....#.
......#.....#..#...##.#..##.#..###
...###.#....#..##.#.#.#....#...###
..#.###.####..###.#.##..#.##.###..
...##...#.#..##.#............##.##
....#.##.##.##..#......##.........
.#..#.#..#.##......##...#.#.#...##
.##.....#.#.##...#.#.#...#..###...
#.#.#..##......#...#...#.......#..
#.......#..#####.###.#..#..#.#.#..
.#......##......##...#..#..#..###.
#.#...#..#....##.#....#.##.#....#.
....#..#....##..#...##..#..#.#.##.
#.#.#.#.##.#.#..###.......#....###
...#.#..##....###.####.#..#.#..#..
#....##..#...##.#.#.........##.#..
.#....#.#...#.#.........#..#......
...#..###...#...#.#.#...#.#..##.##
.####.##.#..#.#.#.#...#.##......#.
.##....##..#.#.#.......#.....####.
#.##.##....#...#..#.#..###..#.###.
...###.#..#.....#.#.#.#....#....#.
......#...#.........##....#....##.
.....#.....#..#.##.#.###.#..##....
.#.....#.#.....#####.....##..#....
.####.##...#.......####..#....##..
.#.#.......#......#.##..##.#.#..##
......##.....##...##.##...##......".to_owned()
}

fn get_points(input: String) -> Vec<Point> {
    let mut results = Vec::new();
    let mut y = 0;
    for line in input.lines() {
        let mut x = 0;
        for char in line.chars() {
            if char == '#' {
                results.push(Point {x, y});
            }

            x = x + 1;
        }

        y = y + 1;
    }

    results
}

fn get_angle(point1: &Point, point2: &Point) -> f32 {
    let distance_x = (point2.x - point1.x) as f32;
    let distance_y = (point2.y - point1.y) as f32;
    let radians = distance_y.atan2(distance_x);
    let degrees = 180_f32 / f32::consts::PI * radians;

    if degrees < 0_f32 {
        360_f32 + degrees
    } else if degrees > 360_f32 {
        degrees - 360_f32
    } else {
        degrees
    }
}

fn get_distance(point1: &Point, point2: &Point) -> f32 {
    let distance_x = point2.x - point1.x;
    let distance_y = point2.y - point1.y;

    ((distance_x.pow(2) + distance_y.pow(2)) as f32).sqrt()
}

fn sort_refs(mut references: Vec<Reference>) -> Vec<Reference> {
    references.sort_by(|a, b| a.angle.partial_cmp(&b.angle).unwrap());
    for reference in &mut references {
        let origin_point = &reference.origin;
        reference.points.sort_by(|a, b| {
            let a_distance = get_distance(origin_point, a);
            let b_distance = get_distance(origin_point, b);
            a_distance.partial_cmp(&b_distance).unwrap()
        })
    }

    references
}