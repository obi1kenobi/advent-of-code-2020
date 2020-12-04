use std::{collections::HashSet, fs};

fn main() {
    let contents = fs::read_to_string(
        "/mnt/c/Users/predrag/Dropbox/Documents/Code/advent-of-code-2020/day4/input.txt")
        .unwrap();

    let passports: Vec<Vec<_>> = contents
        .trim()
        .split("\n\n")
        .map(|x| x.split_ascii_whitespace().map(|y| y.trim()).collect())
        .collect();

    println!("{}", solve(&passports));
}


fn solve(passports: &Vec<Vec<&str>>) -> i32 {
    let mut required_fields = HashSet::new();
    required_fields.insert("byr");  // (Birth Year)
    required_fields.insert("iyr");  // (Issue Year)
    required_fields.insert("eyr");  // (Expiration Year)
    required_fields.insert("hgt");  // (Height)
    required_fields.insert("hcl");  // (Hair Color)
    required_fields.insert("ecl");  // (Eye Color)
    required_fields.insert("pid");  // (Passport ID)

    let mut optional_fields = HashSet::new();
    optional_fields.insert("cid");  // (Country ID)

    let allowed_fields: HashSet<_> = required_fields.union(&optional_fields).cloned().collect();

    let allowed_eye_colors: HashSet<_> = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].iter().cloned().collect();

    let mut valid_passports = 0;
    for passport in passports.iter() {
        let mut is_valid = true;
        let mut found_fields: HashSet<&str> = HashSet::new();
        for field_and_data in passport.iter() {
            let values: Vec<_> = field_and_data.split(":").collect();
            assert!(values.len() == 2);

            let field_name = values[0];
            if found_fields.contains(field_name) {
                is_valid = false;
                break;
            } else {
                found_fields.insert(field_name);
            }

            let field_value = values[1];
            is_valid = is_valid && match field_name {
                "byr" => {
                    parse_and_check_bounds(field_value, 1920, 2002)
                }
                "iyr" => {
                    parse_and_check_bounds(field_value, 2010, 2020)
                }
                "eyr" => {
                    parse_and_check_bounds(field_value, 2020, 2030)
                }
                "hgt" => {
                    if field_value.len() <= 3 {
                        false
                    } else {
                        match field_value.split_at(field_value.len() - 2) {
                            (height, "cm") => parse_and_check_bounds(height, 150, 193),
                            (height, "in") => parse_and_check_bounds(height, 59, 76),
                            _ => false,
                        }
                    }
                }
                "hcl" => {
                    if field_value.len() != 7 {
                        false
                    } else {
                        match field_value.split_at(1) {
                            ("#", color) if color.chars().find(|x| !x.is_ascii_hexdigit()).is_none() => true,
                            _ => false,
                        }
                    }
                }
                "ecl" => {
                    allowed_eye_colors.contains(&field_value)
                }
                "pid" => {
                    field_value.len() == 9 && field_value.chars().find(|x| !x.is_ascii_digit()).is_none()
                }
                "cid" => true,
                _ => {
                    false
                }
            };

            if !is_valid {
                break;
            }
        }

        if !is_valid {
            continue;
        }

        let missing_required_fields = required_fields.difference(&found_fields);
        let unrecognized_fields = found_fields.difference(&allowed_fields);
        if missing_required_fields.count() > 0 || unrecognized_fields.count() > 0 {
            is_valid = false;
        }

        if is_valid {
            valid_passports += 1;
        }
    }

    valid_passports
}


fn parse_and_check_bounds(value: &str, lower_inclusive: i32, upper_inclusive: i32) -> bool {
    match value.parse::<i32>() {
        Ok(value) if value >= lower_inclusive && value <= upper_inclusive => true,
        _ => false,
    }
}