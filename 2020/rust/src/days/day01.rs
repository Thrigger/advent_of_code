pub fn solve(part: u32, input: &Vec<&str>, sample: &Vec<&str>) -> i64 {
    assert_eq!(part1(&sample), 514579, "Error, sample problem is not solved");
    assert_eq!(part2(&sample), 241861950, "Error, sample problem is not solved");
    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => panic!("Illegal value as input. Valid inputs are 1 and 2, recieved {}", part),
    }
}

fn find_2_with_sum (sum: i64, data: &Vec<i64>, start_index: usize) -> i64 {
    let mut solution = 0;
    for i in start_index..data.len() {
        for j in i..data.len() {
            if data[i] + data[j] == sum {
                solution = data[i] * data[j];
                break;
            }
        }
        if solution != 0 {
            break;
        }
    }
    solution
}

fn part1(expenses: &Vec<&str>) -> i64 {
    let data: Vec<i64> = expenses.iter().filter_map(|s| s.parse().ok()).collect();

    find_2_with_sum(2020, &data, 0)
}

fn part2(expenses: &Vec<&str>) -> i64 {
    let data: Vec<i64> = expenses.iter().filter_map(|s| s.parse().ok()).collect();
    let mut solution = 0;

    for i in 0..data.len() {
        let remaining_sum = 2020 - data[i];
        solution = find_2_with_sum(remaining_sum, &data, i);
        if solution != 0 {
            solution = solution * data[i];
            break;
        }
    }
    solution
}



