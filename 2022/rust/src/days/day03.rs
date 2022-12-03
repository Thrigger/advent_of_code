pub fn solve(part: u32, input: &Vec<&str>, sample: &Vec<&str>) -> i64 {
    assert_eq!(part1(&sample), 157, "Error, sample problem is not solved");
    assert_eq!(part2(&sample), 70, "Error, sample problem is not solved");

    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => panic!("Illegal value as input. Valid inputs are 1 and 2, recieved {}", part),
    }
}

fn get_item_points(item: char) -> i64 {
    let mut value = item as u32;
    if value > 96 {
        value -= 96;
    } else {
        value -= 38;
    }
    value as i64
}


fn part2(data: &Vec<&str>) -> i64 {
    let mut total: i64  = 0;

    let mut i = 0;
    while i < data.len() {
        let items: Vec<char> = data[i].chars().collect();

        for item in items {
            if data[i].contains(item) && data[i+1].contains(item) && data[i+2].contains(item) {
                total += get_item_points(item);
                break;
            }
        }
        i+=3;
    }
    
    total
}

fn part1(data: &Vec<&str>) -> i64 {
    let mut total: i64  = 0;

    for rucksack in data {
        let size = rucksack.len();
        let items: Vec<char> = rucksack.chars().collect();
        let mut i = 0;

        for item in items {
            let last_place = rucksack.rfind(item).unwrap();

            if (i < size/2 && last_place >= size/2) || (i >= size/2 && last_place < size/2) {
                total += get_item_points(item);
                break;
            }
            i += 1;
        }
    }
    
    total
}

