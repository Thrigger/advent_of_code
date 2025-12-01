pub fn solve(part: u32, input: &Vec<&str>, sample: &Vec<&str>) -> i64 {
    assert_eq!(part1(&sample), 3, "Error, sample problem is not solved");
    assert_eq!(part2(&sample), 6, "Error, sample problem is not solved");

    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => panic!("Illegal value as input. Valid inputs are 1 and 2, recieved {}", part),
    }
}

fn part2(data: &Vec<&str>) -> i64 {
    let mut res = 0;
    let mut current = 50;

    for each in data{
        let c = &each[..1];
        let mut val = each[1..].parse::<i64>().unwrap();
        res += val / 100;
        val = val % 100;

        let old = current;
        if c == "L" {
            current -= val;
        } else {
            current += val;
        }

        if old != 0 && (current <= 0 || current >= 100) {
            res += 1;
        }
        current += 100;
        current %= 100;
    }

    res
}

fn part1(data: &Vec<&str>) -> i64 {
    let mut res = 0;
    let mut current = 50;

    for each in data{
        let c = &each[..1];
        let val = &each[1..].parse::<i64>().unwrap();

        if c == "L" {
            current += 100 - val;
        } else {
            current += val;
        }
        current %= 100;

        if current == 0 {
            res += 1;
        }
    }

    res
}

