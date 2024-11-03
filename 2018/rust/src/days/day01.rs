use std::collections::HashMap;

pub fn solve(part: u32, input: &Vec<i64>, sample: &Vec<i64>) -> i64 {
    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => panic!("Illegal value as input. Valid inputs are 1 and 2, recieved {}", part),
    }
}

fn part2(data: &Vec<i64>) -> i64 {
    let mut sum: i64 = 0;
    let mut seen = HashMap::new();
    seen.insert(sum, true);
    let mut i = 0;
    loop {
        sum += data[i];
        if let Some(v) = seen.get(&sum) {
            break sum;
        }
        seen.insert(sum, true);
        i = (i +1) % data.len();
    }
}

fn part1(data: &Vec<i64>) -> i64 {
    data.iter().sum()
}

