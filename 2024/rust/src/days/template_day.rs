pub fn solve(part: u32, input: &Vec<&str>, sample: &Vec<&str>) -> i64 {
//pub fn solve(part: u32, input: &Vec<i64>, sample: &Vec<i64>) -> i64 {
    assert_eq!(part1(&sample), 0, "Error, sample problem is not solved");
    //assert_eq!(part2(&sample), 0, "Error, sample problem is not solved");

    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => panic!("Illegal value as input. Valid inputs are 1 and 2, recieved {}", part),
    }
}

fn part2(data: &Vec<&str>) -> i64 {
//fn part2(data: &Vec<i64>) -> i64 {
    let mut total = 0;
    //data.iter().map(|i| fuel2(*i)).sum();
    total
}

fn part1(data: &Vec<&str>) -> i64 {
//fn part1(data: &Vec<i64>) -> i64 {
    let mut total = 0;

    for line in data {
        // parse each line of each as a int. groups of strings -> ints
        //let ints: Vec<i64> = line.lines().filter_map(|s| match s.parse::<i64>() {
        //    Ok(i) => Some(i),
        //    _     => None,
        //}).collect();
        println!("{}", line);
    }
    
    total
}

