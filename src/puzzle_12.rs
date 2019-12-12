use regex::Regex;

pub fn run() {
    let mut moons = read_moons();
    part_2(&mut moons);
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Tuple {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Moon {
    position: Tuple,
    velocity: Tuple,
}

fn part_1(moons: &mut Vec<Moon>) {
    for step in 0..1001 {
        if step > 0 {
            apply_gravity(moons);
            update_position(moons);
        }

        println!("After {} steps", step);
        for moon in moons.iter() {
            let potential = moon.position.x.abs() + moon.position.y.abs() + moon.position.z.abs();
            let kinetic = moon.velocity.x.abs() + moon.velocity.y.abs() + moon.velocity.z.abs();
            let total = potential * kinetic;

            println!("{:?}", moon);
            println!("pot: {}, kin: {}, total: {}", potential, kinetic, total);
        }
        println!();
    }
}

#[derive(Eq, PartialEq, Debug)]
enum Axis { X, Y, Z }

fn part_2(moons: &mut Vec<Moon>) {
    let mut loop_rates = Vec::new();
    for axis in &[Axis::X, Axis::Y, Axis::Z] {
        let mut steps = 0;
        let mut simulation = moons.clone();
        loop {
            if steps > 0 {
                let mut is_match = false;
                for x in 0..moons.len() {
                    is_match = match axis {
                        Axis::X => moons[x].position.x == simulation[x].position.x && moons[x].velocity.x == simulation[x].velocity.x,
                        Axis::Y => moons[x].position.y == simulation[x].position.y && moons[x].velocity.y == simulation[x].velocity.y,
                        Axis::Z => moons[x].position.z == simulation[x].position.z && moons[x].velocity.z == simulation[x].velocity.z,
                    };

                    if !is_match {
                        break;
                    }
                }

                if is_match {
                    println!("1 {:?} rotation in {} steps", axis, steps);
                    loop_rates.push(steps as i128);
                    break;
                }
            }

            steps = steps + 1;
            apply_gravity(&mut simulation);
            update_position(&mut simulation);
        }
    }

    let test = lcm(loop_rates[0], lcm(loop_rates[1], loop_rates[2]));
    println!("Least common multiple: {}", test);
}

fn read_moons() -> Vec<Moon> {
    let input = "<x=-13, y=14, z=-7>
<x=-18, y=9, z=0>
<x=0, y=-3, z=-3>
<x=-15, y=3, z=-13>";

    let mut moons = Vec::new();
    let regex = Regex::new(r"<x=([0-9|-]*), y=([0-9|-]*), z=([0-9|-]*)>").unwrap();
    for line in input.lines() {
        let captures = regex.captures(line).unwrap();
        moons.push(Moon {
            position: Tuple {
                x: captures.get(1).unwrap().as_str().parse().unwrap(),
                y: captures.get(2).unwrap().as_str().parse().unwrap(),
                z: captures.get(3).unwrap().as_str().parse().unwrap(),
            },
            velocity: Tuple {
                x: 0,
                y: 0,
                z: 0,
            }
        });
    }

    moons
}

fn apply_gravity(moons: &mut Vec<Moon>) {
    for x in 0..moons.len() {
        for y in (x + 1)..moons.len() {
            if moons[x].position.x < moons[y].position.x {
                moons[x].velocity.x = moons[x].velocity.x + 1;
                moons[y].velocity.x = moons[y].velocity.x - 1;
            } else if moons[x].position.x > moons[y].position.x {
                moons[x].velocity.x = moons[x].velocity.x - 1;
                moons[y].velocity.x = moons[y].velocity.x + 1;
            }

            if moons[x].position.y < moons[y].position.y {
                moons[x].velocity.y = moons[x].velocity.y + 1;
                moons[y].velocity.y = moons[y].velocity.y - 1;
            } else if moons[x].position.y > moons[y].position.y {
                moons[x].velocity.y = moons[x].velocity.y - 1;
                moons[y].velocity.y = moons[y].velocity.y + 1;
            }

            if moons[x].position.z < moons[y].position.z {
                moons[x].velocity.z = moons[x].velocity.z + 1;
                moons[y].velocity.z = moons[y].velocity.z - 1;
            } else if moons[x].position.z > moons[y].position.z {
                moons[x].velocity.z = moons[x].velocity.z - 1;
                moons[y].velocity.z = moons[y].velocity.z + 1;
            }
        }
    }
}

fn update_position(moons: &mut Vec<Moon>) {
    for moon in moons {
        moon.position.x = moon.position.x + moon.velocity.x;
        moon.position.y = moon.position.y + moon.velocity.y;
        moon.position.z = moon.position.z + moon.velocity.z;
    }
}

fn lcm(number1: i128, number2: i128) -> i128 {
    number1 * number2 / gcd(number1, number2)
}

fn gcd(number1: i128, number2: i128) -> i128 {
    let mut remainder;
    let mut a = number1;
    let mut b = number2;
    while b != 0 {
        remainder = a % b;
        a = b;
        b = remainder;
    }

    a
}