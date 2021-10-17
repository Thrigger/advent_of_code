pub fn solve(part: u32, input: &str, sample: &str) -> i64 {
    assert_eq!(part1(sample), 37, "Error, sample problem is not solved");
    assert_eq!(part2(sample), 26, "Error, sample problem is not solved");
    match part {
        1 => part1(input),
        2 => part2(input),
        _ => panic!("Illegal value as input. Valid inputs are 1 and 2, recieved {}", part),
    }
}

fn part1(data: &str) -> i64 {
    let mut seats = data.split("\n")
                        .filter_map(|x| return_line(x))
                        .map(|s| s.chars().collect::<Vec<char>>())
                        .collect::<Vec<Vec<char>>>();
    let mut seat_changes;

    loop {
        seat_changes = seat_people(&mut seats);
        if seat_changes == 0 {
            break;
        }
    }
    count_occupied(&seats)
}

fn part2(data: &str) -> i64 {
    let mut seats = data.split("\n")
                        .filter_map(|x| return_line(x))
                        .map(|s| s.chars().collect::<Vec<char>>())
                        .collect::<Vec<Vec<char>>>();
    let mut seat_changes;

    loop {
        seat_changes = seat_people_part2(&mut seats);
        if seat_changes == 0 {
            break;
        }
    }
    count_occupied(&seats)
}

fn return_line(s: &str) -> Option<&str> {
    match s {
        "" => None,
        _ => Some(&s),
    }
}

fn count_occupied(seats: &Vec<Vec<char>>) -> i64 {
    let mut occupied = 0;

    for row in seats {
        for seat in row {
            if *seat == '#' {
                occupied += 1;
            }
        }
    }
    occupied
}

fn any_neighbors(row: usize, col: usize, dir: i64, seats: &Vec<Vec<char>>) -> i64 {
    let is_possible_seat = match dir {
        1 => row > 0 && col > 0,
        2 => row > 0,
        3 => row > 0 && col < seats[0].len() - 1,
        4 => col > 0,
        6 => col < seats[0].len() - 1,
        7 => row < seats.len() - 1 && col > 0,
        8 => row < seats.len() - 1,
        9 => row < seats.len() - 1 && col < seats[0].len() - 1,
        _ => false,
    };

    if is_possible_seat {
        let (next_row, next_col) = match dir {
            1 => (row - 1, col - 1), 
            2 => (row-1,col),
            3 => (row-1,col+1),
            4 => (row,col-1),
            6 => (row, col+1),
            7 => (row+1,col-1),
            8 => (row+1,col),
            9 => (row+1,col+1),
            _ => (0,0),
        };
        if seats[next_row][next_col] == '#' {
            return 1;
        } else if seats[next_row][next_col] == 'L' {
            return 0;
        } else {
            return any_neighbors(next_row, next_col, dir, &seats);
        }

    } 
    /* If condition is false we have reach the end of the seats and therfore no neighbours */
    0
}

fn seat_people_part2(seats: &mut Vec<Vec<char>>) -> i64 {
    /* Naming the seat directions:
     * 1 2 3
     * 4 X 6
     * 7 8 9
     * to make it easier to read the code.
     */
    let mut nof_changes: i64 = 0;
    let mut new_seats = seats.clone();

    for row in 0..seats.len() {
        for col in 0..seats[0].len() {
            let mut nof_neighbors = 0;
            let directions = vec![1, 2, 3, 4, 6, 7, 8, 9];
            for dir in directions {
                nof_neighbors += any_neighbors(row, col, dir, &seats);
            }
            if nof_neighbors >= 5 && new_seats[row][col] == '#' {
                new_seats[row][col] = 'L';
                nof_changes += 1;
            } else if nof_neighbors == 0 && new_seats[row][col] == 'L' {
                new_seats[row][col] = '#';
                nof_changes += 1;
            }
        }
    }
    
    *seats = new_seats.clone();
    nof_changes
}


fn seat_people(seats: &mut Vec<Vec<char>>) -> i64 {
    let mut nof_changes: i64 = 0;
    let mut new_seats = seats.clone();

    for row in 0..seats.len() {
        for col in 0..seats[0].len() {
            if seats[row][col] != '.' {
                let mut nof_neighbors = 0;

                if row > 0 {
                    if col > 0 && seats[row-1][col-1]=='#' {
                        nof_neighbors += 1;
                    } 
                    if col < seats[0].len() - 1 && seats[row-1][col+1]=='#' {
                        nof_neighbors += 1;
                    } 
                    if seats[row-1][col]=='#' {
                        nof_neighbors += 1;
                    }
                }
                if row < seats.len() - 1 {
                    if col > 0 && seats[row+1][col-1]=='#' {
                        nof_neighbors += 1;
                    } 
                    if col < seats[0].len() - 1 && seats[row+1][col+1]=='#' {
                        nof_neighbors += 1;
                    } 
                    if seats[row+1][col]=='#' {
                        nof_neighbors += 1;
                    }
                }
                if col > 0 && seats[row][col-1]=='#' {
                    nof_neighbors += 1;
                }
                if col < seats[0].len() - 1 && seats[row][col+1]=='#' {
                    nof_neighbors += 1;
                }

                if nof_neighbors >= 4 && new_seats[row][col] == '#' {
                    new_seats[row][col] = 'L';
                    nof_changes += 1;
                } else if nof_neighbors == 0 && new_seats[row][col] == 'L' {
                    new_seats[row][col] = '#';
                    nof_changes += 1;
                }
            }
        }
    }
    
    *seats = new_seats.clone();
    nof_changes
}

