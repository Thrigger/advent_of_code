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

        let mut indexes = vec![];
        for numb in &numbs {
            indexes.push(line.find(numb));
        }
        let mut min = None;
        for each in indexes {
            if min == None || each < min {
                min = each;
            }
        }
        if let Some(i) = min {
            new_line = new_line.replacen(&numbs[i], &i.to_string(), 1);
        }
        let mut indexes = vec![];
        for numb in &numbs {
            indexes.push(line.rfind(numb));
        }
        let mut max = None;
        for each in indexes {
            if max == None || each > max {
                max = each;
            }
        }
        if let Some(i) = max {
            new_line = new_line.replace(&numbs[i], &i.to_string());
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

