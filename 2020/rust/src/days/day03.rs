pub fn solve(part: u32, input: &Vec<&str>, sample: &Vec<&str>) -> i64 {
    assert_eq!(part1(&sample), 7, "Error, sample problem is not solved");
    assert_eq!(part2(&sample), 336, "Error, sample problem is not solved");
    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => panic!("Illegal value as input. Valid inputs are 1 and 2, recieved {}", part),
    }
}

fn part1(data: &Vec<&str>) -> i64 {
    trees_on_path(&data, 3, 1)
}
 
fn part2(data: &Vec<&str>) -> i64 {
    let mut number_of_trees: [i64; 5] = [0; 5];
    number_of_trees[0] = trees_on_path(&data, 1, 1);
    number_of_trees[1] = trees_on_path(&data, 3, 1);
    number_of_trees[2] = trees_on_path(&data, 5, 1);
    number_of_trees[3] = trees_on_path(&data, 7, 1);
    number_of_trees[4] = trees_on_path(&data, 1, 2);

    let mut sum = 1;
    for each in number_of_trees.iter() {
        sum *= *each as i64;
    }
    sum
}

fn trees_on_path(data: &Vec<&str>, x_speed: usize, y_speed: usize) -> i64 {
    let mut x_pos = 0;
    let mut trees = 0;
    let mut i = 0;
    while i < data.len() {
        if data[i].chars().collect::<Vec<char>>()[x_pos] == '#' {
            trees += 1;
        }
        x_pos = (x_pos + x_speed) % data[i].len();

        i += y_speed;
    }
    trees
}
  
