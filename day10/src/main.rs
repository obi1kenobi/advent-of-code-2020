use std::{cmp::max, fs};

fn main() {
    let contents = fs::read_to_string(
        "/mnt/c/Users/predrag/Dropbox/Documents/Code/advent-of-code-2020/day10/input.txt",
    )
    .unwrap();

    let lines: Vec<i64> = contents
        .trim()
        .split("\n")
        .map(|x| x.parse().unwrap())
        .collect();

    let (one_diff, three_diff) = solve_part1(&lines);
    println!("{} * {} = {}", one_diff, three_diff, one_diff * three_diff);
    println!("{}", solve_part2(&lines));
}

fn solve_part1(adapters: &Vec<i64>) -> (i64, i64) {
    let mut all_adapters: Vec<i64> = adapters.clone();
    all_adapters.sort();
    all_adapters.insert(0, 0);
    all_adapters.push(all_adapters.last().unwrap() + 3);

    let mut one_diff: i64 = 0;
    let mut three_diff: i64 = 0;

    let mut current_adapter = all_adapters[0];
    for next_adapter in all_adapters[1..all_adapters.len()].iter().cloned() {
        match next_adapter - current_adapter {
            3 => three_diff += 1,
            2 => {}
            1 => one_diff += 1,
            _ => unreachable!(),
        }
        current_adapter = next_adapter;
    }

    (one_diff, three_diff)
}

fn solve_part2(adapters: &Vec<i64>) -> i64 {
    let mut all_adapters: Vec<i64> = adapters.clone();
    all_adapters.sort();

    let target_max_joltage = *all_adapters.last().unwrap();

    let mut dp: Vec<i64> = Vec::new();
    dp.resize((target_max_joltage + 1) as usize, 0);

    dp[0] = 1;

    for current_adapter in all_adapters.iter().cloned() {
        let current_joltage = current_adapter as usize;

        for acceptable_joltage in max(0, current_adapter - 3) as usize..current_joltage {
            dp[current_joltage] += dp[acceptable_joltage];
        }
    }

    dp[target_max_joltage as usize]
}
