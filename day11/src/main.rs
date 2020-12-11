use std::fs;

fn main() {
    let contents = fs::read_to_string(
        "/mnt/c/Users/predrag/Dropbox/Documents/Code/advent-of-code-2020/day11/input.txt",
    )
    .unwrap();

    let seats: Vec<Vec<char>> = contents
        .trim()
        .split("\n")
        .map(|x| x.chars().collect())
        .collect();

    println!("{}", solve_part1(&seats));
    println!("{}", solve_part2(&seats));
}

fn solve_part1(seats: &Vec<Vec<char>>) -> i64 {
    let mut local_seats = seats.clone();
    loop {
        let new_seats = simulate_step_part1(&local_seats);

        if new_seats == local_seats {
            break;
        }
        local_seats = new_seats;
    }

    let result = local_seats
        .iter()
        .map(|x| x.iter().map(|y| if *y == '#' { 1 } else { 0 }).sum::<i64>())
        .sum();

    result
}

fn simulate_step_part1(seats: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_seats = seats.clone();

    let row_max = seats.len();
    let col_max = seats[0].len();

    for row in 0..row_max {
        for col in 0..col_max {
            if seats[row][col] == '.' {
                continue;
            }

            let dx = [0, 1, 1, 1, 0, -1, -1, -1];
            let dy = [1, 1, 0, -1, -1, -1, 0, 1];

            let mut filled_neighbors: usize = 0;
            for neighbor_idx in 0..dx.len() {
                let new_x = (row as i64) + dx[neighbor_idx];
                let new_y = (col as i64) + dy[neighbor_idx];

                if new_x >= 0 && new_x < row_max as i64 && new_y >= 0 && new_y < col_max as i64 {
                    let neighbor_row = new_x as usize;
                    let neighbor_col = new_y as usize;

                    filled_neighbors += match seats[neighbor_row][neighbor_col] {
                        'L' | '.' => 0,
                        '#' => 1,
                        _ => unreachable!(),
                    }
                }
            }

            if seats[row][col] == '#' {
                if filled_neighbors >= 4 {
                    new_seats[row][col] = 'L';
                }
            } else if seats[row][col] == 'L' {
                if filled_neighbors == 0 {
                    new_seats[row][col] = '#'
                }
            }
        }
    }

    new_seats
}

fn solve_part2(seats: &Vec<Vec<char>>) -> i64 {
    let mut local_seats = seats.clone();
    loop {
        let new_seats = simulate_step_part2(&local_seats);
        if new_seats == local_seats {
            break;
        }
        local_seats = new_seats;
    }

    let result = local_seats
        .iter()
        .map(|x| x.iter().map(|y| if *y == '#' { 1 } else { 0 }).sum::<i64>())
        .sum();

    result
}

fn simulate_step_part2(seats: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_seats = seats.clone();

    let row_max = seats.len();
    let col_max = seats[0].len();

    for row in 0..row_max {
        for col in 0..col_max {
            if seats[row][col] == '.' {
                continue;
            }

            let dx = [0, 1, 1, 1, 0, -1, -1, -1];
            let dy = [1, 1, 0, -1, -1, -1, 0, 1];

            let mut filled_neighbors: usize = 0;
            for neighbor_idx in 0..dx.len() {
                let mut new_x = row as i64;
                let mut new_y = col as i64;
                loop {
                    new_x += dx[neighbor_idx];
                    new_y += dy[neighbor_idx];

                    if new_x >= 0 && new_x < row_max as i64 && new_y >= 0 && new_y < col_max as i64
                    {
                        let neighbor_row = new_x as usize;
                        let neighbor_col = new_y as usize;

                        match seats[neighbor_row][neighbor_col] {
                            'L' => {
                                break;
                            }
                            '#' => {
                                filled_neighbors += 1;
                                break;
                            }
                            '.' => {
                                continue;
                            }
                            _ => unreachable!(),
                        }
                    } else {
                        break;
                    }
                }
            }

            if seats[row][col] == '#' {
                if filled_neighbors >= 5 {
                    new_seats[row][col] = 'L';
                }
            } else if seats[row][col] == 'L' {
                if filled_neighbors == 0 {
                    new_seats[row][col] = '#'
                }
            }
        }
    }

    new_seats
}
