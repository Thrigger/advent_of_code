use thrigger_support::data_conversion::binary_to_u32;
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
    0
}

fn part1(data: &Vec<&str>) -> i64 {
    let mut input: Vec<Vec<char>> = vec![];
    let mut gam = vec![];
    let mut eps = vec![];

    for row in data {
        input.push(row.chars().collect());
    }

    for i in 0..input[0].len() {
        let mut ones = 0;
        for j in 0..input.len() {
            ones += input[j][i].to_digit(10).unwrap();
        }
        if ones > input.len() as u32/2 {
            gam.push('1');
            eps.push('0');
        } else {
            gam.push('0');
            eps.push('1');
        }
    }

    
    println!("{:?}", gam);
    println!("{:?}", eps);


    0
}

