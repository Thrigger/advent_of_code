pub fn solve(part: u32, input: &Vec<&str>, sample: &Vec<&str>) -> i64 {
    assert_eq!(part1(&sample), 15, "Error, sample problem is not solved, part 1");
    assert_eq!(part2(&sample), 12, "Error, sample problem is not solved, part 2");

    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => panic!("Illegal value as input. Valid inputs are 1 and 2, recieved {}", part),
    }
}

fn part2(data: &Vec<&str>) -> i64 {
    let mut total= 0;

    for each in data {
        let values: Vec<&str> = each.split(" ").collect();
        
        let opp = match values[0] {
            "A" => 0,
            "B" => 1,
            "C" => 2,
            _ => panic!(),
        };
        let my = match values[1] {
            "X" => (opp +3 -1)%3,
            "Y" => opp,
            "Z" => (opp+1)%3,
            _ => panic!(),
        };

        let mut ret_val = 0;

        if (opp + 1)%3 == my {
            ret_val += 6;
        } else if opp == my {
            ret_val += 3;
        }

        ret_val += my + 1;
        total += ret_val;
    }
    
    total
}
fn part1(data: &Vec<&str>) -> i64 {
    let mut total= 0;

    for each in data {
        let values: Vec<&str> = each.split(" ").collect();
        let opp = match values[0] {
            "A" => 0,
            "B" => 1,
            "C" => 2,
            _ => panic!(),
        };
        let my = match values[1] {
            "X" => 0,
            "Y" => 1,
            "Z" => 2,
            _ => panic!(),
        };

        let mut ret_val = 0;

        if (opp + 1)%3 == my {
            ret_val += 6;
        } else if opp == my {
            ret_val += 3;
        }

        ret_val += my + 1;
        total += ret_val;
    }
    
    total
}

