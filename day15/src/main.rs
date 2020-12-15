use std::{collections::HashMap, fs};

fn main() {
    let contents = fs::read_to_string(
        "/mnt/c/Users/predrag/Dropbox/Documents/Code/advent-of-code-2020/day15/input.txt",
    )
    .unwrap();

    let numbers: Vec<i64> = contents
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    println!("{}", solve(&numbers, 2020));
    println!("{}", solve(&numbers, 30000000));
}

fn solve(numbers: &Vec<i64>, number_at_position: usize) -> i64 {
    let mut position: HashMap<i64, usize> = HashMap::new();
    for i in 0..numbers.len() - 1 {
        position.insert(numbers[i], i + 1);
    }

    let mut last_number = numbers[numbers.len() - 2];
    let mut next_number = numbers.last().unwrap().clone();
    for i in numbers.len()..=number_at_position {
        if position.contains_key(&next_number) {
            let last_round = position[&next_number];
            last_number = next_number;
            next_number = (i - last_round) as i64;
        } else {
            last_number = next_number;
            next_number = 0;
        }
        position.insert(last_number, i);
    }

    last_number
}
