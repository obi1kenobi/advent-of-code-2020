use std::fs;

fn main() {
    let contents = fs::read_to_string(
        "/mnt/c/Users/predrag/Dropbox/Documents/Code/advent-of-code-2020/day12/input.txt",
    )
    .unwrap();

    let lines: Vec<_> = contents.trim().split("\n").collect();

    println!("{}", solve_part1(&lines));
    println!("{}", solve_part2(&lines));
}

fn solve_part2(lines: &Vec<&str>) -> i64 {
    let (origin_x, origin_y) = (0 as i64, 0 as i64);
    let mut wayp_x: i64 = -1;
    let mut wayp_y: i64 = 10;
    let mut cur_x = origin_x;
    let mut cur_y = origin_y;

    #[rustfmt::skip]
    let rot_matrices: [[[i64; 2]; 2]; 4] = [
        [
            [1, 0],
            [0, 1],
        ], [
            [0, 1],
            [-1, 0],
        ], [
            [-1, 0],
            [0, -1],
        ], [
            [0, -1],
            [1, 0],
        ]
    ];

    for line in lines.iter() {
        match line.split_at(1) {
            ("F", val) => {
                let dist: i64 = val.parse().unwrap();
                cur_x += wayp_x * dist;
                cur_y += wayp_y * dist;
            }
            ("S", val) => {
                let dist: i64 = val.parse().unwrap();
                wayp_x += dist;
            }
            ("N", val) => {
                let dist: i64 = val.parse().unwrap();
                wayp_x -= dist;
            }
            ("E", val) => {
                let dist: i64 = val.parse().unwrap();
                wayp_y += dist;
            }
            ("W", val) => {
                let dist: i64 = val.parse().unwrap();
                wayp_y -= dist;
            }
            (dir, val) => {
                let orig_ang: i64 = val.parse().unwrap();
                assert!(orig_ang % 90 == 0);

                let rot_idx: usize;
                if dir == "R" {
                    rot_idx = ((orig_ang % 360) / 90) as usize;
                } else if dir == "L" {
                    rot_idx = (((360 - (orig_ang % 360)) % 360) / 90) as usize;
                } else {
                    unreachable!();
                }

                let rot_mat = rot_matrices[rot_idx];
                let new_x = wayp_x * rot_mat[0][0] + wayp_y * rot_mat[0][1];
                let new_y = wayp_x * rot_mat[1][0] + wayp_y * rot_mat[1][1];

                wayp_x = new_x;
                wayp_y = new_y;
            }
        }
    }

    (cur_x - origin_x).abs() + (cur_y - origin_y).abs()
}

fn solve_part1(lines: &Vec<&str>) -> i64 {
    let (origin_x, origin_y) = (0 as i64, 0 as i64);
    let mut cur_x = origin_x;
    let mut cur_y = origin_y;
    let mut facing: usize = 0;

    // east, south, west, north
    let dx: [i64; 4] = [0, 1, 0, -1];
    let dy: [i64; 4] = [1, 0, -1, 0];

    for line in lines.iter() {
        match line.split_at(1) {
            ("F", val) => {
                let dist: i64 = val.parse().unwrap();
                cur_x += dx[facing] * dist;
                cur_y += dy[facing] * dist;
            }
            ("S", val) => {
                let dist: i64 = val.parse().unwrap();
                cur_x += dist;
            }
            ("N", val) => {
                let dist: i64 = val.parse().unwrap();
                cur_x -= dist;
            }
            ("E", val) => {
                let dist: i64 = val.parse().unwrap();
                cur_y += dist;
            }
            ("W", val) => {
                let dist: i64 = val.parse().unwrap();
                cur_y -= dist;
            }
            ("R", val) => {
                let ang: i64 = val.parse().unwrap();
                assert!(ang % 90 == 0);
                facing += (ang / 90) as usize;
                facing %= 4;
            }
            ("L", val) => {
                let ang: i64 = val.parse().unwrap();
                assert!(ang % 90 == 0);
                facing += ((ang * 4) - (ang / 90)) as usize;
                facing %= 4;
            }
            _ => unreachable!(),
        }
    }

    (cur_x - origin_x).abs() + (cur_y - origin_y).abs()
}
