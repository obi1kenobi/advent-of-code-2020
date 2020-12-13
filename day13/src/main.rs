use std::fs;

fn main() {
    let contents = fs::read_to_string(
        "/mnt/c/Users/predrag/Dropbox/Documents/Code/advent-of-code-2020/day13/input.txt",
    )
    .unwrap();

    let lines: Vec<_> = contents.trim().split("\n").collect();

    println!("{}", solve_part2("7,13,x,x,59,x,31,19"));
    println!("{}", solve_part2("17,x,13,19"));
    println!("{}", solve_part2("67,7,59,61"));

    println!("{}", solve_part1(&lines));
    println!("{}", solve_part2(&lines[1]));
}

fn solve_part1(lines: &Vec<&str>) -> i64 {
    let start_time: i64 = lines[0].parse().unwrap();
    let depart_intervals: Vec<i64> = lines[1]
        .split(",")
        .filter(|x| *x != "x")
        .map(|x| x.parse().unwrap())
        .collect();

    let mut current_time = start_time;
    loop {
        for depart_interval in depart_intervals.iter() {
            if current_time % depart_interval == 0 {
                return (current_time - start_time) * depart_interval;
            }
        }
        current_time += 1;
    }
}

fn solve_part2(line: &str) -> i64 {
    let depart_intervals: Vec<i64> = line.split(",").map(|x| x.parse().unwrap_or(-1)).collect();

    let mut sub: i64 = 0;
    let mut current_increment = depart_intervals[0];
    assert!(current_increment != -1);

    for (offset, next_id) in depart_intervals[1..depart_intervals.len()]
        .iter()
        .cloned()
        .enumerate()
    {
        if next_id == -1 {
            continue;
        }
        let mut mult: i64 = 1;
        let minutes_later = ((offset + 1) as i64 + next_id - (sub % next_id)) % next_id;

        loop {
            let remainder = (mult * current_increment) % next_id;
            if minutes_later == remainder {
                sub += mult * current_increment;
                current_increment = lcm(current_increment, next_id);
                break;
            }
            mult += 1;
        }
    }

    current_increment - sub
}

fn gcd(a: i64, b: i64) -> i64 {
    if a < b {
        return gcd(b, a);
    } else {
        let rem = a % b;
        if rem == 0 {
            b
        } else {
            gcd(b, rem)
        }
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    let common = gcd(a, b);
    return a / common * b;
}
