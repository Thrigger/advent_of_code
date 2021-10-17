use std::collections::HashMap;

pub fn solve(part: u32, input: &str, sample: &str) -> i64 {
    assert_eq!(part1(&sample), 4, "Error, sample problem is not solved");
    assert_eq!(part2(&sample), 32, "Error, sample problem is not solved");
    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => panic!("Illegal value as input. Valid inputs are 1 and 2, recieved {}", part),
    }
}

fn contains_gold_bag(rules: &HashMap <&str, &str>, mut known: &mut HashMap<String, i64>, name: &str) -> i64 {
    let mut number_of_gold_bags = 0;
    /* Load rules regarding current bag */
    let contains = match rules.get(name) {
        Some(s) => s,
        None => "no other bags",
    };

    /* Check if this subproblem has been solved allready */
    match known.get(name) {
        Some(u) => return *u,
        None => (),
    }

    if name == "shiny gold" {
        known.entry(String::from(name)).or_insert(1);
        return 1
    } else if contains == "no other bags" {
        known.entry(String::from(name)).or_insert(0);
        return 0
    } else {
        /* Try to find if the bags contained can contain a golden bag */
        for bag in contains.split(", ") {
            let number_of_bags: i64 = bag[..1].parse().unwrap();
            let mut name_of_bag = &bag[2..];
            if name_of_bag.contains("bags") {
                name_of_bag = &name_of_bag[..name_of_bag.len()-5];
            } else {
                name_of_bag = &name_of_bag[..name_of_bag.len()-4];
            }

            number_of_gold_bags += number_of_bags *
                                    contains_gold_bag(&rules, &mut known, name_of_bag);
        }
    }

    known.entry(String::from(name)).or_insert(number_of_gold_bags);
    number_of_gold_bags 
}

fn part1(data: &str) -> i64 {
    let mut sum = 0;
    let mut bag_rules: HashMap<&str, &str> = HashMap::new();
    let mut known_sol: HashMap<String, i64> = HashMap::new();
   
    /* Read bag rules from input */
    for line in data.lines() {
        if line == "" {
            // Last line
            break;
        }
        let mut line_iter = line.split(" bags contain ");
        let     name      = line_iter.next().unwrap();
        let mut contains  = line_iter.next().unwrap();
        /* Strip ending dot */
        contains = &contains[..contains.len()-1];

        bag_rules.entry(name).or_insert(contains);
    }

    /* Loop each key in rules and find the solution */
    for bag in bag_rules.keys() {
        if contains_gold_bag(&bag_rules, &mut known_sol, &bag) > 0 {
            sum += 1;
        }
    }

    /* Removing one from sum since the sum includes the golden bag */
    sum-1
}

fn count_total_bags(    rules: &HashMap <&str, &str>, 
                    mut known: &mut HashMap<String, i64>, 
                        name: &str) -> i64 {
    /* Start at 1 since this bag is counted as well */
    let mut total = 1;

    /* Load rules regarding current bag */
    let contains = match rules.get(name) {
        Some(s) => s,
        None => "no other bags",
    };

    /* Check if this subproblem has been solved allready */
    match known.get(name) {
        Some(u) => return *u,
        None => (),
    }

    if contains == "no other bags" {
        known.entry(String::from(name)).or_insert(1);
        return 1
    } else {
        /* Try to find how many bags can be contained in this one. */
        for bag in contains.split(", ") {
            let number_of_bags: i64 = bag[..1].parse().unwrap();
            let mut name_of_bag = &bag[2..];
            if name_of_bag.contains("bags") {
                name_of_bag = &name_of_bag[..name_of_bag.len()-5];
            } else {
                name_of_bag = &name_of_bag[..name_of_bag.len()-4];
            }

            total += number_of_bags * count_total_bags(&rules, &mut known, name_of_bag);
        }
    }

    known.entry(String::from(name)).or_insert(total);
    total
}

fn part2(data: &str) -> i64 {
    let mut bag_rules: HashMap<&str, &str> = HashMap::new();
    let mut known_sol: HashMap<String, i64> = HashMap::new();
   
    /* Read bag rules from input */
    for line in data.lines() {
        if line == "" {
            // Last line
            break;
        }
        let mut line_iter = line.split(" bags contain ");
        let     name      = line_iter.next().unwrap();
        let mut contains  = line_iter.next().unwrap();
        /* Strip ending dot */
        contains = &contains[..contains.len()-1];

        bag_rules.entry(name).or_insert(contains);
    }

    let sum = count_total_bags(&bag_rules, &mut known_sol, "shiny gold");

    /* Removing one from sum since the sum includes the golden bag */
    sum-1
}

