pub fn run() {
    let input = get_input();
    let pattern = vec![0, 1, 0, -1];
    let offset = get_offset(&input);
    println!("Offset: {}", offset);

    let mut current = input;
    for x in 0..100 {
        let result = run_phase(&current, &pattern);
        current = result;
        println!("Phase {} completed", x);
    }

    for x in 0..8 {
        print!("{}", current[x as usize + offset as usize]);
    }
}

fn get_input() -> Vec<i32> {
    //let raw = "59791911701697178620772166487621926539855976237879300869872931303532122404711706813176657053802481833015214226705058704017099411284046473395211022546662450403964137283487707691563442026697656820695854453826690487611172860358286255850668069507687936410599520475680695180527327076479119764897119494161366645257480353063266653306023935874821274026377407051958316291995144593624792755553923648392169597897222058613725620920233283869036501950753970029182181770358827133737490530431859833065926816798051237510954742209939957376506364926219879150524606056996572743773912030397695613203835011524677640044237824961662635530619875905369208905866913334027160178".to_string();
    let raw = "03036732577212944063491565474664";
    let mut repeated = String::new();
    for _ in 0..10000 {
        repeated.push_str(raw);
    }
    repeated.chars().map(|x| x.to_digit(10).unwrap() as i32).collect()
}

fn run_phase(input: &Vec<i32>, base_pattern: &Vec<i32>) -> Vec<i32> {
    let mut results = Vec::new();
    for x in 0..input.len() {
        let pattern = build_repeating_pattern(base_pattern, (x + 1) as i32);
        let mut pattern_index = 1;
        let mut value = 0;
        for y in 0..input.len() {
            let input_digit = input[y];
            let pattern_digit = pattern[pattern_index];
            value = value + (input_digit * pattern_digit);

            pattern_index = pattern_index + 1;
            if pattern_index >= pattern.len() {
                pattern_index = 0;
            }
        }

        let result = (value % 10).abs();
        results.push(result);
    }

    results
}

fn build_repeating_pattern(pattern: &Vec<i32>, repeat_count: i32) -> Vec<i32> {
    let mut results = Vec::new();
    for x in 0..pattern.len() {
        for _ in 0..repeat_count {
            results.push(pattern[x]);
        }
    }

    results
}

fn get_offset(input: &Vec<i32>) -> i32 {
    let mut value = String::new();
    for x in 0..7 {
        let digit = input[x];
        value.push_str(digit.to_string().as_str());
    }

    value.parse().unwrap()
}