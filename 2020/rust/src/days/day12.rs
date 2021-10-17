pub fn solve(part: u32, input: &Vec<&str>, sample: &Vec<&str>) -> i64 {
    assert_eq!(part1(sample), 25, "Error, sample problem is not solved");
    assert_eq!(part2(sample), 286, "Error, sample problem is not solved");
    match part {
        1 => part1(input),
        2 => part2(input),
        _ => panic!("Illegal value as input. Valid inputs are 1 and 2, recieved {}", part),
    }
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    North,
    West,
    South,
    East,
}

fn part1(instruction: &Vec<&str>) -> i64 {
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut current_direction: Direction = Direction::East;

    for each in instruction {
        if &each[..1] == "F" {
            match current_direction {
                Direction::North => y += &each[1..].parse::<i64>().unwrap(),
                Direction::South => y -= &each[1..].parse::<i64>().unwrap(),
                Direction::East  => x += &each[1..].parse::<i64>().unwrap(),
                Direction::West  => x -= &each[1..].parse::<i64>().unwrap(),
            };
        } else if &each[..1] == "N" {
            y += &each[1..].parse::<i64>().unwrap();
        } else if &each[..1] == "S" {
            y -= &each[1..].parse::<i64>().unwrap();
        } else if &each[..1] == "E" {
            x += &each[1..].parse::<i64>().unwrap();
        } else if &each[..1] == "W" {
            x -= &each[1..].parse::<i64>().unwrap();
        } else if &each[..1] == "R" {
            current_direction = rotate(true, current_direction, &each[1..]);
        } else if &each[..1] == "L" {
            current_direction = rotate(false, current_direction, &each[1..]);
        } else {
            panic!("Cant read command");
        }
    }

    x.abs() + y.abs()
}

struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn rotate(&mut self, clk_wise: bool, degrees: i64) {
        let rotation_const = if clk_wise {1} else {-1};
        let mut deg = degrees;

        while deg > 0 {
            let temp = self.y;
            self.y = -rotation_const * self.x;
            self.x = rotation_const * temp;
            deg -= 90;
        }
    }
}

fn part2(instruction: &Vec<&str>) -> i64 {
    let mut ship = Point {x: 0, y:0};
    let mut waypoint = Point {x: 10, y:1};

    for each in instruction {
        let value = &each[1..].parse::<i64>().unwrap();
        if &each[..1] == "F" {
            ship.x += waypoint.x * value;
            ship.y += waypoint.y * value;
        } else if &each[..1] == "N" {
            waypoint.y += value;
        } else if &each[..1] == "S" {
            waypoint.y -= value;
        } else if &each[..1] == "E" {
            waypoint.x += value;
        } else if &each[..1] == "W" {
            waypoint.x -= value;
        } else if &each[..1] == "R" {
            waypoint.rotate(true, *value);
        } else if &each[..1] == "L" {
            waypoint.rotate(false, *value);
        } else {
            panic!("Cant read command");
        }
    }

    ship.x.abs() + ship.y.abs()
}

fn rotate(clk_wise: bool, current: Direction, input: &str) -> Direction {
    let mut deg: i64 = input.parse().expect("Error parsing rotation degrees");
    let mut new_direction = current;

    while deg > 0 {
        new_direction = match new_direction {
            Direction::North => if clk_wise {Direction::East}  else {Direction::West},
            Direction::East  => if clk_wise {Direction::South} else {Direction::North},
            Direction::South => if clk_wise {Direction::West}  else {Direction::East},
            Direction::West  => if clk_wise {Direction::North} else {Direction::South},
        };
        deg -= 90;
    }
    new_direction
}

