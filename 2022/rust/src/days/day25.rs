pub fn solve(part: u32, input: &Vec<&str>, sample: &Vec<&str>) -> i64 {
    assert_eq!(part1(&sample), 4890, "Error, sample problem is not solved");
    //assert_eq!(part2(&sample), 0, "Error, sample problem is not solved");

    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => panic!("Illegal value as input. Valid inputs are 1 and 2, recieved {}", part),
    }
}

fn part2(data: &Vec<&str>) -> i64 {
    0
}

fn parse_snafu(snafu: &str) -> i64 {
    let mut total = 0;
    let mut mult = 1;
    for digit in snafu.chars().rev() {
        total += mult * match digit {
            '0'..='2' => digit.to_digit(10).unwrap() as i64,
            '-'       => -1,
            '='       => -2,
            _         => panic!(),
        };
        mult *= 5;
    }
    total
}

fn to_snafu(value: i64) -> String {
    let mut val: i128 = value as i128;
    let mut base_5: i128 = 0;

    for i in (0..20).rev() {
        let power = 5_i128.pow(i);
        let res = val/power;

        if res > 0 {
            base_5 += res*10_i128.pow(i);
            val = val%power;
        }
    }

    let mut snafu = vec![];
    loop {
        let current = base_5 % 10;
        match current {
            0..=2 => snafu.push(char::from_digit(current as u32, 10).unwrap()),
            3 => {base_5 += 10; snafu.push('=');},
            4 => {base_5 += 10; snafu.push('-');},
            _ => (),
        };

        base_5 /= 10;
        if base_5 == 0 {
            break;
        }
    }
    snafu.reverse();

    String::from_iter(snafu)
}

fn part1(data: &Vec<&str>) -> i64 {
    let total = data.iter().map(|s| parse_snafu(s)).sum();

    println!("String result: {}", to_snafu(total));
    total
}

