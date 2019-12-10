use std::f32;

pub fn run() {
    let input = read_input();
    let points = get_points(input);

    let mut candidate = None;
    let mut visible_count = 0;

    for point1 in &points {
        let mut angles = Vec::new();
        for point2 in &points {
            if point1 == point2 {
                continue;
            }

            let angle = get_angle(point1, point2);
            let mut valid = true;
            for existing_angle in &angles {
                if (existing_angle - angle).abs() < f32::EPSILON {
                    valid = false;
                    break;
                }
            }

            if valid {
                angles.push(angle);
            }
        }

        if visible_count < angles.len() {
            candidate = Some(point1);
            visible_count = angles.len();
        }
    }

    println!("{:?} ({})", candidate, visible_count);
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Point {
    x: i32,
    y: i32,
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
    distance_y.atan2(distance_x)
}
