use std::collections::HashMap;

//pub fn solve(part: u32, input: &Vec<&str>, sample: &Vec<&str>) -> i64 {
//pub fn solve(part: u32, input: &Vec<i64>, sample: &Vec<i64>) -> i64 {
pub fn solve(part: u32, input: &str, sample: &str) -> i64 {

    assert_eq!(part1(&sample), 123456, "Error, sample problem is not solved");
    //assert_eq!(part2(&sample), 123456, "Error, sample problem is not solved");
    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => panic!("Illegal value as input. Valid inputs are 1 and 2, recieved {}", part),
    }
}

//fn part1(data: &str) -> i64 {
//fn part1(data: &Vec<&str>) -> i64 {
fn part1(data: &Vec<&i64>) -> i64 {

}

//fn part2(data: &str) -> i64 {
//fn part2(data: &Vec<&str>) -> i64 {
fn part2(data: &Vec<&i64>) -> i64 {

}

