use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let contents = fs::read_to_string(
        "/mnt/c/Users/predrag/Dropbox/Documents/Code/advent-of-code-2020/day16/input.txt",
    )
    .unwrap();

    let groups: Vec<_> = contents.trim().split("\n\n").collect();
    let fields_info = groups[0];
    let your_ticket_info = groups[1];
    let nearby_tickets_info = groups[2];

    let fields: Vec<_> = fields_info.trim().split("\n").collect();
    let field_valid_ranges: Vec<_> = fields
        .iter()
        .map(|&x| x.split(": ").last().unwrap())
        .flat_map(|x| x.split(" or "))
        .collect();
    let field_valid_rules: Vec<(i64, i64)> = field_valid_ranges
        .iter()
        .map(|&x| x.split("-").map(|y| y.parse().unwrap()))
        .map(|mut y| (y.next().unwrap(), y.last().unwrap()))
        .collect();

    let your_ticket_data: Vec<_> = your_ticket_info.trim().split("\n").collect();
    assert!(your_ticket_data[0] == "your ticket:");
    let your_ticket_numbers: Vec<i64> = your_ticket_data[1]
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    let nearby_tickets_data: Vec<_> = nearby_tickets_info.trim().split("\n").collect();
    assert!(nearby_tickets_data[0] == "nearby tickets:");
    let nearby_tickets: Vec<Vec<i64>> = nearby_tickets_data[1..nearby_tickets_data.len()]
        .iter()
        .map(|&ticket| {
            ticket
                .trim()
                .split(",")
                .map(|y| y.parse().unwrap())
                .collect()
        })
        .collect();

    let (part1_soln, mut valid_tickets) = solve_part1(&field_valid_rules, &nearby_tickets);
    println!("{}", part1_soln);

    valid_tickets.push(your_ticket_numbers.clone());
    println!(
        "{}",
        solve_part2(&fields, &valid_tickets, &your_ticket_numbers)
    );
}

fn solve_part1(
    field_valid_rules: &Vec<(i64, i64)>,
    nearby_tickets: &Vec<Vec<i64>>,
) -> (i64, Vec<Vec<i64>>) {
    let mut result: i64 = 0;
    let mut valid_tickets: Vec<Vec<i64>> = Vec::new();

    for nearby_ticket in nearby_tickets {
        let mut is_valid_ticket = true;
        for value in nearby_ticket {
            let is_valid_value: bool = field_valid_rules
                .iter()
                .filter(|(lower, upper)| lower <= value && upper >= value)
                .next()
                .is_some();

            if !is_valid_value {
                is_valid_ticket = false;
                result += value;
            }
        }

        if is_valid_ticket {
            valid_tickets.push(nearby_ticket.clone());
        }
    }

    (result, valid_tickets)
}

fn solve_part2(fields: &Vec<&str>, valid_tickets: &Vec<Vec<i64>>, your_ticket: &Vec<i64>) -> i64 {
    let field_names: Vec<_> = fields
        .iter()
        .map(|&x| x.split(": ").next().unwrap())
        .collect();
    let field_rule_elements: Vec<Vec<&str>> = fields
        .iter()
        .map(|&x| x.split(": ").last().unwrap().split(" or ").collect())
        .collect();
    let field_rule_ranges: Vec<Vec<(i64, i64)>> = field_rule_elements
        .iter()
        .map(|ranges| {
            ranges
                .iter()
                .map(|&range| range.split("-").map(|value| value.parse::<i64>().unwrap()))
                .map(|mut iter| (iter.next().unwrap(), iter.next().unwrap()))
                .collect()
        })
        .collect();

    // Figure out all the possible field indexes for each value on our ticket.
    // We start off by having all field indexes be possible for each value, and then validate
    // each ticket's values against all fields, discarding field indexes whose rules don't match.
    let mut possible_ticket_index_to_field_index: Vec<HashSet<usize>> = Vec::new();
    for _ in 0..your_ticket.len() {
        possible_ticket_index_to_field_index.push((0..fields.len()).collect());
    }

    for ticket in valid_tickets {
        for (ticket_index, ticket_value) in ticket.iter().enumerate() {
            let valid_mappings: HashSet<_> = field_rule_ranges
                .iter()
                .enumerate()
                .filter(|(_, rule)| {
                    rule.iter()
                        .filter(|(lower, upper)| lower <= ticket_value && upper >= ticket_value)
                        .next()
                        .is_some()
                })
                .map(|(rule_index, _)| rule_index)
                .collect();

            let prior_options = &possible_ticket_index_to_field_index[ticket_index];

            let subsequent_options = prior_options
                .intersection(&valid_mappings)
                .copied()
                .collect();
            possible_ticket_index_to_field_index[ticket_index] = subsequent_options;
        }
    }

    // By this point, we should have enough options ruled out that we can repeatedly make
    // forced choices ("only one option remaining" situations), then eliminate more options
    // as a result, and repeat until we've matched every field index with its spot on the ticket.
    let mut field_index_to_ticket_index: HashMap<usize, usize> = HashMap::new();
    while field_index_to_ticket_index.len() < fields.len() {
        for (ticket_index, possible_field_indexes) in
            possible_ticket_index_to_field_index.iter().enumerate()
        {
            let remaining_field_indexes: HashSet<_> = possible_field_indexes
                .difference(&field_index_to_ticket_index.keys().copied().collect())
                .copied()
                .collect();
            if remaining_field_indexes.len() == 1 {
                field_index_to_ticket_index.insert(
                    remaining_field_indexes.iter().next().unwrap().clone(),
                    ticket_index,
                );
            }
        }
    }

    let mut result: i64 = 1;
    for (field_index, field_name) in field_names.iter().enumerate() {
        if field_name.starts_with("departure") {
            let ticket_index = field_index_to_ticket_index[&field_index];
            result *= your_ticket[ticket_index];
        }
    }
    result
}
