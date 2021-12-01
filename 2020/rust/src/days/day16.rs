
pub fn solve(part: u32, input: &Vec<&str>, sample: &Vec<&str>) -> i64 {

    assert_eq!(part1(&sample), 71, "Error, sample problem is not solved");
    //assert_eq!(part2(&sample), 123456, "Error, sample problem is not solved");
    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => panic!("Illegal value as input. Valid inputs are 1 and 2, recieved {}", part),
    }
}

fn part1(data: &Vec<&str>) -> i64 {
    let mut rules: Vec<Rule> = vec![];
    for line in data[0].lines() {
        add_rule(&mut rules, line);

        println!("{:?}", line);
    }

    let mut new_input = vec![];
    counting_error_and_updating_input(&mut rules, data[2], &mut new_input) 
}

fn counting_error_and_updating_input(mut rules: &mut Vec<Rule>, other_pass: &str, mut new_in: &mut Vec<&str>) -> i64 {
    let mut sum = 0;
    let mut lines = other_pass.lines();
    lines.next();
    for line in lines {
        for value in line.split(",") {
            let value_int = value.parse::<i64>().unwrap();
            if !ok_value(&mut rules, value_int) {
                sum += value_int;
            } else {
                //TODO add this as a ok passport
            }
        }
    }
    sum
}

struct Rule {
    name: String,
    a_min: i64,
    a_max: i64,
    b_min: i64,
    b_max: i64,
}

fn ok_value(rules: &mut Vec<Rule>, value: i64) -> bool {
    for rule in rules {
        if value >= rule.a_min && value <= rule.a_max {
            return true;
        } else if value >= rule.b_min && value <= rule.b_max {
            return true;
        }
    }
    false
}

fn add_rule(rules: &mut Vec<Rule>, line: &str) {
    let mut class = line.split(": ");
    let name = class.next().unwrap().to_string();

    let mut values = class.next().unwrap().split(" or ").collect::<Vec<&str>>();
    let intervall = values[0].split("-").map(|i| i.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let a_min = intervall[0];
    let a_max = intervall[1];
    let intervall = values[1].split("-").map(|i| i.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let b_min = intervall[0];
    let b_max = intervall[1];

    rules.push(Rule{name, a_min, a_max, b_min, b_max}); 
}

fn part2(data: &Vec<&str>) -> i64 {

    let mut rules: Vec<Rule> = vec![];
    for line in data[0].lines() {
        add_rule(&mut rules, line);

        println!("{:?}", line);
    }

    let mut new_input = vec![];
    counting_error_and_updating_input(&mut rules, data[2], &mut new_input);


    //TODO add logic for p2
    0
}


