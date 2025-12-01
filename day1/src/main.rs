use std::fs;

fn calculate(instruction: String, dial: i32) -> (i32, bool) {
    println!("Instruction: {instruction}");
    let (direction, amount) = instruction.split_at(1);
    let amount: i32 = amount.parse().unwrap();
    match direction {
        "L" => {
            let result = dial - (amount % 100);
            println!("Result: {result}");
            if let 0 = result % 100 {
                return (0, true);
            }
            if result < 0 {
                return (100 + result, false);
            }
            (result, false)
        }
        "R" => {
            let result = (dial + (amount % 100));
            println!("Result: {result}");
            if let 0 = result % 100 {
                return (0, true);
            }
            (result % 100, false)
        }
        _ => unreachable!(),
    }
}

fn main() {
    let input: Vec<String> = fs::read_to_string("src/input.txt")
        .expect("Couldn't read file.")
        .trim_end()
        .split("\n")
        .map(|x| x.to_string())
        .collect();
    let mut dial = 50;
    let mut left_at_zero: u32 = 0;

    for instruction in input {
        let (new_dial, at_zero) = calculate(instruction, dial);
        dial = new_dial;
        println!("New dial value: {dial}");
        println!("");
        if at_zero {
            left_at_zero += 1;
        }
    }

    println!("Times left at zero: {left_at_zero}");
}
