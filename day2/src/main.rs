use std::fs;

fn get_input() -> Vec<String> {
    let input: Vec<String> = fs::read_to_string(INPUT)
        .expect("Couldn't read the file.")
        .trim()
        .split(",")
        .map(|x| x.to_string())
        .collect();

    let mut ids: Vec<String> = Vec::new();

    for range in input {
        let split: Vec<u64> = range
            .split("-")
            .map(|x| x.parse::<u64>().unwrap())
            .collect();
        for i in split[0]..=split[1] {
            ids.push(i.to_string());
        }
    }

    ids
}

fn is_invalid(id: String) -> (bool, String) {
    let split = id.split_at(id.len() / 2);

    if split.0 == split.1 {
        return (true, id);
    }
    return (false, id);
}

enum Day2 {}

impl Day2 {
    fn part_one() {
        let ids = get_input();
        let mut sum: usize = 0;

        for id in ids {
            let (invalid, invalid_id) = is_invalid(id.clone());
            if invalid {
                sum += invalid_id.parse::<usize>().unwrap();
            }
        }

        println!("Sum: {sum}");
    }
}

const INPUT: &str = "src/input.txt";

fn main() {
    Day2::part_one();
}
