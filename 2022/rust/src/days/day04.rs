pub fn solve(part: u32, input: &Vec<&str>, sample: &Vec<&str>) -> i64 {
    assert_eq!(part1(&sample), 2, "Error, sample problem is not solved");
    assert_eq!(part2(&sample), 4, "Error, sample problem is not solved");

    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => panic!("Illegal value as input. Valid inputs are 1 and 2, recieved {}", part),
    }
}

fn part2(data: &Vec<&str>) -> i64 {
    let mut total = 0;

    for line in data {
        let assignments: Vec<&str> = line.split(",").collect();

        let first: Vec<i64> = assignments[0].split("-").map(|s| s.parse::<i64>().unwrap()).collect();
        let second: Vec<i64> = assignments[1].split("-").map(|s| s.parse::<i64>().unwrap()).collect();
        
        if (first[0] >= second[0] && first[0]<= second[1]) 
        || (first[1] >= second[0] && first[1]<= second[1])
        || (first[0] <= second[0] && first[1]>= second[0])
        || (first[0] <= second[1] && first[1]>= second[1]) {
                total+=1;
        }

    }
    
    total
}

fn part1(data: &Vec<&str>) -> i64 {
    let mut total = 0;

    for line in data {
        let assignments: Vec<&str> = line.split(",").collect();

        let first: Vec<i64> = assignments[0].split("-").map(|s| s.parse::<i64>().unwrap()).collect();
        let second: Vec<i64> = assignments[1].split("-").map(|s| s.parse::<i64>().unwrap()).collect();
        
        if (first[0] <= second[0] && first[1] >= second[1]) 
            || (first[0] >= second[0] && first[1] <= second[1]) {
                total+=1;
        }
    }
    
    total
}

