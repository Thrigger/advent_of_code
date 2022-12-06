mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

/// Solve function
///
/// Takes 2 arguments, day and part.
pub fn solve(day: u32, part: u32) -> i64 {
    /* Getting the input. */
    let filename = format!("../inputs/d{:02}.txt", day);
    let ex_filename = format!("../inputs/examples/d{:02}.txt", day);
    
    /* Input as string */
    let input_raw: String =
        std::fs::read_to_string(&filename).expect("file not found!");
    let ex_input_raw: String =
        std::fs::read_to_string(&ex_filename).expect("file not found!");

    /* Trim trailing line */
    let input_trimmed = input_raw.trim_end();
    let ex_input_trimmed = ex_input_raw.trim_end();

    /* Input as a vector of strings */
    let input_strs = string_to_str_vec(&input_trimmed);
    let ex_input_strs = string_to_str_vec(&ex_input_trimmed);

    /* Input as a vector of integers */
    let input_ints = string_to_int_vec(&input_trimmed);
    let ex_input_ints = string_to_int_vec(&ex_input_trimmed);

    /* Input as string groups */
    let input_str_group = string_to_str_groups(&input_trimmed);
    let ex_input_str_group = string_to_str_groups(&ex_input_trimmed);

    let result = match day {
        1  => day01::solve(part, &input_str_group, &ex_input_str_group),
        2  => day02::solve(part, &input_strs, &ex_input_strs),
        3  => day03::solve(part, &input_strs, &ex_input_strs),
        4  => day04::solve(part, &input_strs, &ex_input_strs),
        5  => day05::solve(part, &input_str_group, &ex_input_str_group),
        6  => day06::solve(part, &input_strs, &ex_input_strs),
        7  => day07::solve(part, &input_ints, &ex_input_ints),
        //7  => day07::solve(part, &input_strs, &ex_input_strs),
        //7  => day07::solve(part, &input_ints, &ex_input_ints),
        //5  => day04::solve(part, &input_strs, &ex_input_strs),
        //5  => day04::solve(part, &input_str_group, &ex_input_str_group),
        //5  => day04::solve(part, &input_str_group, &ex_input_str_group),
        _ => panic!("Unknown day, please input valid day"),
    };

    result
}

fn string_to_int_vec(input: &str) -> Vec<i64> {
    input.lines().filter_map(|s| match s.parse::<i64>() {
        Ok(i) => Some(i),
        _     => None,
    }).collect()
}

fn string_to_str_vec(input: &str) -> Vec<&str> {
    input.lines().filter_map(|s| match s {
        ""=>None,
        _=>Some(s),}).collect()
}

fn string_to_str_groups(input: &str) -> Vec<&str> {
    input.split("\n\n").collect::<Vec<&str>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day06() {
        assert_eq!(solve(6, 1), 1578);
        assert_eq!(solve(6, 2), 2178);
    }

    #[test]
    fn test_day05() {
        /* Output is a printed string */
        assert_eq!(solve(5, 1), 0);
        assert_eq!(solve(5, 2), 0);
    }

    #[test]
    fn test_day04() {
        assert_eq!(solve(4, 1), 657);
        assert_eq!(solve(4, 2), 938);
    }

    #[test]
    fn test_day03() {
        assert_eq!(solve(3, 1), 8493);
        assert_eq!(solve(3, 2), 2552);
    }

    #[test]
    fn test_day02() {
        assert_eq!(solve(2, 1), 12679);
        assert_eq!(solve(2, 2), 14470);
    }

    #[test]
    fn test_day01() {
        assert_eq!(solve(1, 1), 65912);
        assert_eq!(solve(1, 2), 195625);
    }
}

