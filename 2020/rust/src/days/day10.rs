use std::collections::HashMap;

pub fn solve(part: u32, input: &Vec<i64>, sample: &Vec<i64>) -> i64 {
    assert_eq!(part1(&sample), 220, "Error, sample problem is not solved");
    assert_eq!(part2(&sample), 19208, "Error, sample problem is not solved");
    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => panic!("Illegal value as input. Valid inputs are 1 and 2, recieved {}", part),
    }
}

fn part1(input: &Vec<i64>) -> i64 {
    let mut adapters = input.clone();
    adapters.push(0);
    adapters.sort();
    adapters.push(adapters[adapters.len()-1]+3);
    
    let mut increments = [0; 3];
    for i in 0..adapters.len()-1  {
        let diff = adapters[i+1]-adapters[i];
        match diff {
            1 => increments[0] += 1, 
            2 => increments[1] += 1, 
            3 => increments[2] += 1, 
            _ => println!("Error. Increment is to big"),
        };
    }
    increments[0] * increments[2]
}

fn how_many_solutions(input: &Vec<i64>, mut known: &mut HashMap<i64,i64>) -> i64 {
    let mut sum = match input.len() {
        1 => 1,
        _ => 0,
    };

    if known.contains_key(&input[0]) {
        return *known.get(&input[0]).unwrap();
    }
    let mut i = 1;

    while i < 4 && i < input.len() {
        if input[i]-input[0] <= 3 {
            let mut new_vec = input.clone();
            for _j in 0..i {
                new_vec.remove(0);
            }
            
            sum += how_many_solutions(&new_vec, &mut known);
        } else {
            break;
        }
        i += 1;
    }

    known.insert(input[0], sum);
    sum
}

fn part2(input: &Vec<i64>) -> i64 {
    let mut adapters = input.clone();
    adapters.push(0);
    adapters.sort();
    adapters.push(adapters[adapters.len()-1]+3);

    let mut known = HashMap::new();

    known.insert(adapters[adapters.len()-1], 1);
    
    how_many_solutions(&adapters, &mut known)
}

