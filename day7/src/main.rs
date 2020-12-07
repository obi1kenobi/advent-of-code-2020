use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let contents = fs::read_to_string(
        "/mnt/c/Users/predrag/Dropbox/Documents/Code/advent-of-code-2020/day7/input.txt",
    )
    .unwrap();

    let lines: Vec<_> = contents.trim().split("\n").collect();

    let edges: HashMap<&str, Vec<(usize, &str)>> = parse_rules(lines);

    println!("{}", solve_part1(&edges));
    println!("{}", solve_part2(&edges));
}

fn parse_rules(rules: Vec<&str>) -> HashMap<&str, Vec<(usize, &str)>> {
    let mut edges: HashMap<&str, Vec<(usize, &str)>> = HashMap::new();

    for rule in rules.iter() {
        let components: Vec<_> = rule.split(" contain ").map(|x| x.trim()).collect();
        assert!(components.len() == 2);

        let source = components[0].strip_suffix("bags").unwrap().trim();
        let sinks: Vec<_> = components[1]
            .strip_suffix(".")
            .unwrap()
            .split(", ")
            .map(|x| x.trim())
            .collect();

        let mut edge_targets: Vec<(usize, &str)> = Vec::new();
        for sink in sinks {
            if sink == "no other bags" {
                break;
            }

            let data: Vec<_> = sink.splitn(2, " ").collect();
            assert!(data.len() == 2);
            let count: usize = data[0].parse().unwrap();
            let kind = if data[1].ends_with("s") {
                data[1].strip_suffix("bags").unwrap().trim()
            } else {
                data[1].strip_suffix("bag").unwrap().trim()
            };
            edge_targets.push((count, kind));
        }

        edges.insert(source, edge_targets);
    }

    edges
}

fn solve_part1(edges: &HashMap<&str, Vec<(usize, &str)>>) -> usize {
    let original_bag = "shiny gold";

    let mut reversed_edges: HashMap<&str, Vec<&str>> = HashMap::new();
    for (origin, targets) in edges.iter() {
        for (_, target) in targets.iter() {
            reversed_edges
                .entry(target.clone())
                .or_insert(Vec::new())
                .push(origin.clone());
        }
    }

    let mut reachable: usize = 0;
    let mut processed: HashSet<&str> = HashSet::new();
    let mut queued: Vec<&str> = Vec::new();
    processed.insert(&original_bag);
    queued.push(&original_bag);
    loop {
        match queued.pop() {
            None => break,
            Some(bag_type) => {
                for origin in reversed_edges.get(bag_type).unwrap_or(&Vec::new()).iter() {
                    if processed.insert(origin.clone()) {
                        queued.push(&origin);
                        reachable += 1;
                    }
                }
            }
        }
    }

    reachable
}

fn solve_part2(edges: &HashMap<&str, Vec<(usize, &str)>>) -> usize {
    let original_bag = "shiny gold";

    let mut unvisited_edges: HashMap<&str, HashSet<&str>> = edges
        .iter()
        .map(|(key, value)| {
            (
                key.clone(),
                value.iter().map(|(_, target)| target.clone()).collect(),
            )
        })
        .collect();

    let mut reversed_edges: HashMap<&str, Vec<&str>> = HashMap::new();
    for (origin, targets) in edges.iter() {
        for (_, target) in targets.iter() {
            reversed_edges
                .entry(target.clone())
                .or_insert(Vec::new())
                .push(origin.clone());
        }
    }

    let mut toposorted_bags: Vec<&str> = Vec::new();
    let mut contained_bag_count: HashMap<&str, usize> = HashMap::new();
    let mut processed: HashSet<&str> = HashSet::new();
    let mut queued: Vec<&str> = unvisited_edges
        .iter()
        .filter(|(_, targets)| targets.is_empty())
        .map(|(key, _)| key.clone())
        .collect();
    loop {
        match queued.pop() {
            None => break,
            Some(bag_type) => {
                assert!(unvisited_edges[bag_type].is_empty());
                toposorted_bags.push(&bag_type);

                for dependency in reversed_edges.get(bag_type).unwrap_or(&Vec::new()).iter() {
                    unvisited_edges
                        .get_mut(dependency)
                        .unwrap()
                        .remove(bag_type);
                    if unvisited_edges[dependency].is_empty() {
                        queued.push(dependency);
                        processed.insert(dependency);
                    }
                }
            }
        }
    }

    // Ensure no cycles were found.
    assert!(toposorted_bags.len() == edges.len());

    for bag_type in toposorted_bags.iter() {
        contained_bag_count.insert(
            bag_type,
            edges[bag_type]
                .iter()
                .map(|(count, kind)| count * (contained_bag_count[kind] + 1))
                .sum(),
        );
    }

    contained_bag_count[original_bag]
}
