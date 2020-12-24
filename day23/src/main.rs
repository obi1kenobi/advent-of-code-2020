use std::{
    collections::{HashMap, VecDeque},
    fs,
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

fn reconstruct_board(
    clockwise_neighbor: &HashMap<i64, i64>,
    current_cup: i64,
    num_cups: usize,
) -> (VecDeque<i64>, usize) {
    let mut current_cup_index: usize = 0;
    let mut final_cups = VecDeque::new();
    let mut starting_cup: i64 = 1;
    for index in 0..num_cups {
        if starting_cup == current_cup {
            current_cup_index = index;
        }
        final_cups.push_back(starting_cup);
        starting_cup = clockwise_neighbor[&starting_cup];
    }

    (final_cups, current_cup_index)
}

fn play_game_naively(initial_cups: &VecDeque<i64>, moves: usize) -> (VecDeque<i64>, usize) {
    let mut clockwise_neighbor: HashMap<i64, i64> = initial_cups
        .iter()
        .copied()
        .enumerate()
        .map(|(index, cup)| (cup, initial_cups[(index + 1) % initial_cups.len()]))
        .collect();

    let min_cup_number = *initial_cups.iter().min().unwrap();
    let max_cup_number = *initial_cups.iter().max().unwrap();

    let mut current_cup = initial_cups[0];

    for _move_num in 1..=moves {
        let taken_cups: [i64; 3] = [
            clockwise_neighbor[&current_cup],
            clockwise_neighbor[&clockwise_neighbor[&current_cup]],
            clockwise_neighbor[&clockwise_neighbor[&clockwise_neighbor[&current_cup]]],
        ];
        let next_cup = clockwise_neighbor[taken_cups.last().unwrap()];
        clockwise_neighbor.insert(current_cup, next_cup);

        let mut possible_target_number = current_cup - 1;
        let destination_cup = loop {
            if possible_target_number < min_cup_number {
                possible_target_number = max_cup_number;
            }
            if !taken_cups.contains(&possible_target_number)
                && clockwise_neighbor.contains_key(&possible_target_number)
            {
                break possible_target_number;
            }
            possible_target_number -= 1;
        };

        let destination_neighbor = clockwise_neighbor[&destination_cup];
        clockwise_neighbor.insert(destination_cup, taken_cups[0]);
        clockwise_neighbor.insert(taken_cups[0], taken_cups[1]);
        clockwise_neighbor.insert(taken_cups[1], taken_cups[2]);
        clockwise_neighbor.insert(taken_cups[2], destination_neighbor);

        current_cup = next_cup;
    }

    reconstruct_board(&clockwise_neighbor, current_cup, initial_cups.len())
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
