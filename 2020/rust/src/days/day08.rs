pub fn solve(part: u32, input: &str, sample: &str) -> i64 {
    assert_eq!(part1(&sample), 5, "Error, sample problem is not solved");
    assert_eq!(part2(&sample), 8, "Error, sample problem is not solved");
    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => panic!("Illegal value as input. Valid inputs are 1 and 2, recieved {}", part),
    }
}

fn acc(sum: &mut i64, arg: &str) {
    *sum += arg.parse::<i64>().unwrap();
}

fn jmp(pc: usize, arg: &str) -> usize{
    let pc_int: i64 = pc as i64 + arg.parse::<i64>().unwrap();
    pc_int as usize
}

fn app_loop (app: Vec<&str>) -> Result<i64, i64> {
    let mut accumulator: i64 = 0;
    let mut pc = 0;

    /* This is not nice but I know my input is less than 1k lines */
    let mut inst_visited = [0; 1000];
    
    while pc < app.len() {
        inst_visited[pc] += 1;
        if inst_visited[pc] > 1 {
            break;
        }
        match app[pc].get(..3).unwrap() {
            "acc" => {acc(&mut accumulator, app[pc].get(4..).unwrap()); pc += 1},
            "jmp" => pc = jmp(pc, app[pc].get(4..).unwrap()),
            "nop" => pc += 1,
            _=>{},
        };
    }

    /* Did we find a solution? */
    if pc == app.len() {
        Ok(accumulator)
    } else {
        Err(accumulator)
    }
}

fn part2(data: &str) -> i64 {
    let app: Vec<&str> = data.lines().filter(|s| !s.is_empty()).collect();
    let mut i = 0;
    let mut nof_changed_lines = 0;
    let mut nof_skipped_jmpnop = 0;

    while i < app.len() {
        let mut temp_app: Vec<&str> = app.clone();
        let curr_inst = app[i].get(..3).unwrap();
        let curr_arg = app[i].get(3..).unwrap();
        
        if curr_inst == "jmp" || curr_inst == "nop" {
            if nof_skipped_jmpnop == nof_changed_lines {
                /* switch this line */ 
                let new_line = match curr_inst {
                    "jmp" => format!("nop{}", curr_arg),
                    _     => format!("jmp{}", curr_arg),
                };
                temp_app[i] = &new_line[..];
                match app_loop(temp_app) {
                    Ok(i)  => return i,
                    Err(_) => println!("Error"),
                };

                nof_changed_lines += 1;
                nof_skipped_jmpnop = 0;
                i = 0;
            } else {
                nof_skipped_jmpnop += 1;
                i += 1;
            }
        } else {
            i += 1;
        }
    }

    // Error, function should return early on success
    0
}

fn part1(data: &str) -> i64 {
    let app: Vec<&str> = data.lines().filter(|s| !s.is_empty()).collect();

    match app_loop(app) {
        /* In part 1 it should return Err */
        Ok(_) => 0,
        Err(i) => i,
    }
}

