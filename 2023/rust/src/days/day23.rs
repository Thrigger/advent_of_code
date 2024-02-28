use Point::*;
pub fn solve(part: u32, input: &Vec<&str>, sample: &Vec<&str>) -> i64 {
//pub fn solve(part: u32, input: &Vec<i64>, sample: &Vec<i64>) -> i64 {
    assert_eq!(part1(&sample), 94, "Error, sample problem is not solved");
    assert_eq!(part2(&sample), 154, "Error, sample problem is not solved");

    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => panic!("Illegal value as input. Valid inputs are 1 and 2, recieved {}", part),
    }
}

fn part2(data: &Vec<&str>) -> i64 {
    let mut total = 0;
    let mut map: Vec<Vec<Point>> = vec![];

    for line in data {
        let mut v = vec![];
        for c in line.chars() {
            if c == '#' {
                v.push(C(c));
            } else {
                v.push(C('.'));
            }
        }
        map.push(v);
    }

    let mut start = 0;
    for (i, each) in map[0].iter().enumerate() {
        match each {
            C(c) => {
                if *c == '.' {
                    start = i;
                    break;
                }},
            _ => (),
        }
    }

    println!("start is row 0 col {}", start);

    print_map(&map);
    // Solve like part 1
    // total = get_steps_from_goal(map, 0, start, true);

    let mut visited = vec![(0, start)];
    total = get_steps_from_goal2(&map, visited);

    total
}

fn get_steps_from_goal2(map: &Vec<Vec<Point>>, visited: Vec<(usize, usize)>) -> i64 {
    let (row, col) = visited.last().unwrap();

    // if we want to print map then we need to paint the visited and undo later
    //map[row][col] = I(0);
    //print_map(&map);

    if *row == map.len()-1 {
        return 0;
    }

    let paths = possible_paths2(&map, *row, *col, &visited);
    let mut max_steps = -100000;
    for (next_row, next_col) in paths {
        let mut new_visited = visited.clone();
        new_visited.push((next_row, next_col));
        let steps = get_steps_from_goal2(&map, new_visited) + 1;
        
        if steps > max_steps {
            max_steps = steps;
        }
    }
    max_steps
}

fn possible_paths2(map: &Vec<Vec<Point>>, row: usize, col: usize, visited: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut ret = vec![];
    let mut pos = vec![];

    if row > 0 {
        pos.push((row - 1, col));
    }
    if row < map.len() - 1 {
        pos.push((row + 1, col));
    }
    if col > 0 {
        pos.push((row, col - 1));
    }
    if col < map[0].len() - 1 {
        pos.push((row, col + 1));
    }

    for (r, c) in pos {
        if let C(ch) = map[r][c] {
            //println!("Found {ch}");
            match ch {
                '.' => ret.push((r, c)),
                'v' => if r > row {ret.push((r, c));},
                '^' => if r < row {ret.push((r, c));},
                '>' => if c > col {ret.push((r, c));},
                '<' => if c < col {ret.push((r, c));},
                _ => (),
            }

        }
    }

    let mut i = 0;
    while i < ret.len() {
        if visited.contains(&ret[i]) {
            ret.remove(i);
        } else {
            i += 1;
        }
    }

    ret
}


fn part1(data: &Vec<&str>) -> i64 {
    let mut total = 0;
    let mut map: Vec<Vec<Point>> = vec![];

    for line in data {
        let mut v = vec![];
        for c in line.chars() {
            v.push(C(c));
        }
        map.push(v);
    }

    let mut start = 0;
    for (i, each) in map[0].iter().enumerate() {
        match each {
            C(c) => {
                if *c == '.' {
                    start = i;
                    break;
                }},
            _ => (),
        }
    }

    println!("start is row 0 col {}", start);

    print_map(&map);
    total = get_steps_from_goal(map, 0, start, false);

    total
}

fn get_steps_from_goal(mut map: Vec<Vec<Point>>, row: usize, col: usize, walk_all: bool) -> i64 {
    map[row][col] = I(0);
    //print_map(&map);

    if row == map.len()-1 {
        return 0;
    }

    let mut max_steps = -10000;
    for (next_row, next_col) in possible_paths(&map, row, col, walk_all) {
        let steps = get_steps_from_goal(map.clone(), next_row, next_col, walk_all) + 1;
        
        if steps > max_steps {
            max_steps = steps;
        }
    }
    max_steps
}

#[derive(Clone)]
enum Point{
    C(char),
    I(i64),
}


fn possible_paths(map: &Vec<Vec<Point>>, row: usize, col: usize, walk_all: bool) -> Vec<(usize, usize)> {
    let mut ret = vec![];
    let mut pos = vec![];

    if row > 0 {
        pos.push((row - 1, col));
    }
    if row < map.len() - 1 {
        pos.push((row + 1, col));
    }
    if col > 0 {
        pos.push((row, col - 1));
    }
    if col < map[0].len() - 1 {
        pos.push((row, col + 1));
    }
    for (r, c) in pos {
        if let C(ch) = map[r][c] {
            //println!("Found {ch}");
            match ch {
                '.' => ret.push((r, c)),
                'v' => if r > row || walk_all {ret.push((r, c));},
                '^' => if r < row || walk_all {ret.push((r, c));},
                '>' => if c > col || walk_all {ret.push((r, c));},
                '<' => if c < col || walk_all {ret.push((r, c));},
                _ => (),
            }

        }
    }
    ret
}


fn print_map(map: &Vec<Vec<Point>>) {
    for line in map {
        for each in line {
            match each {
                I(i) => print!("{i}"),
                C(c) => print!("{c}"),
            };
        }
        println!("");
    }
}

