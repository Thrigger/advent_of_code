pub fn solve(part: u32, input: &Vec<&str>, sample: &Vec<&str>) -> i64 {
    assert_eq!(part1(&sample), 18, "Error, sample problem is not solved");
    assert_eq!(part2(&sample), 9, "Error, sample problem is not solved");

    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => panic!("Illegal value as input. Valid inputs are 1 and 2, recieved {}", part),
    }
}

fn part2(data: &Vec<&str>) -> i64 {
    let mut total = 0;

    let mut cross: Vec<Vec<char>> = vec![];
    for line in data {
        cross.push(line.chars().collect());
    }

    let mut i = 0;
    while i < cross.len() {
        let mut j = 0;
        while j < cross[0].len() {
            if i + 2 < cross.len() && j + 2 < cross[i].len() 
                && is_x_mas((cross[i][j], cross[i][j+2], cross[i+1][j+1], cross[i+2][j+2], cross[i+2][j])) {
                total += 1;
            }
            j += 1;
        }
        i += 1;
    }
    
    total
}

fn is_x_mas(chars: (char, char, char, char, char)) -> bool {
    match chars {
        ('M', 'M', 'A', 'S', 'S') => true,
        ('S', 'M', 'A', 'M', 'S') => true,
        ('S', 'S', 'A', 'M', 'M') => true,
        ('M', 'S', 'A', 'S', 'M') => true,
        _ => false,
    }
}

fn is_xmas(chars: (char, char, char, char)) -> bool {
    match chars {
        ('X', 'M', 'A', 'S') => true,
        ('S', 'A', 'M', 'X') => true,
        _ => false,
    }
}

fn part1(data: &Vec<&str>) -> i64 {
    let mut total = 0;

    let mut cross: Vec<Vec<char>> = vec![];
    for line in data {
        cross.push(line.chars().collect());
    }

    let mut i = 0;
    while i < cross.len() {
        let mut j = 0;
        while j < cross[0].len() {
            if j + 3 < cross[i].len() && is_xmas((cross[i][j], cross[i][j+1], cross[i][j+2], cross[i][j+3])) {
                total += 1;
            }
            if i + 3 < cross.len() && is_xmas((cross[i][j], cross[i+1][j], cross[i+2][j], cross[i+3][j])) {
                total += 1;
            }
            if i + 3 < cross.len() && j + 3 < cross[i].len() 
                && is_xmas((cross[i][j+3], cross[i+1][j+2], cross[i+2][j+1], cross[i+3][j])) {
                total += 1;
            }
            if i + 3 < cross.len() && j + 3 < cross[i].len() 
                && is_xmas((cross[i][j], cross[i+1][j+1], cross[i+2][j+2], cross[i+3][j+3])) {
                total += 1;
            }
            j += 1;
        }
        i += 1;
    }
    
    total
}

