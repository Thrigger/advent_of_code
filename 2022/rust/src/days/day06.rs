pub fn solve(part: u32, input: &Vec<&str>, sample: &Vec<&str>) -> i64 {
    assert_eq!(part1(&sample), 7, "Error, sample problem is not solved");
    assert_eq!(part2(&sample), 19, "Error, sample problem is not solved");

    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => panic!("Illegal value as input. Valid inputs are 1 and 2, recieved {}", part),
    }
}

fn part2(data: &Vec<&str>) -> i64 {
    find_start(data[0], 14)
}

fn is_start(mem: &Vec<char>) -> bool {
    let mut i = 0;
    while i < mem.len() {
        let mut j = i+1;
        while j < mem.len() {
            if mem[i] == mem[j] {
                return false;
            }
            j+=1;
        }
        i+=1;
    }
    true
}

fn find_start(line: &str, length: usize) -> i64 {
    let mut total = 0;

    let array = line.chars();
    let mut mem: Vec<char> = vec![];

    for each in array {
        total += 1;
        mem.push(each);
        if mem.len() > length {
            mem.remove(0);
            if is_start(&mem) {
                break;
            }
        }
    }
    total
}


fn part1(data: &Vec<&str>) -> i64 {
    find_start(data[0],  4)
}

