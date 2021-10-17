use std::collections::HashMap;

pub fn solve(part: u32, input: &str, sample: &str) -> i64 {
    assert_eq!(part1(&sample), 11, "Error, sample problem is not solved");
    assert_eq!(part2(&sample), 6, "Error, sample problem is not solved");
    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => panic!("Illegal value as input. Valid inputs are 1 and 2, recieved {}", part),
    }
}

fn part1(input: &str) -> i64 {
    let mut sum = 0;
    for each_group in input.split("\n\n") {
        if each_group == "" {
            break;
        }
        let all_answers_in_group = each_group.replace("\n", "");
        let ans_count = unique_ans(&all_answers_in_group);

        sum += ans_count;
    }

    sum
}

fn part2(input: &str) -> i64 {
    let mut sum = 0;

    for each_group in input.split("\n\n") {
        if each_group == "" {
            break;
        }
        let ans_count = all_same_ans(&each_group);

        sum += ans_count;
    }

    sum
}

fn all_same_ans(input: &str) -> i64 {
    let mut answers = HashMap::new();
    let mut sum = 0;
    let mut members_in_group = 0;

    for line in input.lines() {
        for each in line.chars() {
            *answers.entry(each).or_insert(0) += 1;
        }
        members_in_group += 1;
    }
    for value in answers.values() {
        if *value == members_in_group {
            sum += 1;
        }
    }

    sum
}

fn unique_ans(input: &String) -> i64 {
    let mut answers = HashMap::new();
    let mut sum = 0;
    
    for each in input.chars() {
        answers.insert(each, true);
    }
    for value in answers.values() {
        if *value {
            sum += 1;
        }
    }
    sum
}

