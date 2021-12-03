pub fn solve(part: u32, input: &Vec<&str>, sample: &Vec<&str>) -> i64 {
    assert_eq!(part1(&sample), 150, "Error, sample problem is not solved");
    assert_eq!(part2(&sample), 900, "Error, sample problem is not solved");
    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => panic!("Illegal value as input. Valid inputs are 1 and 2, recieved {}", part),
    }
}

fn part2(data: &Vec<&str>) -> i64 {
    let mut hor = 0;
    let mut dep = 0;
    let mut aim = 0;

    for inst in data {
        let line: Vec<&str> = inst.split(" ").collect();
        let value = line[1].parse::<i64>().unwrap();
        match line[0] {
            "forward" => {hor += value; dep += aim * value},
            "down" =>    aim += value,
            "up" =>      aim -= value,
            _ => (),
        }
    }

    hor*dep
}

fn part1(data: &Vec<&str>) -> i64 {
    let mut hor = 0;
    let mut dep = 0;

    for inst in data {
        let line: Vec<&str> = inst.split(" ").collect();
        match line[0] {
            "forward" => hor += line[1].parse::<i64>().unwrap(),
            "down" =>    dep += line[1].parse::<i64>().unwrap(),
            "up" =>      dep -= line[1].parse::<i64>().unwrap(),
            _ => (),
        }
    }

    hor*dep
}

