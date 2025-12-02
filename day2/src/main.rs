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

enum Isinvalid {}
impl Isinvalid {
    fn p1(id: String) -> (bool, String) {
        let split = id.split_at(id.len() / 2);

        if split.0 == split.1 {
            return (true, id);
        }
        return (false, id);
    }

    fn p2(id: String) -> (bool, String) {
        let max = id.len() / 2;

        for i in 1..=max {
            let mut splits: Vec<String> = Vec::new();
            let mut to_split = id.clone();
            while !to_split.is_empty() {
                if to_split.len() <= i {
                    splits.push(to_split);
                    break;
                }

                let (first, rest) = to_split.split_at(i);
                splits.push(first.to_string());
                to_split = rest.to_string();
            }
            if splits.windows(2).all(|x| x[0] == x[1]) {
                return (true, id);
            }
        }
        return (false, id);
    }
}

enum Day2 {}
impl Day2 {
    fn part_one() {
        let ids = get_input();
        let mut sum: usize = 0;

        for id in ids {
            let (invalid, invalid_id) = Isinvalid::p1(id.clone());
            if invalid {
                sum += invalid_id.parse::<usize>().unwrap();
            }
        }

        println!("Sum: {sum}");
    }

    fn part_two() {
        let ids = get_input();
        let mut sum: usize = 0;

        for id in ids {
            let result = Isinvalid::p2(id);
            if result.0 {
                sum += result.1.parse::<usize>().unwrap()
            }
        }
        println!("Sum: {sum}");
    }
}

const INPUT: &str = "src/input.txt";

fn main() {
    Day2::part_one();
    Day2::part_two();
}
