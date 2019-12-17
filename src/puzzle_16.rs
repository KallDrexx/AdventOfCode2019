use std::time::{SystemTime};

pub fn run() {
    let input = get_input();
    let offset = get_offset(&input) as usize;
    println!("Offset: {}", offset);
    println!("Starting message: {:?}", &input[offset..(offset + 8)]);

    let mut current = input;
    for x in 0..100 {
        let now = SystemTime::now();
        let result = run_phase(&current);
        current = result;

        let elapsed = now.elapsed().unwrap();
        println!("Phase {} completed ({}ms): {:?}", x, elapsed.as_millis(), &current[offset..(offset + 8)]);
    }

    println!("{:?}", current);
}

fn get_input() -> Vec<i32> {
    //let raw = "59791911701697178620772166487621926539855976237879300869872931303532122404711706813176657053802481833015214226705058704017099411284046473395211022546662450403964137283487707691563442026697656820695854453826690487611172860358286255850668069507687936410599520475680695180527327076479119764897119494161366645257480353063266653306023935874821274026377407051958316291995144593624792755553923648392169597897222058613725620920233283869036501950753970029182181770358827133737490530431859833065926816798051237510954742209939957376506364926219879150524606056996572743773912030397695613203835011524677640044237824961662635530619875905369208905866913334027160178";
    let raw = "03036732577212944063491565474664";
    let mut repeated = String::new();
    for _ in 0..10000 {
        repeated.push_str(raw);
    }
    repeated.chars().map(|x| x.to_digit(10).unwrap() as i32).collect()
}

fn run_phase(input: &Vec<i32>) -> Vec<i32> {
    let mut results = Vec::new();
    for y in 0..input.len() {
        let digit = calc_digit(input, y);
        results.push(digit);
    }

    results
}

fn calc_digit(input: &Vec<i32>, digit_num: usize) -> i32 {
    let repeat_count = digit_num + 1;
    let mut value = 0;

    // adds
    let mut index = repeat_count - 1;
    while index < input.len() {
        for x in 0..repeat_count {
            if index + x < input.len() {
                //print!("+ {} ", input[index + x]);
                value = value + input[index + x];
            }
        }

        index = index + (4 * repeat_count);
    }

    // Sub
    index = (repeat_count * 3) - 1;
    while index < input.len() {
        for x in 0..repeat_count {
            if index + x < input.len() {
                //print!("- {} ", input[index + x]);
                value = value - input[index + x];
            }
        }

        index = index + (4 * repeat_count);
    }

    let result = (value % 10).abs();
    result
}

fn get_offset(input: &Vec<i32>) -> i32 {
    let mut value = String::new();
    for x in 0..7 {
        let digit = input[x];
        value.push_str(digit.to_string().as_str());
    }

    value.parse().unwrap()
}