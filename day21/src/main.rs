use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fs,
};

fn parse_line(line: &str) -> (Vec<&str>, Vec<&str>) {
    let components: Vec<_> = line
        .strip_suffix(")")
        .unwrap()
        .split(" (contains")
        .collect();

    assert_eq!(components.len(), 2);
    let foods = components[0].trim().split(" ").map(|x| x.trim()).collect();
    let allergens = components[1].trim().split(",").map(|x| x.trim()).collect();

    (foods, allergens)
}

fn assemble_initial_allergen_data<'a>(
    data: &Vec<(Vec<&'a str>, Vec<&'a str>)>,
) -> (
    HashMap<&'a str, HashSet<&'a str>>,
    HashMap<&'a str, HashSet<&'a str>>,
) {
    let mut allergen_to_food: HashMap<&str, HashSet<&str>> = HashMap::new();
    for (foods, allergens) in data {
        let mentioned_foods: HashSet<&str> = foods.iter().cloned().collect();
        for &allergen in allergens {
            let current_suspect_foods = allergen_to_food.get(allergen).unwrap_or(&mentioned_foods);
            let remaining_suspect_foods: HashSet<_> = current_suspect_foods
                .intersection(&mentioned_foods)
                .cloned()
                .collect();

            allergen_to_food.insert(allergen, remaining_suspect_foods);
        }
    }

    let mut food_to_possible_allergens = transpose_map_of_sets(&allergen_to_food);
    for (foods, _) in data {
        for &food in foods {
            food_to_possible_allergens
                .entry(food)
                .or_insert(HashSet::new());
        }
    }

    (allergen_to_food, food_to_possible_allergens)
}

fn transpose_map_of_sets<'a>(
    map_of_sets: &HashMap<&'a str, HashSet<&'a str>>,
) -> HashMap<&'a str, HashSet<&'a str>> {
    let mut result = HashMap::new();
    for (&key, values) in map_of_sets.iter() {
        for &value in values {
            result.entry(value).or_insert(HashSet::new()).insert(key);
        }
    }
    result
}

fn solve_part1(
    data: &Vec<(Vec<&str>, Vec<&str>)>,
    food_to_possible_allergens: &HashMap<&str, HashSet<&str>>,
) -> usize {
    let no_allergen_foods: HashSet<_> = food_to_possible_allergens
        .iter()
        .filter_map(|(&food, possible_allergens)| {
            if possible_allergens.is_empty() {
                Some(food)
            } else {
                None
            }
        })
        .collect();

    let mut result = 0usize;
    for (foods, _) in data {
        result += foods
            .iter()
            .filter_map(|&x| -> Option<usize> { no_allergen_foods.get(x).map(|_| 1usize) })
            .sum::<usize>();
    }
    result
}

fn solve_part2(
    allergen_to_food: &HashMap<&str, HashSet<&str>>,
    food_to_allergens: &HashMap<&str, HashSet<&str>>,
) -> String {
    let mut allergen_in_possible_foods = allergen_to_food.clone();
    let mut food_to_possible_allergens: HashMap<&str, HashSet<&str>> = food_to_allergens
        .iter()
        .filter_map(|(&food, allergens)| {
            if !allergens.is_empty() {
                Some((food, allergens.clone()))
            } else {
                None
            }
        })
        .collect();

    let allergen_to_food: BTreeMap<&str, &str> = loop {
        let mut converged = true;
        for (&_allergen, possibilities) in &allergen_in_possible_foods {
            assert!(possibilities.len() > 0);
            if possibilities.len() > 1 {
                converged = false;
                break;
            }
        }

        if converged {
            break allergen_in_possible_foods
                .iter()
                .map(|(&allergen, foods)| (allergen, foods.iter().next().unwrap().clone()))
                .collect();
        }

        for (&allergen, possibilities) in &allergen_in_possible_foods {
            if possibilities.len() == 1 {
                let food_with_allergen = *possibilities.iter().next().unwrap();
                assert!(food_to_possible_allergens
                    .get(food_with_allergen)
                    .unwrap()
                    .contains(allergen));

                food_to_possible_allergens
                    .insert(food_with_allergen, vec![allergen].iter().cloned().collect());
            }
        }

        allergen_in_possible_foods = transpose_map_of_sets(&food_to_possible_allergens);
        for (&food, possibilities) in &food_to_possible_allergens {
            if possibilities.len() == 1 {
                let allergen_of_food = *possibilities.iter().next().unwrap();
                assert!(allergen_in_possible_foods
                    .get(allergen_of_food)
                    .unwrap()
                    .contains(food));

                allergen_in_possible_foods
                    .insert(allergen_of_food, vec![food].iter().cloned().collect());
            }
        }
        food_to_possible_allergens = transpose_map_of_sets(&allergen_in_possible_foods);
    };

    println!("{:?}", allergen_to_food);

    allergen_to_food
        .values()
        .map(|&x| x.to_owned())
        .collect::<Vec<_>>()
        .join(",")
}

fn main() {
    let contents = fs::read_to_string(
        "/mnt/c/Users/predrag/Dropbox/Documents/Code/advent-of-code-2020/day21/input.txt",
    )
    .unwrap();

    let data: Vec<_> = contents.trim().split("\n").map(parse_line).collect();
    let (allergen_to_food, food_to_possible_allergens) = assemble_initial_allergen_data(&data);

    println!("{}", solve_part1(&data, &food_to_possible_allergens));
    println!(
        "{}",
        solve_part2(&allergen_to_food, &food_to_possible_allergens)
    );
}
