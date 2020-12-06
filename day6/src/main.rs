use std::{collections::HashSet, fs};

fn main() {
    let contents = fs::read_to_string(
        "/mnt/c/Users/predrag/Dropbox/Documents/Code/advent-of-code-2020/day6/input.txt",
    )
    .unwrap();

    let groups: Vec<Vec<_>> = contents
        .trim()
        .split("\n\n")
        .map(|x| x.split("\n").collect())
        .collect();

    println!("{}", solve_part1(&groups));
    println!("{}", solve_part2(&groups));
}

fn solve_part1(groups: &Vec<Vec<&str>>) -> usize {
    let mut total_positive_answers = 0;

    for group in groups.iter() {
        let mut answers: HashSet<char> = HashSet::new();

        for person in group.iter().cloned() {
            answers.extend(person.chars());
        }

        total_positive_answers += answers.len();
    }

    total_positive_answers
}

fn solve_part2(groups: &Vec<Vec<&str>>) -> usize {
    let mut total_positive_answers = 0;

    for group in groups.iter() {
        let mut initialized = false;
        let mut answers: HashSet<char> = HashSet::new();

        for person in group.iter().cloned() {
            if !initialized {
                answers = person.chars().collect();
                initialized = true;
            } else {
                answers = answers
                    .intersection(&person.chars().collect())
                    .cloned()
                    .collect();
            }
        }

        total_positive_answers += answers.len();
    }

    total_positive_answers
}
