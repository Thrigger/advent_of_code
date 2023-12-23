pub fn solve(part: u32, input: &Vec<&str>, sample: &Vec<&str>) -> i64 {
    assert_eq!(part1(&sample), 0, "Error, sample problem is not solved");
    //assert_eq!(part2(&sample), 0, "Error, sample problem is not solved");

    match part {
    //    1 => part1(&input),
        2 => part2(&input),
        _ => panic!("Illegal value as input. Valid inputs are 1 and 2, recieved {}", part),
    }
}

fn part2(data: &Vec<&str>) -> i64 {
    let mut total = 0;
    //data.iter().map(|i| fuel2(*i)).sum();
    total
}

fn part1(data: &Vec<&str>) -> i64 {
    let mut total = 0;

    let mut map: Vec<Vec<u32>> = vec![];

    for line in data {
        map.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect());
    }

    for line in map {
        println!("{:?}", line);
    }
    
    total
}

