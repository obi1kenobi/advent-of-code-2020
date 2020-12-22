use std::{
    collections::{HashSet, VecDeque},
    fs,
};

fn main() {
    let contents = fs::read_to_string(
        "/mnt/c/Users/predrag/Dropbox/Documents/Code/advent-of-code-2020/day22/input.txt",
    )
    .unwrap();

    let players: Vec<Vec<_>> = contents
        .trim()
        .split("\n\n")
        .map(|x| x.trim().split("\n").collect())
        .collect();
    assert_eq!(players[0][0], "Player 1:");
    assert_eq!(players[1][0], "Player 2:");

    let player_decks: Vec<_> = players.iter().map(parse_player).collect();

    println!("{}", solve_part1(&player_decks[0], &player_decks[1]));
    println!("{}", solve_part2(&player_decks[0], &player_decks[1]));
}

fn parse_player(player: &Vec<&str>) -> VecDeque<i64> {
    player[1..player.len()]
        .iter()
        .map(|&x| x.parse().unwrap())
        .collect()
}

fn score_deck(winning_deck: &VecDeque<i64>) -> i64 {
    let mut total_score: i64 = 0;
    for (index, card) in winning_deck.iter().rev().enumerate() {
        total_score += *card * (index + 1) as i64;
    }

    total_score
}

fn solve_part1(first_player: &VecDeque<i64>, second_player: &VecDeque<i64>) -> i64 {
    let mut player_a = first_player.clone();
    let mut player_b = second_player.clone();

    let winning_deck = loop {
        match (player_a.pop_front(), player_b.pop_front()) {
            (Some(a), Some(b)) => {
                if a > b {
                    player_a.push_back(a);
                    player_a.push_back(b);
                } else if a < b {
                    player_b.push_back(b);
                    player_b.push_back(a);
                } else {
                    unreachable!();
                }
            }
            (Some(a), None) => {
                player_a.push_front(a);
                break &player_a;
            }
            (None, Some(b)) => {
                player_b.push_front(b);
                break &player_b;
            }
            (None, None) => unreachable!(),
        }
    };

    score_deck(winning_deck)
}

fn solve_part2(first_player: &VecDeque<i64>, second_player: &VecDeque<i64>) -> i64 {
    let player_a = first_player.clone();
    let player_b = second_player.clone();

    let (_, winning_deck) = play_game(player_a, player_b);

    score_deck(&winning_deck)
}

fn play_game(mut player_a: VecDeque<i64>, mut player_b: VecDeque<i64>) -> (usize, VecDeque<i64>) {
    let mut inf_game_prevention: HashSet<(Vec<i64>, Vec<i64>)> = HashSet::new();

    let (winning_player_id, winning_deck) = loop {
        let player_a_deck: Vec<_> = player_a.iter().cloned().collect();
        let player_b_deck: Vec<_> = player_b.iter().cloned().collect();

        if !inf_game_prevention.insert((player_a_deck, player_b_deck)) {
            break (1usize, player_a);
        }

        match (player_a.pop_front(), player_b.pop_front()) {
            (Some(a), Some(b)) => {
                if player_a.len() as i64 >= a && player_b.len() as i64 >= b {
                    let player_a_subdeck: VecDeque<i64> =
                        player_a.iter().cloned().take(a as usize).collect();
                    let player_b_subdeck: VecDeque<i64> =
                        player_b.iter().cloned().take(b as usize).collect();

                    match play_game(player_a_subdeck, player_b_subdeck) {
                        (1usize, _) => {
                            player_a.push_back(a);
                            player_a.push_back(b);
                        }
                        (2usize, _) => {
                            player_b.push_back(b);
                            player_b.push_back(a);
                        }
                        _ => unreachable!(),
                    }
                } else {
                    // insufficient deck size, play as normal
                    if a > b {
                        player_a.push_back(a);
                        player_a.push_back(b);
                    } else if a < b {
                        player_b.push_back(b);
                        player_b.push_back(a);
                    } else {
                        unreachable!();
                    }
                }
            }
            (Some(a), None) => {
                player_a.push_front(a);
                break (1usize, player_a);
            }
            (None, Some(b)) => {
                player_b.push_front(b);
                break (2usize, player_b);
            }
            (None, None) => unreachable!(),
        }
    };

    (winning_player_id, winning_deck.clone())
}
