mod day01;
mod day05;

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
        //1 => day01::solve(part, &input_ints, &ex_input_ints),
        //1 => day01::solve(part, &input_ints, &ex_input_ints),
        5 => day05::solve(part, &input_str_group, &ex_input_str_group),
        //11 => day11::solve(part, &input_str_group, &ex_input_str_group),
        //11 => day11::solve(part, &input_ints, &ex_input_ints),
        //11 => day11::solve(part, &input_ints, &ex_input_ints),
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
    fn test_day05() {
        assert_eq!(solve(5, 1), 51752125);
        //assert_eq!(solve(5, 2), 0);
    }
}

