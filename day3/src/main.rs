use std::fs;

fn get_input() -> Vec<String> {
    let input = fs::read_to_string(INPUT)
        .expect("Couldn't read file.")
        .trim()
        .split("\n")
        .map(|line| line.to_string())
        .collect::<Vec<String>>();
    input
}

fn part_one() {
    let input = get_input();
    let mut sum: u64 = 0;
    for battery in input {
        let mut joltage = String::new();

        // Divide line into individual digits
        let mut digits = battery
            .chars()
            .filter(|c| c.is_digit(10))
            .map(|c| c.to_string().parse::<u8>().unwrap())
            .collect::<Vec<u8>>();

        // Choose the first digit
        let first = &digits.clone()[..digits.len() - 1];
        let max = first.iter().max().unwrap();
        joltage += max.to_string().as_str();

        digits.remove(first.iter().position(|x| x == max).unwrap());

        // Choose the second digit
        let digits = &digits[first.iter().position(|x| x == max).unwrap()..];
        let max = digits.iter().max().unwrap();
        joltage += max.to_string().as_str();

        sum += joltage.parse::<u64>().unwrap();

        // println!("Battery: {battery}\nMax:{max}\nLen:{}", digits.len());
        // sleep(Duration::from_secs(2));
    }
    println!("Sum: {sum}");
}

const INPUT: &str = "src/input.txt";

fn main() {
    part_one();
}
