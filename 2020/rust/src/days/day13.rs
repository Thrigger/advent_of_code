pub fn solve(part: u32, input: &str, sample: &str) -> i64 {

    assert_eq!(part1(&sample), 295, "Error, sample problem is not solved");
    assert_eq!(part2(&sample), 1068781, "Error, sample problem is not solved");
    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => panic!("Illegal value as input. Valid inputs are 1 and 2, recieved {}", part),
    }
}

fn part1(data: &str) -> i64 {
    let mut data_itter  = data.lines();
    let start_time      = data_itter.next().unwrap().parse::<i64>().unwrap();
    let all_buses       = data_itter.next().unwrap();
    let buses: Vec<i64> = get_usable_buses(all_buses);

    let mut time = start_time;
    let mut fitting_bus = 0;
    loop {
        for each in &buses {
            if time % each == 0 {
                fitting_bus = *each;
                break;
            }
        }
        
        if fitting_bus != 0 {
            break;
        }
        time += 1;
    }
    let wait_time = time - start_time;

    fitting_bus * wait_time
}

fn part2(data: &str) -> i64 {
    let mut data = data.lines();
    
    let start_time = data.next().unwrap().parse::<i64>().unwrap();
    let buses = data.next().unwrap().split(",");

    let mut solution = start_time;
    let mut increment = 1;
    for (offset, each) in buses.enumerate() {
        if each != "x" {
            loop {
                let bus_number = each.parse::<i64>().unwrap();
                if (solution + offset as i64) % bus_number == 0 {
                    increment = increment * bus_number;
                    break;
                } else {
                    solution += increment;
                }
            }
        }
    }

    solution
}

fn get_usable_buses(all_buses: &str) -> Vec<i64> {
    let mut usable_buses: Vec<i64> = vec![];

    for bus in all_buses.split(",") {
        if bus != "x" {
            let new_value = bus.parse::<i64>().unwrap();
            usable_buses.push(new_value);
        }
    }

    usable_buses
}
