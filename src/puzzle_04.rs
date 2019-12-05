
pub fn run() {
    let count = calculate(347312, 805915);
    println!("Count: {}", count);
}

fn calculate(min: i32, max: i32) -> i32 {
    let mut count = 0;
    for x in min..max {
        if is_valid(x) {
            count = count + 1;
        }
    }

    count
}

fn is_valid(number: i32) -> bool {
    let mut double_count = 0;
    let as_string = number.to_string();
    let digits = vec![
        as_string.get(0..1).unwrap().parse::<u8>().unwrap(),
        as_string.get(1..2).unwrap().parse::<u8>().unwrap(),
        as_string.get(2..3).unwrap().parse::<u8>().unwrap(),
        as_string.get(3..4).unwrap().parse::<u8>().unwrap(),
        as_string.get(4..5).unwrap().parse::<u8>().unwrap(),
        as_string.get(5..6).unwrap().parse::<u8>().unwrap(),
    ];

    let mut last_digit = digits[0];
    let mut current_dup_count = 0;
    let mut has_set_of_two = false;

    for current_digit in digits.into_iter().skip(1) {
        if current_digit < last_digit {
            return false;
        }

        if current_digit == last_digit {
            current_dup_count = current_dup_count + 1;
            double_count = double_count + 1;
        } else {
            if current_dup_count > 0 && current_dup_count == 1 {
                has_set_of_two = true;
            }

            current_dup_count = 0;
        }

        last_digit = current_digit;
    }

    if current_dup_count > 0 && current_dup_count == 1 {
        has_set_of_two = true;
    }

    has_set_of_two
}