use std::{collections::HashMap, fs};

#[derive(Debug, Clone)]
enum Rule {
    // rule_id is always the first usize
    Literal(usize, String),
    Compound(usize, Vec<Vec<usize>>),
    OneOrMore(usize, usize), // rule 8: one or more applications of the rule in the second usize
    MatchingPrefixAndSuffix(usize, usize, usize), // rule 11: matching number of prefix and suffix rules
}

fn parse_rule(rule_text: &str) -> (usize, Rule) {
    let components: Vec<_> = rule_text.split(":").collect();
    let rule_id: usize = components[0].parse().unwrap();
    let rule_content = components[1].trim();

    if rule_content.starts_with("\"") {
        let data = rule_content
            .strip_prefix("\"")
            .unwrap()
            .strip_suffix("\"")
            .unwrap();
        assert!(data.len() == 1);
        (rule_id, Rule::Literal(rule_id, data.to_string()))
    } else {
        let composite_of = rule_content
            .split("|")
            .map(|x| x.trim().split(" ").map(|y| y.parse().unwrap()).collect())
            .collect();
        (rule_id, Rule::Compound(rule_id, composite_of))
    }
}

fn match_rule_with_pattern_prefix(
    rule_to_match: usize,
    rules: &HashMap<usize, Rule>,
    pattern: &str,
) -> Option<String> {
    // Some("") = full match, None = not a match at all
    let current_rule = &rules[&rule_to_match];

    match current_rule {
        Rule::Literal(_, target) => pattern.strip_prefix(target).map(|x| x.to_string()),
        Rule::Compound(_, possibilities) => {
            let mut outcome: Option<String> = None;
            for possibility_set in possibilities {
                let mut rest_of_pattern = pattern.to_string();
                let mut possibility_set_success = true;
                for next_rule in possibility_set {
                    if let Some(remainder) =
                        match_rule_with_pattern_prefix(*next_rule, rules, &rest_of_pattern)
                    {
                        // continue matching
                        rest_of_pattern = remainder;
                    } else {
                        possibility_set_success = false;
                        break;
                    }
                }

                if possibility_set_success {
                    outcome = Some(rest_of_pattern);
                }
            }

            outcome
        }
        _ => unreachable!(),
    }
}

fn solve_part1(rules: &HashMap<usize, Rule>, patterns: &Vec<String>) -> usize {
    patterns
        .iter()
        .map(|pat| {
            if let Some(s) = match_rule_with_pattern_prefix(0, rules, pat) {
                if s == "" {
                    1
                } else {
                    0
                }
            } else {
                0
            }
        })
        .sum()
}

fn match_rule(
    rules: &HashMap<usize, Rule>,
    rule_to_evaluate: &Rule,
    rule_stack: &mut Vec<usize>,
    pattern: &str,
) -> bool {
    match rule_to_evaluate {
        Rule::Literal(_, target) => pattern.strip_prefix(target).map_or(false, |remainder| {
            match_pattern_with_cyclic_rules(rules, rule_stack, remainder)
        }),
        Rule::Compound(_, possibilities) => {
            let mut possibility_sets = possibilities.iter();
            loop {
                let original_stack_len = rule_stack.len();
                match possibility_sets.next() {
                    None => break false,
                    Some(possibility_set) => {
                        rule_stack.extend(possibility_set.iter().rev());

                        let current_stack_len = rule_stack.len();
                        let result = match_pattern_with_cyclic_rules(rules, rule_stack, pattern);
                        assert!(rule_stack.len() == current_stack_len);

                        rule_stack.truncate(original_stack_len);

                        if result {
                            break true;
                        }
                    }
                }
            }
        }
        Rule::OneOrMore(_, repeat_rule_id) => {
            let original_stack_len = rule_stack.len();

            let mut remaining_pattern = pattern.to_owned();
            let outcome = loop {
                if let Some(match_remainder) =
                    match_rule_with_pattern_prefix(*repeat_rule_id, rules, &remaining_pattern)
                {
                    // The rule was a prefix match, we can try to match the current number of rule applications.
                    rule_stack.push(*repeat_rule_id);
                    if match_pattern_with_cyclic_rules(rules, rule_stack, pattern) {
                        break true;
                    }

                    remaining_pattern = match_remainder;
                } else {
                    break false;
                }
            };

            rule_stack.truncate(original_stack_len);

            outcome
        }
        Rule::MatchingPrefixAndSuffix(_, prefix_rule_id, suffix_rule_id) => {
            let original_stack_len = rule_stack.len();

            let mut applications: usize = 1;
            let mut remaining_pattern = pattern.to_owned();
            let outcome = loop {
                if let Some(match_remainder) =
                    match_rule_with_pattern_prefix(*prefix_rule_id, rules, &remaining_pattern)
                {
                    // The rule was a prefix match, we can try to match the current number of rule applications.
                    rule_stack.push(*suffix_rule_id);
                    let stack_length_with_only_suffixes = original_stack_len + applications;
                    assert!(rule_stack.len() == stack_length_with_only_suffixes);

                    rule_stack.extend(std::iter::repeat(*prefix_rule_id).take(applications));

                    if match_pattern_with_cyclic_rules(rules, rule_stack, pattern) {
                        break true;
                    };

                    rule_stack.truncate(stack_length_with_only_suffixes);
                    remaining_pattern = match_remainder;
                    applications += 1;
                } else {
                    break false;
                }
            };

            rule_stack.truncate(original_stack_len);

            outcome
        }
    }
}

fn match_pattern_with_cyclic_rules(
    rules: &HashMap<usize, Rule>,
    rule_stack: &mut Vec<usize>, // evaluate tail first
    pattern: &str,
) -> bool {
    match rule_stack.pop() {
        None => pattern.is_empty(),
        Some(rule_to_evaluate) => {
            let current_rule = &rules[&rule_to_evaluate];

            let result = match_rule(rules, current_rule, rule_stack, pattern);

            rule_stack.push(rule_to_evaluate); // replace rule on stack before returning

            result
        }
    }
}

fn solve_part2(rules: &HashMap<usize, Rule>, patterns: &Vec<String>) -> usize {
    patterns
        .iter()
        .map(|pat| {
            let mut rule_stack = vec![0];
            if match_pattern_with_cyclic_rules(rules, &mut rule_stack, pat) {
                1
            } else {
                0
            }
        })
        .sum()
}

fn parse_file(path: &str) -> (HashMap<usize, Rule>, Vec<String>) {
    let contents = fs::read_to_string(path).unwrap();

    let parts: Vec<_> = contents.trim().split("\n\n").collect();
    let rules: HashMap<usize, Rule> = parts[0].trim().split("\n").map(parse_rule).collect();
    let patterns: Vec<_> = parts[1].trim().split("\n").map(&str::to_owned).collect();

    (rules, patterns)
}

fn main() {
    let (rules, patterns) = parse_file(
        "/mnt/c/Users/predrag/Dropbox/Documents/Code/advent-of-code-2020/day19/input.txt",
    );

    println!("{}", solve_part1(&rules, &patterns));

    let mut cyclic_rules: HashMap<usize, Rule> = rules.clone();
    cyclic_rules.insert(8, Rule::OneOrMore(8, 42));
    cyclic_rules.insert(11, Rule::MatchingPrefixAndSuffix(11, 42, 31));
    println!("{}", solve_part2(&cyclic_rules, &patterns));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_avoid_false_positive_match_part1() {
        let path = "/mnt/c/Users/predrag/Dropbox/Documents/Code/advent-of-code-2020/day19/sample_input5.txt";
        let (mut rules, patterns) = crate::parse_file(path);

        assert!(patterns.len() == 1);

        rules.insert(
            1337,
            crate::Rule::Compound(1337, vec![vec![42, 42, 42, 31]]),
        );

        let result = crate::match_rule_with_pattern_prefix(1337, &rules, &patterns[0]);
        assert!(result.map_or(true, |x| !x.is_empty()));
    }

    #[test]
    fn test_avoid_false_positive_match_part2() {
        let path = "/mnt/c/Users/predrag/Dropbox/Documents/Code/advent-of-code-2020/day19/sample_input5.txt";
        let (mut rules, patterns) = crate::parse_file(path);

        assert!(patterns.len() == 1);

        rules.insert(8, crate::Rule::OneOrMore(8, 42));
        rules.insert(11, crate::Rule::MatchingPrefixAndSuffix(11, 42, 31));

        let mut rule_stack = vec![0];
        assert_eq!(
            false,
            crate::match_pattern_with_cyclic_rules(&rules, &mut rule_stack, &patterns[0])
        );
    }
}
