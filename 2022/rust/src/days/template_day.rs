pub fn solve(part: u32, input: &Vec<i64>, sample: &Vec<i64>) -> i64 {
    //assert_eq!(part1(&sample), 0, "Error, sample problem is not solved");
    //assert_eq!(part2(&sample), 0, "Error, sample problem is not solved");

    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => panic!("Illegal value as input. Valid inputs are 1 and 2, recieved {}", part),
    
}

fn part2(data: &Vec<&str>) -> i64 {
fn part2(data: &Vec<i64>) -> i64 {
    //data.iter().map(|i| fuel2(*i)).sum();
    0
}

//fn part1(data: &Vec<&str>) -> i64 {
fn part1(data: &Vec<i64>) -> i64 {
    let mut ret_val = 0;

    for each in data {
        //let ints: Vec<i64> = each.lines().filter_map(|s| match s.parse::<i64>() {
        //    Ok(i) => Some(i),
        //    _     => None,
        //}).collect();
        println!("{}", each);
    }
    
    ret_val
}

