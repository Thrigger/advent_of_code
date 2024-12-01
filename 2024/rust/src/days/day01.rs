pub fn solve(part: u32, input: &Vec<&str>, sample: &Vec<&str>) -> i64 {
    assert_eq!(part1(&sample), 11, "Error, sample problem is not solved");
    assert_eq!(part2(&sample), 31, "Error, sample problem is not solved");

    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => panic!("Illegal value as input. Valid inputs are 1 and 2, recieved {}", part),
    }
}

fn get_left_right(data: &Vec<&str>) -> (Vec<usize>, Vec<usize>) {
    let mut left: Vec<usize>  = vec![];
    let mut right: Vec<usize>  = vec![];

    for line in data {
        let parts: Vec<&str> = line.split("   ").collect();

        left.push(parts[0].parse::<usize>().unwrap());
        right.push(parts[1].parse::<usize>().unwrap());
    }
    
    left.sort();
    right.sort();
    (left, right)
}

fn part2(data: &Vec<&str>) -> i64 {
    let mut total = 0;

    let (mut right, mut left) = get_left_right(data);

    for each in left {
        total += each * right.iter().filter(|r| **r == each).count();
    }

    total.try_into().unwrap()
}

fn part1(data: &Vec<&str>) -> i64 {
    let mut total = 0;

    let (mut right, mut left) = get_left_right(data);

    let mut i = 0;
    while i < right.len() {
        if left[i] > right[i] {
            total += left[i] - right[i]
        } else {
            total += right[i] - left[i]
        }
        i += 1;
    }

    total.try_into().unwrap()
}

