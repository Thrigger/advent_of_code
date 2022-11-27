pub fn solve(part: u32, input: &Vec<i64>, sample: &Vec<i64>) -> i64 {
    assert_eq!(part1(&sample), 34241, "Error, sample problem is not solved");
    assert_eq!(part2(&sample), 51316, "Error, sample problem is not solved");

    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => panic!("Illegal value as input. Valid inputs are 1 and 2, recieved {}", part),
    }
}

fn fuel2(mass: i64) -> i64 {
    let mut fuel = mass/3 - 2;
    if fuel >= 9 {
        fuel += fuel2(fuel)
    }
    fuel
}

fn part2(data: &Vec<i64>) -> i64 {
    data.iter().map(|i| fuel2(*i)).sum()
}

fn part1(data: &Vec<i64>) -> i64 {
    data.iter().map(|i| i/3-2).sum()
}

