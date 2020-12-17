use std::{fs, mem};

fn main() {
    let contents = fs::read_to_string(
        "/mnt/c/Users/predrag/Dropbox/Documents/Code/advent-of-code-2020/day17/input.txt",
    )
    .unwrap();

    let cubes: Vec<Vec<char>> = contents
        .trim()
        .split("\n")
        .map(|x| x.chars().collect())
        .collect();

    println!("{}", solve_part1(&cubes));
    println!("{}", solve_part2(&cubes));
}

fn solve_part1(cubes: &Vec<Vec<char>>) -> i64 {
    let num_cycles: usize = 6;
    let start_x = cubes.len();
    let start_y = cubes[0].len();

    let max_bounds = 2 * num_cycles;
    let max_x = start_x + max_bounds;
    let max_y = start_y + max_bounds;
    let max_z = 1 + max_bounds;

    let inner_vec = vec!['.'; max_z];
    let mid_vec: Vec<Vec<char>> = vec![inner_vec.clone(); max_y];

    let mut sim_from: Vec<Vec<Vec<char>>> = vec![mid_vec.clone(); max_x];
    let mut sim_to: Vec<Vec<Vec<char>>> = vec![mid_vec.clone(); max_x];

    for (x, data) in cubes.iter().enumerate() {
        for (y, &c) in data.iter().enumerate() {
            sim_to[x + num_cycles][y + num_cycles][num_cycles] = c;
        }
    }

    for _ in 1..=num_cycles {
        mem::swap(&mut sim_from, &mut sim_to);

        for (x, data_x) in sim_from.iter().enumerate() {
            for (y, data_y) in data_x.iter().enumerate() {
                for (z, &c) in data_y.iter().enumerate() {
                    sim_to[x][y][z] = '.';

                    let mut active_neighbors: usize = 0;

                    for dx in -1..=1 {
                        let new_x = x as i64 + dx;
                        if new_x < 0 || new_x >= max_x as i64 {
                            continue;
                        }

                        for dy in -1..=1 {
                            let new_y = y as i64 + dy;
                            if new_y < 0 || new_y >= max_y as i64 {
                                continue;
                            }

                            for dz in -1..=1 {
                                if dx == 0 && dy == 0 && dz == 0 {
                                    continue;
                                }

                                let new_z = z as i64 + dz;
                                if new_z < 0 || new_z >= max_z as i64 {
                                    continue;
                                }

                                active_neighbors +=
                                    match sim_from[new_x as usize][new_y as usize][new_z as usize] {
                                        '#' => 1,
                                        '.' => 0,
                                        _ => unreachable!(),
                                    }
                            }
                        }
                    }

                    if c == '#' && (active_neighbors == 2 || active_neighbors == 3) {
                        sim_to[x][y][z] = '#';
                    } else if c == '.' && active_neighbors == 3 {
                        sim_to[x][y][z] = '#';
                    }
                }
            }
        }
    }

    let result = sim_to
        .iter()
        .flat_map(|x| {
            x.iter()
                .flat_map(|y| y.iter().map(|&z| if z == '#' { 1 as i64 } else { 0 }))
        })
        .sum();

    result
}

fn solve_part2(cubes: &Vec<Vec<char>>) -> i64 {
    let num_cycles: usize = 6;
    let start_x = cubes.len();
    let start_y = cubes[0].len();

    let max_bounds = 2 * num_cycles;
    let max_x = start_x + max_bounds;
    let max_y = start_y + max_bounds;
    let max_z = 1 + max_bounds;
    let max_w = 1 + max_bounds;

    let inner_vec = vec!['.'; max_w];
    let mid_vec: Vec<Vec<char>> = vec![inner_vec.clone(); max_z];
    let outer_vec: Vec<Vec<Vec<char>>> = vec![mid_vec.clone(); max_y];

    let mut sim_from: Vec<Vec<Vec<Vec<char>>>> = vec![outer_vec.clone(); max_x];
    let mut sim_to: Vec<Vec<Vec<Vec<char>>>> = vec![outer_vec.clone(); max_x];

    for (x, data) in cubes.iter().enumerate() {
        for (y, &c) in data.iter().enumerate() {
            sim_to[x + num_cycles][y + num_cycles][num_cycles][num_cycles] = c;
        }
    }

    for _ in 1..=num_cycles {
        mem::swap(&mut sim_from, &mut sim_to);

        for (x, data_x) in sim_from.iter().enumerate() {
            for (y, data_y) in data_x.iter().enumerate() {
                for (z, data_z) in data_y.iter().enumerate() {
                    for (w, &c) in data_z.iter().enumerate() {
                        sim_to[x][y][z][w] = '.';

                        let mut active_neighbors: usize = 0;

                        for dx in -1..=1 {
                            let new_x = x as i64 + dx;
                            if new_x < 0 || new_x >= max_x as i64 {
                                continue;
                            }

                            for dy in -1..=1 {
                                let new_y = y as i64 + dy;
                                if new_y < 0 || new_y >= max_y as i64 {
                                    continue;
                                }

                                for dz in -1..=1 {
                                    let new_z = z as i64 + dz;
                                    if new_z < 0 || new_z >= max_z as i64 {
                                        continue;
                                    }

                                    for dw in -1..=1 {
                                        let new_w = w as i64 + dw;
                                        if new_w < 0 || new_w >= max_w as i64 {
                                            continue;
                                        }

                                        if dx == 0 && dy == 0 && dz == 0 && dw == 0 {
                                            continue;
                                        }

                                        active_neighbors += match sim_from[new_x as usize]
                                            [new_y as usize]
                                            [new_z as usize]
                                            [new_w as usize]
                                        {
                                            '#' => 1,
                                            '.' => 0,
                                            _ => unreachable!(),
                                        }
                                    }
                                }
                            }
                        }

                        if c == '#' && (active_neighbors == 2 || active_neighbors == 3) {
                            sim_to[x][y][z][w] = '#';
                        } else if c == '.' && active_neighbors == 3 {
                            sim_to[x][y][z][w] = '#';
                        }
                    }
                }
            }
        }
    }

    let result = sim_to
        .iter()
        .flat_map(|x| {
            x.iter().flat_map(|y| {
                y.iter()
                    .flat_map(|z| z.iter().map(|&w| if w == '#' { 1 as i64 } else { 0 }))
            })
        })
        .sum();

    result
}
