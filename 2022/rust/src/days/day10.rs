pub fn solve(part: u32, input: &Vec<&str>, sample: &Vec<&str>) -> i64 {
    assert_eq!(part1(&sample), 13140, "Error, sample problem is not solved");
    //assert_eq!(part2(&sample), 0, "Error, sample problem is not solved");

    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => panic!("Illegal value as input. Valid inputs are 1 and 2, recieved {}", part),
    }
}

fn part2(data: &Vec<&str>) -> i64 {
    let mut reg_x = 1;
    let mut cyc = 0;

    for line in data {
        let parts: Vec<&str> = line.split(" ").collect();
        let op: &str = parts[0];
        let val: i64 = match op {
            "addx" => parts[1].parse().unwrap(),
            _ => 0,
        };

        let range = match op {
            "addx" => 2,
            "noop" => 1,
            _ => panic!(),
        };

        for _ in 00..range {
            let sprit = vec![reg_x - 1, reg_x, reg_x +1];
            if sprit.contains(&(cyc%40)) {
                print!("{}", '\u{1F385}');
            } else {
                print!(" ");
            }

            if cyc % 40 == 39 {
                println!("");
            }
            cyc += 1;
        }

        if op == "addx" {
            reg_x += val;
        }
    }
    
    1337
}

fn part1(data: &Vec<&str>) -> i64 {
    let mut total = 0;
    let mut reg_x = 1;
    let stop_values: Vec<i64> = vec![20,60,100,140,180,220];
    
    let mut cyc = 1;
    for line in data {
        let parts: Vec<&str> = line.split(" ").collect();
        let op: &str = parts[0];
        let val: i64 = match op {
            "addx" => parts[1].parse().unwrap(),
            _ => 0,
        };

        let range = match op {
            "addx" => 2,
            "noop" => 1,
            _ => panic!(),
        };

        for _ in 00..range {
            if stop_values.contains(&cyc) {
                total += reg_x * cyc;
            }
            cyc += 1;
        }

        if op == "addx" {
            reg_x += val;
        }
    }
    
    total
}

