use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

pub fn solve(part: u32, input: &Vec<&str>, sample: &Vec<&str>) -> i64 {
    assert_eq!(part1(&sample), 95437, "Error, sample problem is not solved");
    //assert_eq!(part2(&sample), 0, "Error, sample problem is not solved");

    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => panic!("Illegal value as input. Valid inputs are 1 and 2, recieved {}", part),
    }
}

struct Node {
    name: String,
    total: Option<i64>,
    sub: Vec<Rc<RefCell<Node>>>,
    parent: Option<Rc<RefCell<Node>>>,
}

fn part2(data: &Vec<&str>) -> i64 {
    0
}

fn part1(data: &Vec<&str>) -> i64 {
    let mut total = 0;

    let root = Rc::new(RefCell::new(Node {
        name: String::from("/"),
        total: None,
        sub: vec![],
        parent: None,
    }));

    let mut current_node = Rc::clone(&root);
    let mut i = 0;
    for line in data.iter().skip(1) {
        if &line[0..1] == "$" {
            //Commande
            if &line[2..4] == "ls" {
                // do nothing or call recursive add
            } else if &line[2..6] == "cd ." {
                //return this dir
            } else if &line[2..4] == "cd" {
                println!("Changing dir to {}", &line[5..]);
                //current_node = current_node.borrow().sub[0];
            } 
        } else {
            if &line[0..3] == "dir" {
                let mut new_dir = Rc::new(RefCell::new(Node {
                    name: String::from(&line[4..]),
                    total: None,
                    sub: vec![],
                    parent: Some(Rc::clone(&current_node)),
                }));
                current_node.borrow_mut().sub.push(Rc::clone(&new_dir));
                println!("Adding new sub dir:{}{}/",current_node.borrow().name, current_node.borrow().sub.last().unwrap().borrow().name);
            } else {
                let mut parts: Vec<&str> = line.split(" ").collect();
                
                let old_val = current_node.borrow().total;
                current_node.borrow_mut().total = match old_val {
                    Some(v) => Some(v + parts[0].parse::<i64>().unwrap()),
                    None => Some(parts[0].parse::<i64>().unwrap()),
                };
                println!("Adding to total: {}",current_node.borrow().total.unwrap());
            }
        }
    }
    
    total
}

