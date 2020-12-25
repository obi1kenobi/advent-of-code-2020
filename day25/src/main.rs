use std::fs;

fn main() {
    let contents = fs::read_to_string(
        "/mnt/c/Users/predrag/Dropbox/Documents/Code/advent-of-code-2020/day25/input.txt",
    )
    .unwrap();

    let subject_number: i64 = 7;
    let modulus: i64 = 20201227;
    let lines: Vec<i64> = contents
        .trim()
        .split('\n')
        .map(|x| x.parse().unwrap())
        .collect();
    let card_pubkey = lines[0];
    let door_pubkey = lines[1];

    println!(
        "{}",
        solve_part1(subject_number, modulus, card_pubkey, door_pubkey)
    );
}

fn calculate_powmod(base: i64, power: i64, modulus: i64) -> i64 {
    let mut value = 1i64;
    for _ in 0..power {
        value = (value * base) % modulus;
    }
    value
}

fn calculate_discrete_log(base: i64, modulus: i64, powmod: i64) -> i64 {
    let mut value = 1i64;
    for discrete_log in 0..modulus {
        if value == powmod {
            return discrete_log;
        }
        value = (value * base) % modulus;
    }

    unreachable!()
}

fn solve_part1(subject_number: i64, modulus: i64, card_pubkey: i64, door_pubkey: i64) -> i64 {
    let card_secret_key = calculate_discrete_log(subject_number, modulus, card_pubkey);
    let door_secret_key = calculate_discrete_log(subject_number, modulus, door_pubkey);

    assert_eq!(
        card_pubkey,
        calculate_powmod(subject_number, card_secret_key, modulus)
    );
    assert_eq!(
        door_pubkey,
        calculate_powmod(subject_number, door_secret_key, modulus)
    );

    let door_calculated_key = calculate_powmod(card_pubkey, door_secret_key, modulus);
    let card_calculated_key = calculate_powmod(door_pubkey, card_secret_key, modulus);

    assert_eq!(card_calculated_key, door_calculated_key);

    card_calculated_key
}
