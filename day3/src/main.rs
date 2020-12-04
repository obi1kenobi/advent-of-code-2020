use std::fs;

fn main() {
    let contents = fs::read_to_string(
        "/mnt/c/Users/predrag/Dropbox/Documents/Code/advent-of-code-2020/day3/input.txt",
    )
    .unwrap();

    let map: Vec<Vec<char>> = contents
        .trim()
        .split("\n")
        .map(|x| x.chars().collect())
        .collect();

    println!("{}", solve(&map, 1, 3));

    let slope_1_1 = solve(&map, 1, 1);
    let slope_1_3 = solve(&map, 1, 3);
    let slope_1_5 = solve(&map, 1, 5);
    let slope_1_7 = solve(&map, 1, 7);
    let slope_2_1 = solve(&map, 2, 1);
    println!(
        "{} * {} * {} * {} * {} = {}",
        slope_1_1,
        slope_1_3,
        slope_1_5,
        slope_1_7,
        slope_2_1,
        slope_1_1 * slope_1_3 * slope_1_5 * slope_1_7 * slope_2_1,
    )
}

fn solve(map: &Vec<Vec<char>>, dx: usize, dy: usize) -> usize {
    let start_x: usize = 0;
    let start_y: usize = 0;
    let max_x = map.len();
    let max_y: usize = map[0].len();

    let mut encountered_trees: usize = 0;
    let mut current_x = start_x;
    let mut current_y = start_y;

    while current_x < max_x {
        if map[current_x][current_y] == '#' {
            encountered_trees += 1;
        }

        current_x += dx;
        current_y += dy;
        while current_y >= max_y {
            current_y -= max_y;
        }
    }

    encountered_trees
}
