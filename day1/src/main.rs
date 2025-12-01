use std::fs;

fn get_input() -> Vec<String> {
    let input: Vec<String> = fs::read_to_string("src/input.txt")
        .expect("Couldn't read file.")
        .trim_end()
        .split("\n")
        .map(|x| x.to_string())
        .collect();
    input
}

fn calculate_part_one(instruction: String, dial: i32) -> (i32, bool) {
    let (direction, amount) = instruction.split_at(1);
    let amount: i32 = amount.parse().unwrap();
    match direction {
        "L" => {
            let result = dial - (amount % 100);
            if let 0 = result % 100 {
                return (0, true);
            }
            if result < 0 {
                return (100 + result, false);
            }
            (result % 100, false)
        }
        "R" => {
            let result = dial + (amount % 100);
            if let 0 = result % 100 {
                return (0, true);
            }
            (result % 100, false)
        }
        _ => unreachable!(),
    }
}

fn calculate_part_two(instruction: String, dial: i32, mut at_zero: u32) -> (i32, u32) {
    let (direction, amount) = instruction.split_at(1);
    let amount: i32 = amount.parse().unwrap();

    match direction {
        "L" => {
            let result = (dial - (amount % 100)) % 100;
            let spins = amount / 100;
            if result <= 0 {
                if dial != 0 {
                    at_zero += 1;
                }
                return ((100 + result) % 100, at_zero + spins as u32);
            }
            return (result, at_zero + spins as u32);
        }
        "R" => {
            let result = (dial + amount) % 100;
            let spins = amount / 100;
            if dial + (amount % 100) >= 100 {
                at_zero += 1;
            }
            return (result, at_zero + spins as u32);
        }
        _ => unreachable!(),
    }
}

fn part_one() {
    let input = get_input();
    let mut dial = 50;
    let mut left_at_zero: u32 = 0;

    for instruction in input {
        let (new_dial, at_zero) = calculate_part_one(instruction, dial);
        dial = new_dial;
        if at_zero {
            left_at_zero += 1;
        }
    }

    println!("Times left at zero: {left_at_zero}");
}

fn part_two() {
    let input = get_input();
    let mut dial = 50;
    let mut at_zero: u32 = 0;
    for instruction in input {
        (dial, at_zero) = calculate_part_two(instruction, dial, at_zero);
        println!("At zero: {at_zero}");
    }
}

fn main() {
    part_one();
    part_two();
}
