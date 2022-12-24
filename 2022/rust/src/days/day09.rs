use std::collections::HashMap;
use thrigger_support::point::Point;

pub fn solve(part: u32, input: &Vec<&str>, sample: &Vec<&str>) -> i64 {
    assert_eq!(part1(&sample), 13, "Error, sample problem is not solved");
    assert_eq!(part2(&sample), 1, "Error, sample problem is not solved");

    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => panic!("Illegal value as input. Valid inputs are 1 and 2, recieved {}", part),
    }
}

fn part1_2(data: &Vec<&str>, len: usize) -> i64 {
    let mut points: Vec<Point> = vec![];
    for _ in 0..len {
        points.push(Point::new_origin());
    }

    let mut visited = HashMap::new();

    for line in data {
        let parts: Vec<&str> = line.split(" ").collect();
        let mut dir: &str = parts[0];
        let val: i64 = parts[1].parse().unwrap();

        for _ in 0..val {
            match dir {
                "R" => points[0].x += 1,
                "L" => points[0].x -= 1,
                "U" => points[0].y += 1,
                "D" => points[0].y -= 1,
                &_ => (),
            };
            for i in 0..points.len() {
                if i+1 < points.len() {
                    let dx = points[i].x-points[i+1].x;
                    let dy = points[i].y-points[i+1].y;

                    if i64::abs(dx) > 1 || i64::abs(dy) > 1 {
                        points[i+1].x = points[i].x;
                        points[i+1].y = points[i].y;

                        if i64::abs(dx) > 1 {
                            points[i+1].x -= dx/i64::abs(dx);
                        } 
                        if i64::abs(dy) > 1 {
                            points[i+1].y -= dy/i64::abs(dy);
                        }
                    }
                } else if i == points.len() -1 {
                    visited.insert(points[i].to_string(), true);
                }
            }
        }
    }
    visited.drain().count().try_into().unwrap()
}

fn part2(data: &Vec<&str>) -> i64 {
    part1_2(data, 10)
}

fn part1(data: &Vec<&str>) -> i64 {
    part1_2(data, 2)
}
