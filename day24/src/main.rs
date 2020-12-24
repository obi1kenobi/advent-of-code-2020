use std::{collections::HashSet, fs};

fn main() {
    let contents = fs::read_to_string(
        "/mnt/c/Users/predrag/Dropbox/Documents/Code/advent-of-code-2020/day24/input.txt",
    )
    .unwrap();

    let all_directions: Vec<_> = contents.trim().split('\n').map(parse_directions).collect();

    println!("{}", solve_part1(&all_directions));
    println!("{}", solve_part2(&all_directions));
}

#[derive(Debug)]
enum HexDirection {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

impl HexDirection {
    fn variants() -> &'static [HexDirection] {
        &[
            HexDirection::East,
            HexDirection::SouthEast,
            HexDirection::SouthWest,
            HexDirection::West,
            HexDirection::NorthWest,
            HexDirection::NorthEast,
        ]
    }

    fn to_coordinates(&self) -> (i64, i64) {
        match self {
            HexDirection::East => (1, 0),
            HexDirection::West => (-1, 0),
            HexDirection::SouthEast => (0, 1),
            HexDirection::NorthWest => (0, -1),
            HexDirection::SouthWest => (-1, 1),
            HexDirection::NorthEast => (1, -1),
        }
    }
}

fn parse_directions(line: &str) -> Vec<HexDirection> {
    let mut result = Vec::new();
    let mut remaining_line = line;

    loop {
        let (next_direction, remainder) = parse_next_direction(remaining_line);
        remaining_line = remainder;
        match next_direction {
            None => break,
            Some(direction) => result.push(direction),
        }
    }

    result
}

fn parse_next_direction(line: &str) -> (Option<HexDirection>, &str) {
    let mut char_iter = line.chars();

    match char_iter.next() {
        None => (None, line),
        Some('e') => (Some(HexDirection::East), &line[1..]),
        Some('w') => (Some(HexDirection::West), &line[1..]),
        Some('s') => match char_iter.next() {
            Some('e') => (Some(HexDirection::SouthEast), &line[2..]),
            Some('w') => (Some(HexDirection::SouthWest), &line[2..]),
            _ => unreachable!(),
        },
        Some('n') => match char_iter.next() {
            Some('e') => (Some(HexDirection::NorthEast), &line[2..]),
            Some('w') => (Some(HexDirection::NorthWest), &line[2..]),
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

fn get_coordinates_from_directions(directions: &[HexDirection]) -> (i64, i64) {
    // https://www.redblobgames.com/grids/hexagons/#coordinates-axial
    let mut axial_q: i64 = 0;
    let mut axial_r: i64 = 0;

    for direction in directions {
        let (off_q, off_r) = direction.to_coordinates();

        axial_q += off_q;
        axial_r += off_r;
    }

    (axial_q, axial_r)
}

fn calculate_initial_flipped_tiles(all_directions: &[Vec<HexDirection>]) -> HashSet<(i64, i64)> {
    let mut flipped_tiles: HashSet<(i64, i64)> = HashSet::new();

    for directions in all_directions {
        let flipped_tile = get_coordinates_from_directions(directions);

        if !flipped_tiles.insert(flipped_tile) {
            let removed = flipped_tiles.remove(&flipped_tile);
            assert!(removed);
        }
    }

    flipped_tiles
}

fn solve_part1(all_directions: &[Vec<HexDirection>]) -> usize {
    calculate_initial_flipped_tiles(all_directions).len()
}

fn solve_part2(all_directions: &[Vec<HexDirection>]) -> usize {
    let initial_flips = calculate_initial_flipped_tiles(all_directions);

    let mut currently_black_tiles;
    let mut next_black_tiles = initial_flips;

    let days_to_simulate = 100;
    for _day in 1..=days_to_simulate {
        currently_black_tiles = next_black_tiles.clone();

        let possibly_active_tiles: HashSet<(i64, i64)> = currently_black_tiles
            .iter()
            .flat_map(|&(q, r)| {
                let mut result = vec![(q, r)];
                result.extend(
                    HexDirection::variants()
                        .iter()
                        .map(HexDirection::to_coordinates)
                        .map(|(dq, dr)| (q + dq, r + dr)),
                );
                result
            })
            .collect();

        for (q, r) in possibly_active_tiles {
            let is_black_tile = currently_black_tiles.contains(&(q, r));

            let num_black_neighbors = HexDirection::variants()
                .iter()
                .map(HexDirection::to_coordinates)
                .map(|(dq, dr)| (q + dq, r + dr))
                .filter(|coords| currently_black_tiles.contains(coords))
                .count();

            if is_black_tile {
                if num_black_neighbors == 0 || num_black_neighbors > 2 {
                    let removed = next_black_tiles.remove(&(q, r));
                    assert!(removed);
                }
            } else if num_black_neighbors == 2 {
                let added = next_black_tiles.insert((q, r));
                assert!(added);
            }
        }
    }

    next_black_tiles.len()
}
