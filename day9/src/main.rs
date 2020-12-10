use std::{collections::HashSet, fs};

fn main() {
    let contents = fs::read_to_string(
        "/mnt/c/Users/predrag/Dropbox/Documents/Code/advent-of-code-2020/day9/input.txt",
    )
    .unwrap();

    let numbers: Vec<i64> = contents
        .trim()
        .split("\n")
        .map(|x| x.parse().unwrap())
        .collect();

    let preamble_length: usize = 25;
    let solution_part1 = solve_part1(&numbers, preamble_length);
    println!("{}", solution_part1);
    println!("{}", solve_part2(&numbers, solution_part1));
}

fn solve_part1(numbers: &Vec<i64>, preamble_length: usize) -> i64 {
    let mut valid_window: HashSet<i64> = HashSet::new();

    for i in 0..preamble_length {
        valid_window.insert(numbers[i]);
    }
    for i in preamble_length..numbers.len() {
        if !is_valid_sum(numbers[i], &valid_window) {
            return numbers[i];
        }

        valid_window.remove(&numbers[i - preamble_length]);
        valid_window.insert(numbers[i]);
    }

    unreachable!();
}

fn is_valid_sum(current_number: i64, window: &HashSet<i64>) -> bool {
    for number in window.iter() {
        let remainder = current_number - number;
        if remainder != *number && window.contains(&remainder) {
            return true;
        }
    }

    false
}

fn solve_part2(numbers: &Vec<i64>, target_number: i64) -> i64 {
    let mut low_water_mark: usize = 0;
    let mut high_water_mark: usize = 0;
    let mut current_sum: i64 = 0;

    while high_water_mark < numbers.len() {
        assert!(low_water_mark <= high_water_mark);

        if current_sum == target_number {
            break;
        } else if current_sum < target_number {
            current_sum += numbers[high_water_mark];
            high_water_mark += 1;

            if current_sum == target_number {
                break;
            }
        } else if current_sum > target_number {
            current_sum -= numbers[low_water_mark];
            low_water_mark += 1;
        } else {
            unreachable!();
        }
    }

    if current_sum != target_number {
        unreachable!();
    }

    let mut numbers_in_window: Vec<i64> = Vec::new();
    for i in low_water_mark..high_water_mark {
        numbers_in_window.push(numbers[i]);
    }

    numbers_in_window.sort();

    numbers_in_window.first().unwrap() + numbers_in_window.last().unwrap()
}
