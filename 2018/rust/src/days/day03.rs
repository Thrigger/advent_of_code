pub fn solve(part: u32, input: &Vec<&str>, sample: &Vec<&str>) -> i64 {
    assert_eq!(part1(&sample), 4, "Error, sample problem is not solved");
    assert_eq!(part2(&sample), 3, "Error, sample problem is not solved");

    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => panic!("Illegal value as input. Valid inputs are 1 and 2, recieved {}", part),
    }
}

fn part2(data: &Vec<&str>) -> i64 {
    let size = 10000;
    let mut grid = vec![vec![0; size]; size];

    for line in data {
        fill_grid(&mut grid, line);
    }

    for line in data {
        if let Some(id) = is_correct_claim(&grid, line) {
            return id as i64;
        }
    }

    0
}

fn is_correct_claim(grid: &Vec<Vec<i64>>, claim: &str) -> Option<usize>{
    let mut parts: Vec<&str> = claim.split(" ").collect();
    parts[2] = &parts[2][..parts[2].len()-1];
    let cords: Vec<usize> = parts[2].split(",").map(|s| s.parse::<usize>().unwrap()).collect();
    let size: Vec<usize> = parts[3].split("x").map(|s| s.parse::<usize>().unwrap()).collect();

    for i in cords[0]..(cords[0] + size[0]) {
        for j in cords[1]..(cords[1] + size[1]) {
            if grid[i][j] != 1 {
                return None;
            }
        }
    }
    Some(parts[0][1..].parse::<usize>().unwrap())
}

fn fill_grid(grid: &mut Vec<Vec<i64>>, claim: &str) {
    let mut parts: Vec<&str> = claim.split(" ").collect();
    parts[2] = &parts[2][..parts[2].len()-1];
    let cords: Vec<usize> = parts[2].split(",").map(|s| s.parse::<usize>().unwrap()).collect();
    let size: Vec<usize> = parts[3].split("x").map(|s| s.parse::<usize>().unwrap()).collect();

    for i in cords[0]..(cords[0] + size[0]) {
        for j in cords[1]..(cords[1] + size[1]) {
            grid[i][j] += 1;
        }
    }
}

fn part1(data: &Vec<&str>) -> i64 {
    let size = 10000;
    let mut grid = vec![vec![0; size]; size];

    for line in data {
        fill_grid(&mut grid, line);
    }

    let mut total = 0;
    for row in grid {
        for point in row {
            if point > 1 {
                total += 1;
            }
        }
    }
    total
}

