use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

pub fn solve(part: u32, input: &Vec<&str>, sample: &Vec<&str>) -> i64 {
    //assert_eq!(part1(&sample), 95437, "Error, sample problem is not solved");
    //assert_eq!(part2(&sample), 0, "Error, sample problem is not solved");

    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => panic!("Illegal value as input. Valid inputs are 1 and 2, recieved {}", part),
    }
}

pub struct LineParsed {
    pub cmd: String,
    pub arg: String,
}

pub struct Node {
    pub name: String,
    pub total: i64,
    pub sub: Vec<Node>,
}


fn is_large(node: Node, large_limit: i64) -> Vec<i64> {
    let mut results = vec![];
    for each in node.sub {
        results.append(&mut is_large(each, large_limit));
    }
    if node.total > large_limit {
        results.push(node.total);
    }
    results
}

fn part2(data: &Vec<&str>) -> i64 {
    let mut total = 0;

    let mut parsed_input = parse_input(data);
    let root = parse_dir(&parsed_input);

    let fs_size = 70000000;
    let total_needed_size = 30000000;
    let needed_size = total_needed_size - (fs_size - root.total);

    let mut candidates = is_large(root, needed_size);
    candidates.sort();
    candidates[0]
}

fn get_new_instruction_offset(data: &[LineParsed]) -> usize {
    let mut i = 0;
    let mut depth = 0;

    while i < data.len() {
        if &data[i].cmd == "cd" && &data[i].arg == ".." {
            depth -= 1;
            if depth == 0 {
                return i+1;
            }
        } else if &data[i].cmd == "cd" {
            depth += 1;
        }
        i += 1;
    }
    if i == data.len() {
        return i
    };
    panic!("Couldnt find offset");
    0
}

fn parse_dir(data: &[LineParsed]) -> Node {
    let mut node = Node {
        name: String::from(&data[0].arg),
        total: 0,
        sub: vec![],
    };

    let mut j = 1; //skiping first cd
    let mut sub_dirs = vec![];
    while j < data.len() {
        if &data[j].cmd == "dir" {
            sub_dirs.push(String::from(&data[j].arg));
        } else if &data[j].cmd == "file" {
            node.total += data[j].arg.parse::<i64>().unwrap();
        } else if &data[j].cmd == "cd" {
            break;
        }
        j += 1;
    }

    // Get everything in dirs
    while j<data.len() {
        if &data[j].cmd == "dir" || &data[j].cmd == "file" {
            panic!("Should not be listed stuff here");
        } else if &data[j].cmd == "cd" && &data[j].arg == ".." {
            if sub_dirs.len() > 0 {
                panic!("Dont know the size of all subdirs");
            }
            node.total += node.sub.iter().map(|d| d.total).sum::<i64>();
            return node;
        } else if &data[j].cmd == "cd" {
            let offset = get_new_instruction_offset(&data[j..]);
            let sub_dir = parse_dir(&data[j..j+offset]);
            sub_dirs.retain(|d| d != &sub_dir.name);
            node.sub.push(sub_dir);
            j += offset;
        }
    } 

    if j == data.len() {
        if sub_dirs.len() > 0 {
            panic!("Dont know the size of all subdirs");
        }
        node.total += node.sub.iter().map(|d| d.total).sum::<i64>();
        return node;
    }

    panic!("should not end up here");
    Node {
        name: String::from("ERROR"),
        total: 0,
        sub: vec![],
    }
}

fn is_sub100k(node: Node) -> i64 {
    let mut total = 0;
    for each in node.sub {
        total += is_sub100k(each);
    }
    if node.total < 100000 {
        total += node.total;
    }
    total
}

fn parse_input(data: &Vec<&str>) -> Vec<LineParsed> {
    let mut parsed_input = vec![];
    for line in data {
        let mut cmd = "".to_string();
        let mut arg = "".to_string();
        if line.len() > 5 && &line[0..4] == "$ cd" {
            cmd = "cd".to_string();
            arg = line[5..].to_string();
            parsed_input.push(LineParsed{cmd, arg});
        } else if line.len() > 5 && &line[0..3] == "dir" {
            cmd = "dir".to_string();
            arg = line[4..].to_string();
            parsed_input.push(LineParsed{cmd, arg});
        } else if &line[0..4] != "$ ls" {
            cmd = "file".to_string();
            arg = line.split(" ").next().unwrap().to_string(); //collect::<Vec<&str>>().next();
            parsed_input.push(LineParsed{cmd, arg});
        }
    }
    parsed_input
}

fn part1(data: &Vec<&str>) -> i64 {
    let mut total = 0;

    let mut parsed_input = parse_input(data);
    let root = parse_dir(&parsed_input);

    let sub100k_dirs = is_sub100k(root);
    sub100k_dirs
}

