pub fn solve(part: u32, input: &Vec<&str>, sample: &Vec<&str>) -> i64 {
    assert_eq!(part1(&sample), 10605, "Error, sample problem is not solved");
    assert_eq!(part2(&sample), 2713310158, "Error, sample problem is not solved");

    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => panic!("Illegal value as input. Valid inputs are 1 and 2, recieved {}", part),
    }
}

fn part2(data: &Vec<&str>) -> i64 {
    let mut total = 0;
    let mut monkeys: Vec<Monkey> = vec![];

    let mut tot_div = 1;
    for group in data {
        monkeys.push(creat_monkey(group));
        tot_div *= monkeys.last().unwrap().test;
    }

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            for j in 0..monkeys[i].items.len() {
                let new_item = new_value(&monkeys[i].operation, monkeys[i].items[j]);
                let new_item = new_item % tot_div;
                monkeys[i].inspects += 1;

                let next_monkey = match new_item % monkeys[i].test {
                    0 => monkeys[i].be,
                    _ => monkeys[i].bne,
                };

                monkeys[next_monkey].items.push(new_item);
            }
            monkeys[i].items.clear();
        }
    }
    
    let mut inspects: Vec<i64> = vec![];
    for each in monkeys {
        inspects.push(each.inspects);
    }
    inspects.sort();
    inspects.reverse();

    inspects[0] * inspects[1]
}

fn part1(data: &Vec<&str>) -> i64 {
    let mut total = 0;
    let mut monkeys: Vec<Monkey> = vec![];

    for group in data {
        monkeys.push(creat_monkey(group));
    }

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            for j in 0..monkeys[i].items.len() {
                let new_item = new_value(&monkeys[i].operation, monkeys[i].items[j])/3;
                monkeys[i].inspects += 1;

                let next_monkey = match new_item % monkeys[i].test {
                    0 => monkeys[i].be,
                    _ => monkeys[i].bne,
                };

                monkeys[next_monkey].items.push(new_item);
            }
            monkeys[i].items.clear();
        }
    }
    
    let mut inspects = vec![];
    for each in monkeys {
        inspects.push(each.inspects);
    }
    inspects.sort();
    inspects.reverse();

    inspects[0] * inspects[1]
}

fn new_value(op: &str, old: i64) -> i64 {
    let mut parts: Vec<&str> = op.split(" ").collect();
    let mult = match parts[1] {
        "*" => true,
        _ => false,
    };
    parts.remove(1);

    let parts = parts.iter().map(|s| if s==&"old" {old} else {s.parse::<i64>().unwrap()});

    match mult {
        true => parts.product(),
        false => parts.sum(),
    }
}

fn creat_monkey(input: &str) -> Monkey {
    let mut in_split: Vec<&str> = input.split("\n").map(|s| s.trim()).collect();

    let items: Vec<i64> = in_split[1][16..]
        .split(", ")
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    let operation =  in_split[2][17..].to_string();
    let test = in_split[3][19..].parse::<i64>().unwrap();
    let be = in_split[4][25..].parse::<usize>().unwrap();
    let bne = in_split[5][26..].parse::<usize>().unwrap();

    Monkey {items, operation, test, be, bne, inspects: 0}
}

struct Monkey {
    items: Vec<i64>,
    operation: String,
    test: i64,
    be: usize,
    bne: usize,
    inspects: i64,
}


