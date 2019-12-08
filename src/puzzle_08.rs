use std::fs::{File};
use std::io::Read;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

pub fn run() {
    let input = read_input();
    let layers = split_into_layers(input, WIDTH, HEIGHT);

    calc_part_1(&layers);

    for layer in &layers {
        println!("{:?}", layer);
    }

    let mut resulting_image = Vec::new();
    for x in 0..(WIDTH * HEIGHT) {
        let mut pixel = 0;
        for y in 0..layers.len() {
            if layers[y][x] != 2 {
                // White or black
                pixel = layers[y][x];
                break;
            }
        }

        resulting_image.push(pixel);
    }

    println!();
    println!("Final image:");
    println!("{:?}", resulting_image);
    render_layer(&resulting_image);
}

fn read_input() -> Vec<i32> {
    let mut file = File::open("src/inputs/08a.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    content.trim().chars()
        .map(|ch| ch.to_string().parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn split_into_layers(mut input: Vec<i32>, width: usize, height: usize) -> Vec<Vec<i32>> {
    let mut results = Vec::new();

    loop {
        if input.len() == width * height {
            break;
        }

        let remaining = input.split_off(width * height);
        results.push(input.clone());

        input = remaining;
    }

    results.push(input);
    results
}

fn render_layer(layer: &Vec<i32>) {
    for x in 0..layer.len() {
        if x % WIDTH == 0 {
            println!();
        }

        if layer[x] == 0 {
            print!(" ")
        } else {
            print!("█")
        }
    }

    println!();
    println!();
}

fn calc_part_1(layers: &Vec<Vec<i32>>) {
    let mut min_zeros = 5000;
    let mut answer = 0;
    for x in 0..layers.len() {
        let mut zero_count = 0;
        let mut one_count = 0;
        let mut two_count = 0;

        for digit in &layers[x] {
            if *digit == 0 { zero_count = zero_count + 1 }
            if *digit == 1 { one_count = one_count + 1 }
            if *digit == 2 { two_count = two_count + 1 }
        }

        if zero_count < min_zeros {
            min_zeros = zero_count;
            answer = one_count * two_count;
        }

        //println!("Layer {}: {} {} {}", x + 1, zero_count, one_count, two_count);
    }

    println!("Part 1 answer: {}", answer);
}