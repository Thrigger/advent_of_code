use thrigger_support::point::Point;

pub fn solve(part: u32, input: &Vec<&str>, sample: &Vec<&str>) -> i64 {
    assert_eq!(part1(&sample), 31, "Error, sample problem is not solved");
    //assert_eq!(part2(&sample), 0, "Error, sample problem is not solved");

    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => panic!("Illegal value as input. Valid inputs are 1 and 2, recieved {}", part),
    }
}

fn part2(data: &Vec<&str>) -> i64 {
    let mut total = 0;
    //data.iter().map(|i| fuel2(*i)).sum();
    total
}

fn part1(data: &Vec<&str>) -> i64 {
    let mut total = 0;

    let mut hill_map = parse_map(&data);
    print_map(&hill_map);
    println!("map {}x{}", hill_map[0].len(), hill_map.len());

    let mut paths: Vec<Path> = vec![];
    paths.push(Path {cost: 0, points: vec![Point::new(0, 0)]});

    let mut lowest_so_far = 0;
    loop {
        // Find the path with the lowest current cost
        paths.sort_by_key(|k| k.cost);

        let lowest_cost_path = paths[0].clone();
        let last_point = &lowest_cost_path.points.last().unwrap();
        //println!("New itteration:\tCost {} - {}", paths[0].cost, last_point.to_string());

        let neighbors = get_neighbors(&last_point, hill_map[0].len(), hill_map.len());
        for each in neighbors {
            if !lowest_cost_path.points.contains(&each) {

                //println!("Neighbor: {}", each.to_string());
                let height = hill_map[each.y as usize][each.x as usize];
                //println!("Neighbor: {} h: {}", each.to_string(), height);
                if height <= hill_map[last_point.y as usize][last_point.x as usize] + 1  {
                    //println!("Neigbour added");
                    // TODO have i allready come here but in a cheaper path?

                    let mut new_path = lowest_cost_path.clone();

                    new_path.points.push(each.clone());
                    new_path.cost += 1;
                    
                    if new_path.cost > lowest_so_far {
                        lowest_so_far = new_path.cost;
                        println!("lowest {}",lowest_so_far);
                    }
                    if height == 25 {
                        for p in &new_path.points {
                            print!("{} ", p.to_string());
                        }
                        total = new_path.points.len() as i64;
                        break;
                    }
                    paths.push(new_path);
                }
            }
        }

        paths.remove(0);
        //  create a clone path for each path/point that is not yet in the path

        //println!("");
        if total > 0 {
            break;
        }
    }
    
    total
}

fn get_neighbors(current: &Point, x_max: usize, y_max: usize) -> Vec<Point> {
    let mut neighbors = vec![];

    if current.x > 0 {
        let mut neighbor = Point::new(current.x-1, current.y);
        neighbors.push(neighbor);
    }
    if current.y > 0 {
        let mut neighbor = Point::new(current.x, current.y-1);
        neighbors.push(neighbor);
    }
    if current.x < (x_max - 1) as i64 {
        let mut neighbor = Point::new(current.x+1, current.y);
        neighbors.push(neighbor);
    }
    if current.y < (y_max - 1) as i64 {
        let mut neighbor = Point::new(current.x, current.y+1);
        neighbors.push(neighbor);
    }
    neighbors
}

fn get_path_with_lowest_cost(paths: &mut Vec<Path>) -> &mut Path {
    let mut i = 0;
    let mut min_cost = -1;

    &mut paths[0]
}

#[derive(Clone)]
struct Path {
    cost: i64,
    points: Vec<Point>,
}

fn convert_height(c: char) -> i64 {
    let mut val = c as i64 - 97;
    val = match val {
        -14 => 0,
        -28 => 'z' as i64 - 97,
        _ => val,
    };
    val
}

fn parse_map(data: &Vec<&str>) -> Vec<Vec<i64>> {
    let mut hill_map = vec![];
    for line in data {
        let values = line.chars().map(|c| convert_height(c)).collect::<Vec<i64>>();
        hill_map.push(values);
    }
    hill_map
}

fn print_map(hill_map: &Vec<Vec<i64>>) {
    println!("Map:");
    for line in hill_map {
        println!("{:?}", line);
    }
    println!("");
}


