mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;

/// Solve function
///
/// Takes 2 arguments, day and part.
pub fn solve(day: u32, part: u32) -> i64 {
    /* Getting both example data and the real input. */
    let filename          = format!("../inputs/d{:02}.txt", day);
    let example_filename  = format!("../inputs/examples/d{:02}.txt", day);
    
    /* Input as string */
    let input: String   = std::fs::read_to_string(&filename).expect("file not found!");
    let example: String = std::fs::read_to_string(&example_filename).expect("file not found!");

    /* Input as a vector of strings */
    let input_vec   = string_to_str_vec(&input);
    let example_vec = string_to_str_vec(&example);

    /* Input as a vector of integers */
    let input_int_vec   = string_to_int_vec(&input);
    let example_int_vec = string_to_int_vec(&example);

    let result = match day {
        1  => day01::solve(part, &input_vec, &example_vec),
        2  => day02::solve(part, &input_vec, &example_vec),
        3  => day03::solve(part, &input_vec, &example_vec),
        4  => day04::solve(part, &input, &example),
        5  => day05::solve(part, &input_vec, &example_vec),
        //5  => day05::solve(part, &input, &example),
        6  => day06::solve(part, &input, &example),
        7  => day07::solve(part, &input, &example),
        8  => day08::solve(part, &input, &example),
        9  => day09::solve(part, &input, &example),
        10 => day10::solve(part, &input_int_vec, &example_int_vec),
        11 => day11::solve(part, &input, &example),
        12 => day12::solve(part, &input_vec, &example_vec),
        13 => day13::solve(part, &input, &example),
        _ => panic!("Unknown day, please input valid day"),
    };

    result
}

fn string_to_int_vec(input: &String) -> Vec<i64> {
    input.lines().filter_map(|s| match s.parse::<i64>() {
        Ok(i) => Some(i),
        _     => None,
    }).collect()
}

fn string_to_str_vec(input: &String) -> Vec<&str> {
    input.lines().filter_map(|s| match s {
        ""=>None,
        _=>Some(s),}).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day01() {
        assert_eq!(solve(1, 1), 969024);
        assert_eq!(solve(1, 2), 230057040);
    }
    #[test]
    fn test_day02() {
        assert_eq!(solve(2, 1), 422);
        assert_eq!(solve(2, 2), 451);
    }
    #[test]
    fn test_day03() {
        assert_eq!(solve(3, 1), 195);
        assert_eq!(solve(3, 2), 3772314000);
    }
    #[test]
    fn test_day04() {
        assert_eq!(solve(4, 1), 226);
        assert_eq!(solve(4, 2), 160);
    }
    #[test]
    fn test_day05() {
        assert_eq!(solve(5, 1), 974);
        assert_eq!(solve(5, 2), 646);
    }
    #[test]
    fn test_day06() {
        assert_eq!(solve(6, 1), 6170);
        assert_eq!(solve(6, 2), 2947);
    }
    #[test]
    fn test_day07() {
        assert_eq!(solve(7, 1), 242);
        assert_eq!(solve(7, 2), 176035);
    }
    #[test]
    fn test_day08() {
        assert_eq!(solve(8, 1), 1928);
        assert_eq!(solve(8, 2), 1319);
    }
    #[test]
    fn test_day09() {
        assert_eq!(solve(9, 1), 542529149);
        assert_eq!(solve(9, 2), 75678618);
    }
    #[test]
    fn test_day10() {
        assert_eq!(solve(10, 1), 2470);
        assert_eq!(solve(10, 2), 1973822685184);
    }
    #[test]
    fn test_day11() {
        assert_eq!(solve(11, 1), 2346);
        assert_eq!(solve(11, 2), 2111);
    }
    #[test]
    fn test_day12() {
        assert_eq!(solve(12, 1), 590);
        assert_eq!(solve(12, 2), 42013);
    }
    #[test]
    fn test_day13() {
        assert_eq!(solve(13, 1), 156);
        assert_eq!(solve(13, 2), 404517869995362);
    }
}
