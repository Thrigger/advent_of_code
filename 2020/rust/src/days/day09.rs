pub fn solve(part: u32, input: &str, sample: &str) -> i64 {
    assert_eq!(part1(sample, 5), 127, "Error, sample problem is not solved");
    assert_eq!(part2(sample, 5), 62, "Error, sample problem is not solved");
    match part {
        1 => part1(input, 25),
        2 => part2(input, 25),
        _ => panic!("Illegal value as input. Valid inputs are 1 and 2, recieved {}", part),
    }
}

fn bad_new_value(old: &Vec<i64>, new: i64) -> bool {
    for first_value in old {
        for second_value in old {
            if *first_value + *second_value == new {
                return false;
            }
        }
    }
    true
}

fn find_first_failing (data: &str, mem_length: usize) -> i64 {
    /* This vector need to be 25 long for real problem and 5 long for example */
    let mut current_numbers = vec![0; mem_length];
    
    for (i, each_in) in data.split("\n").enumerate() {
        if each_in == "" {
            break;
        }
        let current = each_in.parse().unwrap();

        if i >= mem_length {
            if bad_new_value(&current_numbers, current) {
                return current;
            }
        }
        
        current_numbers.remove(0);
        current_numbers.push(current);
    }
    0
}

fn part1(data: &str, mem_length: usize) -> i64 {
    find_first_failing(&data, mem_length)
}

fn part2(data: &str, mem_length: usize) -> i64 {
    let first_fail = find_first_failing(&data, mem_length);

    let numbers = data.split("\n").collect::<Vec<&str>>();

    let mut sum;
    let mut min;
    let mut max;
    for i in 0..numbers.len() {
        sum = 0;
        match numbers[i].parse::<i64>(){
            Ok(i) => min = i,
            Err(_) => {println!("Error couldnt parse line"); break},
        };
        match numbers[i].parse::<i64>(){
            Ok(i) => max = i,
            Err(_) => {println!("Error couldnt parse line"); break},
        };
        for j in i..numbers.len() {
            let curr;
            match numbers[j].parse::<i64>(){
                Ok(i) => curr = i,
                Err(_) => {println!("Error couldnt parse line"); break},
            };
            sum += curr;
            if curr < min {
                min = curr;
            } 
            if curr > max {
                max = curr;
            }
        
            if sum == first_fail {
                /* Correct value */
                return min+max;
            } else if sum > first_fail {
                break;
            }
        }
        if sum == first_fail {
            break;
        }
    }
    0
}

