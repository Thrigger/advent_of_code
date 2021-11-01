extern crate thrigger_support;

use std::collections::HashMap;
use thrigger_support::index_of::IndexOf;

pub fn solve(part: u32, input: &str, sample: &str) -> i64 {

    assert_eq!(part1(&sample), 436, "Error, sample problem is not solved");
    assert_eq!(part2(&sample), 175594, "Error, sample problem is not solved");
    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => panic!("Illegal value as input. Valid inputs are 1 and 2, recieved {}", part),
    }
}

fn part1(data: &str) -> i64 {
    let mut input = data.lines().next().unwrap().split(",")
        .filter_map(|i| match i.parse::<i64>() {
            Ok(i) => Some(i),
            Err(_) => None})
        .collect::<Vec<i64>>();

    let mut i = input.len(); 
    while i < 2020 {
        match input[..i-1].index_of_reversed(input[i-1]){
            Some(j) => input.push((i - 1 - j) as i64),
            None => input.push(0),
        };
        i += 1;
    }
    input[2019]
}

fn part2(data: &str) -> i64 {
    let input = data.lines().next().unwrap().split(",")
        .filter_map(|i| match i.parse::<i64>() {
            Ok(i) => Some(i),
            Err(_) => None})
        .collect::<Vec<i64>>();

    let mut mem = HashMap::new();
    let mut i = 0; 

    while i < input.len() - 1 {
        mem.insert(input[i], i);
        i += 1;
    }

    let mut next_numb = get_next_numb(&mem, input[i] as usize, i);
    mem.insert(input[i], i);
    i += 1;

    while i < 30000000 {
        let current_numb = next_numb;
        next_numb = get_next_numb(&mem, current_numb, i);
        mem.insert(current_numb as i64, i);

        i += 1;
        if i % 30000000 == 0  {
            return current_numb as i64;
        }
    }
    0
}

fn get_next_numb(mem: &HashMap<i64, usize>, curr: usize, index: usize) -> usize {
    match mem.get(&(curr as i64)) {
        Some(i) => index - *i,
        None => 0,
    }
}

