pub fn solve(part: u32, input: &Vec<i64>, sample: &Vec<i64>) -> i64 {
    assert_eq!(part1(&sample), 7, "Error, sample problem is not solved");
    assert_eq!(part2(&sample), 5, "Error, sample problem is not solved");
    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => panic!("Illegal value as input. Valid inputs are 1 and 2, recieved {}", part),
    }
}

fn part2(data: &Vec<i64>) -> i64 {

    let mut combined = vec![];
    let mut i = 0;
    while i + 2 < data.len() {
        combined.push(data[i] + data[i+1] + data[i+2]);
        i += 1;
    }

    part1(&combined)
}


fn part1(data: &Vec<i64>) -> i64 {
    let mut result = -1;
    let mut last = 0;

    for each in data {
        if *each > last {
            result += 1;
        }
        last = *each;
    }
    result
}

