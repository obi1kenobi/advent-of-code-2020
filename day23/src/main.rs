use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    collections::VecDeque,
    fs::{self, File},
    path::Path,
};

fn main() {
    let contents = fs::read_to_string(
        "/mnt/c/Users/predrag/Dropbox/Documents/Code/advent-of-code-2020/day23/input.txt",
    )
    .unwrap();

    let lines: Vec<_> = contents.trim().split('\n').collect();
    assert_eq!(lines.len(), 1);
    let cups: VecDeque<i64> = lines[0]
        .chars()
        .map(|x| x.to_string().parse().unwrap())
        .collect();

    println!("{}", solve_part1(&cups, 100));
    println!("{}", solve_part2(&cups, 10000000));
}

fn find_destination_cup_index(cups: &VecDeque<i64>, target_cup_number: i64) -> (usize, i64) {
    let mut best_candidate_below = Option::None;
    let mut best_candidate_above: Option<(usize, i64)> = Option::None;
    for (index, &cup) in cups.iter().enumerate() {
        match cup.cmp(&target_cup_number) {
            Ordering::Equal => {
                best_candidate_below = Option::Some((index, cup));
                break;
            }
            Ordering::Less => {
                if best_candidate_below.is_none() || cup > best_candidate_below.unwrap().1 {
                    best_candidate_below = Option::Some((index, cup));
                }
            }
            Ordering::Greater => {
                if best_candidate_above.is_none() || cup > best_candidate_above.unwrap().1 {
                    best_candidate_above = Option::Some((index, cup));
                }
            }
        }
    }

    best_candidate_below.or(best_candidate_above).unwrap()
}

#[derive(Debug, Serialize, Deserialize)]
struct GameState {
    cups: VecDeque<i64>,
    current_cup_index: usize,
}

fn play_game_saveable(state: GameState, moves: usize) -> GameState {
    let rotated_cups: VecDeque<_> = (0..state.cups.len())
        .map(|i| state.cups[(state.current_cup_index + i) % state.cups.len()])
        .collect();

    let (final_cups, current_cup_index) = play_game_naively(&rotated_cups, moves);

    GameState {
        cups: final_cups,
        current_cup_index,
    }
}

fn play_game_naively(initial_cups: &VecDeque<i64>, moves: usize) -> (VecDeque<i64>, usize) {
    let min_cup_number = *initial_cups.iter().min().unwrap();
    let max_cup_number = *initial_cups.iter().max().unwrap();

    let mut cups = initial_cups.clone();
    let mut current_cup_index: usize = 0;

    for move_num in 1..=moves {
        if move_num % 25000 == 0 {
            println!("playing move {}", move_num);
        }

        let current_cup = cups[current_cup_index];

        // println!("{:?} {}=({})", cups, current_cup_index, current_cup);

        let mut next_cup_index = if current_cup_index + 1 >= cups.len() {
            0
        } else {
            current_cup_index + 1
        };
        let mut taken_cups: [i64; 3] = [0, 0, 0];

        for value in taken_cups.iter_mut() {
            if next_cup_index < current_cup_index {
                current_cup_index -= 1;
            }
            *value = cups.remove(next_cup_index).unwrap();
            next_cup_index %= cups.len();
        }

        assert_eq!(current_cup, cups[current_cup_index]);

        let target_cup_number = if current_cup - 1 < min_cup_number {
            max_cup_number
        } else {
            current_cup - 1
        };
        let (destination_index, _destination_cup) =
            find_destination_cup_index(&cups, target_cup_number);

        // println!("  dest: {}, cup {}", destination_index, _destination_cup);

        let next_destination_index = (destination_index + 1) % cups.len();
        for taken_cup in taken_cups.iter().copied().rev() {
            cups.insert(next_destination_index, taken_cup);
        }
        assert_eq!(cups.len(), initial_cups.len());
        if next_destination_index <= current_cup_index {
            current_cup_index += 3;
        }
        assert_eq!(current_cup, cups[current_cup_index]);

        current_cup_index = (current_cup_index + 1) % initial_cups.len();
    }

    (cups, current_cup_index)
}

fn solve_part1(initial_cups: &VecDeque<i64>, moves: usize) -> String {
    let (cups, _) = play_game_naively(initial_cups, moves);

    let one_cup_index = cups
        .iter()
        .enumerate()
        .filter_map(|(index, &cup)| if cup == 1 { Some(index) } else { None })
        .next()
        .unwrap();

    let cups_in_order: Vec<String> = (1..cups.len())
        .map(|offset| cups[(one_cup_index + offset) % cups.len()].to_string())
        .collect();

    cups_in_order.join("")
}

fn solve_part2(initial_cups: &VecDeque<i64>, moves: usize) -> i64 {
    let max_cup = initial_cups.iter().copied().max().unwrap();
    let all_initial_cups: VecDeque<_> = initial_cups
        .iter()
        .copied()
        .chain(max_cup + 1..=1000000)
        .collect();

    let (final_cups, _) = play_game_naively(&all_initial_cups, moves);

    let one_cup_index = final_cups
        .iter()
        .enumerate()
        .filter_map(|(index, &cup)| if cup == 1 { Some(index) } else { None })
        .next()
        .unwrap();

    let next_two_cups_in_order: Vec<i64> = (1..final_cups.len())
        .map(|offset| final_cups[(one_cup_index + offset) % final_cups.len()])
        .take(2)
        .collect();

    println!("two cups: {:?}", next_two_cups_in_order);
    next_two_cups_in_order[0] * next_two_cups_in_order[1]
}

fn main2() {
    let input_path =
        "/mnt/c/Users/predrag/Dropbox/Documents/Code/advent-of-code-2020/day23/input.txt";

    let fragment_file_stem =
        "/mnt/c/Users/predrag/Dropbox/Documents/Code/advent-of-code-2020/day23/".to_owned();

    let total_rounds = 10000000usize;
    let rounds_per_fragment = 250000usize;
    assert!(total_rounds % rounds_per_fragment == 0);
    let stopping_points: Vec<usize> = (1..total_rounds / rounds_per_fragment)
        .map(|x| x * rounds_per_fragment)
        .rev()
        .collect();

    let final_fragment = fragment_file_stem.clone() + &total_rounds.to_string() + ".json";
    if Path::new(&final_fragment).exists() {
        let contents = fs::read_to_string(final_fragment).unwrap();
        let state: GameState = serde_json::from_str(&contents).unwrap();

        let one_cup_index = state
            .cups
            .iter()
            .enumerate()
            .filter_map(|(index, &cup)| if cup == 1 { Some(index) } else { None })
            .next()
            .unwrap();

        let next_two_cups_in_order: Vec<i64> = (1..state.cups.len())
            .map(|offset| state.cups[(one_cup_index + offset) % state.cups.len()])
            .take(2)
            .collect();

        println!(
            "{} * {} = {}",
            next_two_cups_in_order[0],
            next_two_cups_in_order[1],
            next_two_cups_in_order[0] * next_two_cups_in_order[1]
        );
        return;
    }

    let mut starting_state = None;
    let mut ending_fragment = rounds_per_fragment;
    for stopping_point in stopping_points {
        let current_fragment = fragment_file_stem.clone() + &stopping_point.to_string() + ".json";
        let fragment_path = Path::new(&current_fragment);
        // println!("checking path {}", current_fragment);
        if fragment_path.exists() {
            println!("found path {}", current_fragment);
            let contents = fs::read_to_string(fragment_path).unwrap();
            starting_state = Some(serde_json::from_str(&contents).unwrap());
            ending_fragment = stopping_point + rounds_per_fragment;
            break;
        }
    }

    if starting_state.is_none() {
        let contents = fs::read_to_string(&input_path).unwrap();
        let lines: Vec<_> = contents.trim().split('\n').collect();
        assert_eq!(lines.len(), 1);
        let initial_cups: VecDeque<i64> = lines[0]
            .chars()
            .map(|x| x.to_string().parse().unwrap())
            .collect();

        let max_cup = initial_cups.iter().copied().max().unwrap();
        let all_initial_cups: VecDeque<_> = initial_cups
            .iter()
            .copied()
            .chain(max_cup + 1..=1000000)
            .collect();

        starting_state = Some(GameState {
            cups: all_initial_cups,
            current_cup_index: 0,
        });
    }

    let final_state = play_game_saveable(starting_state.unwrap(), rounds_per_fragment);
    let target_fragment_path = fragment_file_stem.clone() + &ending_fragment.to_string() + ".json";

    println!("done until round {}", ending_fragment);

    serde_json::to_writer(&File::create(target_fragment_path).unwrap(), &final_state).unwrap();
}
