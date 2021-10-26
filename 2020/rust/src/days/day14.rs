use std::collections::HashMap;
extern crate thrigger_support;

pub fn solve(part: u32, input: &Vec<&str>, sample: &Vec<&str>) -> i64 {

//  Cant test part 1 with the same sample input as part 2.
//    assert_eq!(part1(&sample), 165, "Error, sample problem is not solved");
    assert_eq!(part2(&sample), 208, "Error, sample problem is not solved");
    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => panic!("Illegal value as input. Valid inputs are 1 and 2, recieved {}", part),
    }
}

fn part1(data: &Vec<&str>) -> i64 {
    let mut mask_set: u64 = 0;
    let mut mask_clear: u64 = 0;
    let mut mem = HashMap::new();

    for line in data {
        let arg = line.split(" = ").collect::<Vec<&str>>();
        if arg[0].contains("mask") {
            mask_set = u64::from_str_radix(&arg[1].replace("X", "0"), 2).unwrap();
            mask_clear = u64::from_str_radix(&arg[1].replace("X", "1"), 2).unwrap();
        } else {
            let address = thrigger_support::find_between(arg[0], "[", "]").unwrap();
            let value = (arg[1].parse::<u64>().unwrap()) & mask_clear | mask_set;
            mem.insert(address, value);
        }
    }

    mem.values().sum::<u64>() as i64
}

fn combine_with_mask(base: &str, mask: &str) -> String {
    let base_chars: Vec<char> = base.chars().collect();
    let mut mask_chars: Vec<char> = mask.chars().collect();

    let mut i = 0;
    while i < base_chars.len() {
        if mask_chars[i] == '0' {
            mask_chars[i] = base_chars[i];
        }
        i += 1;
    }
    let result = mask_chars.iter().collect();
    result
}

fn expand_addr(addr: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new: Vec<Vec<char>> = vec![];

    for each in addr {
        let mut temp0: Vec<char> = vec![];
        let mut temp1: Vec<char> = vec![];
        let mut swapped = false;
        for bit in each {
            if !swapped && bit == 'X' {
                temp0.push('0');
                temp1.push('1');
                swapped = true;
            } else {
                temp0.push(bit);
                temp1.push(bit);
            }
        }
        new.push(temp0);
        new.push(temp1);
    }
    new
}

fn get_addresses(base: &str, mask: &str) -> Vec<u64> {
    let combined = combine_with_mask(base, mask);

    let mut all_addresses = vec![combined.chars().collect::<Vec<char>>()];
    while all_addresses[0].iter().collect::<String>().find("X") != None {
        all_addresses = expand_addr(all_addresses);
    }

    let mut addresses: Vec<u64> = vec![];

    for each in all_addresses {
        addresses.push(u64::from_str_radix(&each.iter().collect::<String>(), 2).unwrap());
    }
    addresses
}

fn part2(data: &Vec<&str>) -> i64 {
    let mut mask = "";
    let mut mem = HashMap::new();

    for line in data {
        let arg = line.split(" = ").collect::<Vec<&str>>();
        if arg[0].contains("mask") {
            mask = arg[1];
        } else {
            let base_address = thrigger_support::find_between(arg[0], "[", "]").unwrap().parse::<u64>().unwrap();
            let base_address = format!("{:036b}", base_address);
            let addresses = get_addresses(&base_address, mask);
            let value = arg[1].parse::<u64>().unwrap();
            for address in addresses {
                mem.insert(address, value);
            }
        }
    }

    mem.values().sum::<u64>() as i64
}


