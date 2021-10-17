pub fn solve(part: u32, input: &Vec<&str>, sample: &Vec<&str>) -> i64 {
    assert_eq!(part1(&sample), 357, "Error, sample problem is not solved");
    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => panic!("Illegal value as input. Valid inputs are 1 and 2, recieved {}", part),
    }
}

struct Seat {
    row: i64,
    col: i64
}

fn get_seat_id (input: &Seat) -> i64 {
    input.row * 8 + input.col
}

fn find_row(input: &str) -> i64 {
    let mut row_lo: i64 = 0;
    let mut row_hi: i64 = 127;
    for each_char in input.chars() {
        if each_char == 'F' {
            row_hi -= (row_hi-row_lo+1)/2;
        } else if each_char == 'B' {
            row_lo += (row_hi-row_lo+1)/2;
        }
    }

    row_lo
}

fn find_col(input: &str) -> i64 {
    let mut col_lo: i64 = 0;
    let mut col_hi: i64 = 7;
    for each_char in input.chars() {
        if each_char == 'L' {
            col_hi -= (col_hi-col_lo+1)/2;
        } else if each_char == 'R' {
            col_lo += (col_hi-col_lo+1)/2;
        }
    }

    col_lo
}

fn part1(data: &Vec<&str>) -> i64 {
    let mut max_id = 0;

    for line in data {
        let current_seat = Seat {row: find_row(&line[..7]), col: find_col(&line[7..])};
        
        let seat_id = get_seat_id(&current_seat);

        if seat_id >= max_id {
            max_id = seat_id;
        }
    }
    max_id
}

//fn part2(data: &str) -> i64 {
fn part2(data: &Vec<&str>) -> i64 {
    let mut max_id = 0;
    let mut airplane = [[false; 8]; 128];

    for line in data {
        let current_seat = Seat {row: find_row(&line[..7]), col: find_col(&line[7..])};
        
        airplane[current_seat.row as usize][current_seat.col as usize] = true;
        let seat_id = get_seat_id(&current_seat);

        if seat_id >= max_id {
            max_id = seat_id;
        }
    }

    /* Find my spot */
    let mut start = true;
    let mut my_seat = 0;
    // looping rows
    for i in 0..airplane.len() {
        // looping cols
        for j in 0..airplane[0].len() {
            if start == true && airplane[i][j] == true {
                start = false;
            } else if start == false && airplane[i][j]  == false {
                my_seat = get_seat_id(&Seat{row: i as i64, col: j as i64});
                break;
            }
        }
        if my_seat != 0 {
            break;
        }
    }
    my_seat
}

