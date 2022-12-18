pub fn solve(part: u32, input: &Vec<&str>, sample: &Vec<&str>) -> i64 {
    assert_eq!(part1(&sample), 21, "Error, sample problem is not solved");
    assert_eq!(part2(&sample), 8, "Error, sample problem is not solved");

    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => panic!("Illegal value as input. Valid inputs are 1 and 2, recieved {}", part),
    }
}

fn senic(map: &Vec<Vec<i64>>, i: usize, j: usize) -> i64 {
    let limit = map[i][j];
    let mut current = -1;
    let mut row = i;
    let mut sub_tot = 0;
    while row > 0 {
        row -= 1;
        sub_tot += 1;
        if map[row][j] >= current {
            current = map[row][j];
            if current >= limit {
                break;
            }
        }
    }
    let mut total = sub_tot;
    sub_tot = 0;
    row = i+1;
    current = -1;
    while row < map.len() {
        sub_tot += 1;
        if map[row][j] >= current {
            current = map[row][j];
            if current >= limit {
                break;
            }
        }
        row += 1;
    }
    total *= sub_tot;
    sub_tot = 0;
    let mut col = j;
    current = -1;
    while col > 0 {
        col -= 1;
        sub_tot += 1;
        if map[i][col] >= current {
            current = map[i][col];
            if current >= limit {
                break;
            }
        }
    }
    total *= sub_tot;
    sub_tot = 0;
    col = j+1;
    current = -1;
    while col < map.len() {
        sub_tot += 1;
        if map[i][col] >= current {
            current = map[i][col];
            if current >= limit {
                break;
            }
        }
        col += 1;
    }
    total *= sub_tot;

    total
}

fn part2(data: &Vec<&str>) -> i64 {
    let mut map: Vec<Vec<i64>> = vec![];
    let mut spots: Vec<Vec<i64>> = vec![];

    for line in data {
        map.push(line.chars().map(|c| c.to_digit(10).unwrap() as i64).collect::<Vec<i64>>());
        spots.push(vec![0i64; line.len()]);
    }


    let mut i = 0;
    while i < map.len() {
        let mut j = 0;
        while j < map.len() {
            spots[i][j] = senic(&map, i ,j);
            j+=1;
        }
        i+=1;
    }

    let mut max = 0;
    for row in spots {
        for numb in row {
            if numb > max {
                max = numb;
            }
        }
    }
    
    max
}

fn part1(data: &Vec<&str>) -> i64 {
    let mut total = 0;
    let mut map: Vec<Vec<i64>> = vec![];
    let mut vis: Vec<Vec<bool>> = vec![];

    for line in data {
        map.push(line.chars().map(|c| c.to_digit(10).unwrap() as i64).collect::<Vec<i64>>());
        vis.push(vec![false; line.len()]);
    }


    let mut i = 0;
    while i < map.len() {
        let mut j = 0;
        let mut right = -1;
        let mut left = -1;
        while j < map.len() {
            if map[i][j] > right {
                right = map[i][j];
                vis[i][j] = true;
            }
            if map[i][map.len() -1 - j] > left {
                left = map[i][map.len() -1 - j];
                vis[i][map.len() -1 - j] = true;
            }
            j+=1;
        }
        i+=1;
    }

    i = 0;
    while i < map.len() {
        let mut j = 0;
        let mut up = -1;
        let mut down = -1;
        while j < map.len() {
            if map[j][i] > up {
                up = map[j][i];
                vis[j][i] = true;
            }
            if map[map.len() -1 - j][i] > down {
                down = map[map.len() -1 - j][i];
                vis[map.len() -1 - j][i] = true;
            }
            j+=1;
        }
        i+=1;
    }

    for each in vis {
        total += each.iter().filter(|b| **b).count() as i64;
    }
    
    total
}

