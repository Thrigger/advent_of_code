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
        let lvls = line.split(" ")
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let mut i = 0;
        while i < lvls.len() {
            let mut new_vec = lvls.clone();
            new_vec.remove(i);

            let mut inc = is_ok_steps(&new_vec, true);
            let mut dec = is_ok_steps(&new_vec, false);
            if inc || dec {
                total += 1;
                break;
            }

            i += 1;
        }
    }
    
    total
}

fn is_ok_steps(lvls: &Vec<i64>, inc: bool) -> bool {
    let mut last = lvls[0];

    for lvl in &lvls[1..] {
        let diff = match inc {
            true => lvl - last,
            false => last - lvl,
        };
        if diff < 1 || diff > 3 {
            return false;
        }
        last = *lvl;
    }
    true
}

fn part1(data: &Vec<&str>) -> i64 {
    let mut total = 0;

    for line in data {
        let lvls = line.split(" ")
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let mut inc = is_ok_steps(&lvls, true);
        let mut dec = is_ok_steps(&lvls, false);
        if inc || dec {
            total += 1;
        }
    }
    
    total
}
