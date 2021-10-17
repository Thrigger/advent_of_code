pub fn solve(part: u32, input: &Vec<&str>, sample: &Vec<&str>) -> i64 {
    assert_eq!(part1(&sample), 2, "Error, sample problem is not solved");
    assert_eq!(part2(&sample), 1, "Error, sample problem is not solved");
    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => panic!("Illegal value as input. Valid inputs are 1 and 2, recieved {}", part),
    }
}

fn fix_input(input: &str) -> (usize, usize, char, &str) {
    /* Example input 1-3 a: abcde */
    let input_split: Vec<&str> = input.split(' ').collect();
    let min_max: Vec<&str> = input_split[0].split('-').collect();
    let min: usize = min_max[0].parse().unwrap();
    let max: usize = min_max[1].parse().unwrap();
    let key: char = input_split[1].chars().nth(0).unwrap();
    let password: &str = input_split[2];

    (min, max, key, password)
}

fn part1(data: &Vec<&str>) -> i64 {
    let mut solution = 0;

    for i in 0..data.len() {
        let (min, max, key, password) = fix_input(&data[i]);
        let mut occurance = 0;
        for each_char in password.chars() {
            if each_char == key {
                occurance += 1;
            }
        }
        if occurance >= min && occurance <= max {
            solution += 1;
        }
    }
    solution
}

fn part2(data: &Vec<&str>) -> i64 {
    let mut solution = 0;

    for i in 0..data.len() {
        let (min, max, key, password) = fix_input(&data[i]);
        let password: Vec<char> = password.chars().collect();
        if (password[min-1] == key && password[max-1] != key) ||
            (password[min-1] != key && password[max-1] == key) {
            solution += 1;
        }
    }
    solution
}

