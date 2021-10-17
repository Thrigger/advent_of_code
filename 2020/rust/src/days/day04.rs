use std::collections::HashMap;

pub fn solve(part: u32, input: &str, sample: &str) -> i64 {
    assert_eq!(part1(sample), 2, "Error, sample problem is not solved");
    match part {
        1 => part1(input),
        2 => part2(input),
        _ => panic!("Illegal value as input. Valid inputs are 1 and 2, recieved {}", part),
    }
}

fn validate_hgt(input: &str) -> bool {
    let unit = &input[input.len()-2..];
    if unit != "cm" && unit != "in" {
        return false
    }
    let value = input[..input.len()-2].parse::<i32>().unwrap();

    if unit == "in" && value >= 59 && value <= 76 {
        return true
    } else if unit == "cm" && value >= 150 && value <= 193 {
        return true
    }
    false
}

fn validate_hcl(input: &str) -> bool {
    let mut input_bytes = input.bytes();
    if input_bytes.next() != Some(b'#') {
        return false
    }

    for (i, byte) in input_bytes.enumerate() {
        if i >= 6 {
            return false
        }
        if (byte < ('0' as u8) || byte > ('9' as u8)) &&
            (byte < ('a' as u8) || byte > ('f' as u8)) {
                return false
        }
    }

    true
}

fn validate_ecl(input: &str) -> bool {
    match input {
        "amb" => return true,
        "blu" => return true,
        "brn" => return true,
        "gry" => return true,
        "grn" => return true,
        "hzl" => return true,
        "oth" => return true,
        _ => return false,
    };
}

fn part2(data: &str) -> i64 {
    let mut valid_passports = 0;
    let mut current_pass = HashMap::new();
    current_pass.insert(String::from("byr"), false);
    current_pass.insert(String::from("iyr"), false);
    current_pass.insert(String::from("eyr"), false);
    current_pass.insert(String::from("hgt"), false);
    current_pass.insert(String::from("hcl"), false);
    current_pass.insert(String::from("ecl"), false);
    current_pass.insert(String::from("pid"), false);
    current_pass.insert(String::from("cid"), false);

    for passport in data.split("\n\n") {
        let mut is_this_valid = true;

        for field in passport.split(&['\n',' '][..]) {
            if field == "" {
                break;
            }
            let mut field_iter = field.split(':');
            let key = field_iter.next().unwrap();
            let value = field_iter.next().unwrap();
            match key {
               "byr" => current_pass.insert(key.to_string(), 
                                           value.parse::<u32>().unwrap() >= 1920 &&
                                           value.parse::<u32>().unwrap() <= 2002),
               "iyr" => current_pass.insert(key.to_string(), 
                                           value.parse::<u32>().unwrap() >= 2010 && 
                                           value.parse::<u32>().unwrap() <= 2020),
               "eyr" => current_pass.insert(key.to_string(), 
                                           value.parse::<u32>().unwrap() >= 2020 && 
                                           value.parse::<u32>().unwrap() <= 2030),
               "hgt" => current_pass.insert(key.to_string(), validate_hgt(value)),
               "hcl" => current_pass.insert(key.to_string(), validate_hcl(value)),
               "ecl" => current_pass.insert(key.to_string(), validate_ecl(value)),
               "pid" => current_pass.insert(key.to_string(), value.bytes().len() == 9),
               "cid" => current_pass.insert(key.to_string(), true),
               &_ => None,
            };
        }
        // override cid
        current_pass.insert("cid".to_string(), true);

        for each in current_pass.values() {
            if !each {
                is_this_valid = false;
                break;
            }
        }
                
        if is_this_valid {
            valid_passports += 1;
        } 

        for each in current_pass.values_mut() {
            *each = false;
        }
    }

    valid_passports 
}

fn part1(data: &str) -> i64 {
    let mut valid_passports = 0;
    
    let mut current_pass = HashMap::new();
    current_pass.insert(String::from("byr"), false);
    current_pass.insert(String::from("iyr"), false);
    current_pass.insert(String::from("eyr"), false);
    current_pass.insert(String::from("hgt"), false);
    current_pass.insert(String::from("hcl"), false);
    current_pass.insert(String::from("ecl"), false);
    current_pass.insert(String::from("pid"), false);
    current_pass.insert(String::from("cid"), false);

    for passport in data.split("\n\n") {
        let mut is_this_valid = true;

        for field in passport.split(&['\n',' '][..]) {
            if field == "" {
                break;
            }
            let mut field_iter = field.split(':');
            let key = field_iter.next().unwrap();
            current_pass.insert(key.to_string(), true);
        }
        // override cid
        current_pass.insert("cid".to_string(), true);

        for each in current_pass.values() {
            if !each {
                is_this_valid = false;
                break;
            }
        }
                
        if is_this_valid {
            valid_passports += 1;
        }

        for each in current_pass.values_mut() {
            *each = false;
        }

    }

    valid_passports
}
 
