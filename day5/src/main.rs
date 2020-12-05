use std::{collections::HashSet, fs};

fn main() {
    let contents = fs::read_to_string(
        "/mnt/c/Users/predrag/Dropbox/Documents/Code/advent-of-code-2020/day5/input.txt",
    )
    .unwrap();

    let lines: Vec<_> = contents.trim().split("\n").collect();

    println!("{}", solve_part1(&lines));
    println!("{}", solve_part2(&lines));
}

fn solve_part1(lines: &Vec<&str>) -> i32 {
    lines.iter().cloned().map(get_seat_id).max().unwrap()
}

fn solve_part2(lines: &Vec<&str>) -> i32 {
    let mut taken_seats: Vec<_> = lines.iter().cloned().map(get_seat_id).collect();
    taken_seats.sort();
    let min_seat = taken_seats.first().unwrap().clone();
    let max_seat = taken_seats.last().unwrap().clone();

    let mut free_seats: HashSet<i32> = (min_seat..=max_seat).collect();
    for taken_seat in taken_seats {
        free_seats.remove(&taken_seat);
    }

    assert!(free_seats.len() == 1);
    free_seats.iter().next().unwrap().clone()
}

fn get_seat_id(boarding_pass: &str) -> i32 {
    let mut seat_id = 0;

    for current_char in boarding_pass.chars() {
        seat_id <<= 1;
        seat_id += match current_char {
            'F' | 'L' => 0,
            'B' | 'R' => 1,
            _ => unreachable!(),
        }
    }

    seat_id
}
