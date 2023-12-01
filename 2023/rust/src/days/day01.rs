pub fn solve(part: u32, input: &Vec<&str>, sample: &Vec<&str>) -> i64 {
    assert_eq!(part2(&sample), 281, "Error, sample problem is not solved");

    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => panic!("Illegal value as input. Valid inputs are 1 and 2, recieved {}", part),
    }
}

fn part2(data: &Vec<&str>) -> i64 {
    let mut total = 0;

    let numbs = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    for line in data {
        let mut new_line = line.to_string();

        let mut i = 0;
        while i < 9 {
            let mut replace_str: String = numbs[i].to_string();
            replace_str.push_str(&(i+1).to_string());
            replace_str.push_str(numbs[i]);
            new_line = new_line.replace(numbs[i], &replace_str);
            i += 1;
        }

        total += get_line_value(&new_line);
    }

    total
}

fn get_line_value(line: &str) -> i64 {
        let mut first: i64 = 0;
        let mut last: i64 = 0;
        for each in line.chars() {
            if each.is_digit(10) {
                first = each.to_digit(10).unwrap() as i64;
                break;
            }
        }
        for each in line.chars().rev() {
            if each.is_digit(10) {
                last = each.to_digit(10).unwrap() as i64;
                break;
            }
        }

        10 * first + last
}

fn part1(data: &Vec<&str>) -> i64 {
    let mut total = 0;

    for line in data {
        total += get_line_value(&line);
    }
    
    total
}

