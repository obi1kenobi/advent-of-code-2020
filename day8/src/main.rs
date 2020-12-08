use std::{collections::HashSet, fs};

fn main() {
    let contents = fs::read_to_string(
        "/mnt/c/Users/predrag/Dropbox/Documents/Code/advent-of-code-2020/day8/input.txt",
    )
    .unwrap();

    let lines: Vec<_> = contents.trim().split("\n").collect();

    println!("{}", solve_part1(&lines));
    println!("{}", solve_part2(&lines));
}

fn solve_part1(lines: &Vec<&str>) -> i32 {
    let mut accumulator = 0;
    let mut visited_instructions: HashSet<usize> = HashSet::new();
    let mut instruction_ptr: i32 = 0;

    loop {
        if visited_instructions.contains(&(instruction_ptr as usize)) {
            break;
        } else {
            visited_instructions.insert(instruction_ptr as usize);
            match lines[instruction_ptr as usize].split_at(3) {
                ("nop", _) => instruction_ptr += 1,
                ("acc", value) => {
                    accumulator += value.trim().parse::<i32>().unwrap();
                    instruction_ptr += 1;
                }
                ("jmp", value) => {
                    instruction_ptr += value.trim().parse::<i32>().unwrap();
                }
                _ => unreachable!(),
            }
        }
    }

    accumulator
}

fn solve_part2(lines: &Vec<&str>) -> i32 {
    for i in 0..lines.len() {
        match simulate_part2(lines, i) {
            Some(result) => return result,
            None => continue,
        }
    }
    unreachable!();
}

fn simulate_part2(lines: &Vec<&str>, changed_instr: usize) -> Option<i32> {
    let mut accumulator = 0;
    let mut visited_instructions: HashSet<usize> = HashSet::new();
    let mut instruction_ptr: i32 = 0;

    let mut tweaked_lines: Vec<&str> = lines.clone();
    let original_line = lines[changed_instr];
    let (original_instr, original_arg) = original_line.split_at(3);

    let jmp_variant = "jmp".to_owned() + original_arg;
    let nop_variant = "nop".to_owned() + original_arg;

    match original_instr {
        "nop" => tweaked_lines[changed_instr] = jmp_variant.as_ref(),
        "acc" => return None,
        "jmp" => tweaked_lines[changed_instr] = nop_variant.as_ref(),
        _ => unreachable!(),
    }

    loop {
        if (instruction_ptr as usize) == tweaked_lines.len() {
            break;
        } else if visited_instructions.contains(&(instruction_ptr as usize)) {
            return None;
        } else {
            visited_instructions.insert(instruction_ptr as usize);
            match tweaked_lines[instruction_ptr as usize].split_at(3) {
                ("nop", _) => instruction_ptr += 1,
                ("acc", value) => {
                    accumulator += value.trim().parse::<i32>().unwrap();
                    instruction_ptr += 1;
                }
                ("jmp", value) => {
                    instruction_ptr += value.trim().parse::<i32>().unwrap();
                }
                _ => unreachable!(),
            }
        }
    }

    Some(accumulator)
}
