pub fn solve(part: u32, input: &Vec<&str>, sample: &Vec<&str>) -> i64 {
    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => panic!("Illegal value as input. Valid inputs are 1 and 2, recieved {}", part),
    }
}

fn part2(data: &Vec<&str>) -> i64 {
    let mut is_set_up = false;
    let mut map: Vec<Vec<&str>>= vec![vec![]];

    for group in data {
        if !is_set_up {
            map = setup(group);
            is_set_up = true;
        } else {
            for line in group.lines() {
                let parts: Vec<usize> = line.split(" ").filter_map(|s| match s.parse::<usize>() {
                    Ok(i) => Some(i),
                    _     => None,
                }).collect();

                let mut tmp_map: &mut Vec<&str> = &mut map[parts[1]-1];
                let offset = tmp_map.len()-parts[0];
                let mut drained_map: &mut Vec<&str> = &mut tmp_map.drain(offset..).collect();
                map[parts[2]-1].append(&mut drained_map);
            }
        }
    }

    for each in &mut map {
        println!("{}",each.pop().unwrap());
    }
    
    0
}

fn setup(group: &str) -> Vec<Vec<&str>> {
    let mut lines: Vec<&str> = group.lines().collect();
    let mut elements = 0;
    let mut map = vec![];

    for line in lines.iter().rev() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts[0] == "1" {
            elements = parts.len();
            let mut i = 0;
            while i < elements {
                map.push(vec![]);
                i+= 1;
            }
        } else {
            let mut i = 0;
            while i < elements {
                let offset = i * 4;
                let current = &line[offset..offset+3];
                if current != "   " {
                    map[i].push(current);
                }

                i+= 1;
            }
        }

    }
    map
}

fn part1(data: &Vec<&str>) -> i64 {
    let mut is_set_up = false;
    let mut map: Vec<Vec<&str>>= vec![vec![]];

    for group in data {
        if !is_set_up {
            map = setup(group);
            is_set_up = true;
        } else {
            for line in group.lines() {
                let parts: Vec<usize> = line.split(" ").filter_map(|s| match s.parse::<usize>() {
                    Ok(i) => Some(i),
                    _     => None,
                }).collect();

                let mut tmp_map: &mut Vec<&str> = &mut map[parts[1]-1];
                let offset = tmp_map.len()-parts[0];
                let mut drained_map: &mut Vec<&str> = &mut tmp_map.drain(offset..).collect();
                drained_map.reverse();
                map[parts[2]-1].append(&mut drained_map);
            }
        }
    }

    for each in &mut map {
        println!("{}",each.pop().unwrap());
    }
    
    0
}

