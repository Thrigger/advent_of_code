pub fn solve(part: u32, input: &Vec<&str>, sample: &Vec<&str>) -> i64 {
    assert_eq!(part1(&sample), 24000, "Error, sample problem is not solved");
    assert_eq!(part2(&sample), 45000, "Error, sample problem is not solved");

    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => panic!("Illegal value as input. Valid inputs are 1 and 2, recieved {}", part),
    }
}

fn part2(data: &Vec<&str>) -> i64 {
    let mut elfs = vec![];

    for each in data {
        let ints: Vec<i64> = each.lines().filter_map(|s| match s.parse::<i64>() {
            Ok(i) => Some(i),
            _     => None,
        }).collect();
        let sum: i64 = ints.iter().sum();
        elfs.push(sum);
    }

    elfs.sort();
    elfs.reverse();

    elfs[0] + elfs[1] + elfs[2]
}

fn part1(data: &Vec<&str>) -> i64 {
    let mut ret_val = 0;

    for each in data {
        let ints: Vec<i64> = each.lines().filter_map(|s| match s.parse::<i64>() {
            Ok(i) => Some(i),
            _     => None,
        }).collect();
        let sum: i64 = ints.iter().sum();
        if sum > ret_val {
            ret_val =sum;
        }
    }

    ret_val
}

