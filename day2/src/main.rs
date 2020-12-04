use std::fs;

fn main() {
    let contents = fs::read_to_string(
        "/mnt/c/Users/predrag/Dropbox/Documents/Code/advent-of-code-2020/day2/input.txt",
    )
    .unwrap();

    solve_part1(&contents);
    solve_part2(&contents);
}

fn solve_part1(contents: &String) {
    let mut valid_passwords = 0;
    for line in contents.trim().split("\n") {
        let components: Vec<&str> = line.split(":").map(|x| x.trim()).collect();
        assert!(components.len() == 2);
        let policy = components[0];
        let password = components[1];

        let elements: Vec<&str> = policy.split(" ").collect();
        assert!(elements.len() == 2);
        let range_spec = elements[0];
        let character = elements[1].chars().next().unwrap();

        let range: Vec<i32> = range_spec.split("-").map(|x| x.parse().unwrap()).collect();
        assert!(range.len() == 2);
        assert!(range[0] <= range[1]);

        let occurrences: i32 = password.chars().map(|c| (c == character) as i32).sum();
        if occurrences >= range[0] && occurrences <= range[1] {
            valid_passwords += 1;
        }
    }

    println!("{}", valid_passwords);
}

fn solve_part2(contents: &String) {
    let mut valid_passwords = 0;
    for line in contents.trim().split("\n") {
        let components: Vec<&str> = line.split(":").map(|x| x.trim()).collect();
        assert!(components.len() == 2);
        let policy = components[0];
        let password = components[1];

        let elements: Vec<&str> = policy.split(" ").collect();
        assert!(elements.len() == 2);
        let positions_spec = elements[0];
        let character_spec = elements[1].chars().next().unwrap();

        let positions: Vec<i32> = positions_spec
            .split("-")
            .map(|x| x.parse().unwrap())
            .collect();
        assert!(positions.len() == 2);

        let mut has_invalid_chars = false;
        let mut has_required_char = false;
        for (index, password_char) in password.chars().enumerate() {
            let one_based_index = (index + 1) as i32;
            if one_based_index == positions[0] || one_based_index == positions[1] {
                if password_char == character_spec {
                    if has_required_char {
                        has_invalid_chars = true;
                        break;
                    } else {
                        has_required_char = true;
                    }
                }
            }
        }

        if has_required_char && !has_invalid_chars {
            valid_passwords += 1;
        }
    }

    println!("{}", valid_passwords);
}
