use std::{collections::HashMap, fs};

fn main() {
    let contents = fs::read_to_string(
        "/mnt/c/Users/predrag/Dropbox/Documents/Code/advent-of-code-2020/day14/input.txt",
    )
    .unwrap();

    let lines: Vec<_> = contents.trim().split("\n").collect();

    println!("{}", solve_part1(&lines));
    println!("{}", solve_part2(&lines));
}

fn solve_part1(lines: &Vec<&str>) -> u64 {
    let mut current_mask = u64::MAX;
    let mut current_imprint: u64 = 0;
    let mut memory: HashMap<usize, u64> = HashMap::new();
    for &line in lines.iter() {
        let parts: Vec<&str> = line.split(" = ").collect();
        if parts[0] == "mask" {
            current_mask = u64::MAX;
            current_imprint = 0;
            for (index, element) in parts[1].chars().rev().enumerate() {
                match element {
                    'X' => {}
                    '0' => {
                        current_mask &= !((1 as u64) << index);
                    }
                    '1' => {
                        current_mask &= !((1 as u64) << index);
                        current_imprint |= (1 as u64) << index;
                    }
                    _ => unreachable!(),
                }
            }
        } else {
            let mem_loc: usize = parts[0]
                .strip_prefix("mem[")
                .unwrap()
                .strip_suffix("]")
                .unwrap()
                .parse()
                .unwrap();
            let mem_value: u64 = parts[1].parse().unwrap();

            memory.insert(mem_loc, (mem_value & current_mask) | current_imprint);
        }
    }

    memory.values().sum()
}

fn solve_part2(lines: &Vec<&str>) -> u64 {
    let mut current_mask = usize::MAX;
    let mut current_imprint: usize = 0;
    let mut memory: HashMap<usize, u64> = HashMap::new();
    let mut floating_bit_positions: Vec<usize> = Vec::new();

    for &line in lines.iter() {
        let parts: Vec<&str> = line.split(" = ").collect();
        if parts[0] == "mask" {
            current_mask = usize::MAX;
            current_imprint = 0;
            floating_bit_positions = Vec::new();
            for (index, element) in parts[1].chars().rev().enumerate() {
                match element {
                    'X' => {
                        floating_bit_positions.push(index);
                        current_mask &= !((1 as usize) << index);
                    }
                    '0' => {}
                    '1' => {
                        current_imprint |= (1 as usize) << index;
                    }
                    _ => unreachable!(),
                }
            }
        } else {
            let mem_loc: usize = parts[0]
                .strip_prefix("mem[")
                .unwrap()
                .strip_suffix("]")
                .unwrap()
                .parse()
                .unwrap();
            let mem_value: u64 = parts[1].parse().unwrap();

            let base_masked_loc = (mem_loc & current_mask) | current_imprint;

            for options in 0usize..(1 << floating_bit_positions.len()) {
                let mut floating_imprint = 0usize;
                let options_value = options;
                for (index, position) in floating_bit_positions.iter().enumerate() {
                    let is_on: usize = if options_value & (1 << index) != 0 {
                        1usize
                    } else {
                        0usize
                    };
                    floating_imprint |= is_on << position;
                }

                memory.insert(base_masked_loc | floating_imprint, mem_value);
            }
        }
    }

    memory.values().sum()
}
