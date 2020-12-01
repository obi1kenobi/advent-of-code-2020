use std::{collections::{HashMap, HashSet}, fs};

fn main() {
    let contents = fs::read_to_string("./input.txt").unwrap();
    let target_sum = 2020;

    let numbers_in_file = contents
        .trim()
        .split("\n")
        .map( |x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let (pair_num_a, pair_num_b) = find_pair_sum(target_sum, &numbers_in_file).unwrap();
    println!("{} * {} = {}", pair_num_a, pair_num_b, pair_num_a * pair_num_b);

    let (triplet_num_a, triplet_num_b, triplet_num_c) =
        find_triplet_sum(target_sum, &numbers_in_file).unwrap();
    println!(
        "{} * {} * {} = {}",
        triplet_num_a, triplet_num_b, triplet_num_c, triplet_num_a * triplet_num_b * triplet_num_c
    );
}


fn find_pair_sum(target_sum: i32, numbers: &Vec<i32>) -> Option<(i32, i32)> {
    let mut seen_numbers: HashSet<i32> = HashSet::new();

    for number in numbers.into_iter() {
        let remainder = target_sum - number;
        if seen_numbers.contains(&remainder) {
            return Some((remainder, *number));
        }
        seen_numbers.insert(*number);
    }
    None
}


fn find_triplet_sum(target_sum: i32, numbers: &Vec<i32>) -> Option<(i32, i32, i32)> {
    let mut remainder_parts: HashMap<i32, HashSet<i32>> = HashMap::new();

    for (index, num_a) in numbers.into_iter().enumerate().into_iter() {
        for num_b_index in (index + 1)..numbers.len() {
            let num_b = numbers[num_b_index];
            let current_sum = num_a + num_b;

            let remainder = target_sum - current_sum;
            let parts = remainder_parts
                .entry(remainder)
                .or_insert(HashSet::new());
            parts.insert(*num_a);
        }
    }

    for num_c in numbers.into_iter() {
        if let Some(parts) = remainder_parts.get(num_c) {
            let num_a = parts.into_iter().next().unwrap();
            let num_b = target_sum - num_a - num_c;
            return Some((*num_a, num_b, *num_c));
        }
    }

    None
}
