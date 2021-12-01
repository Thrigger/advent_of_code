mod day01;

/// Solve function
///
/// Takes 2 arguments, day and part.
pub fn solve(day: u32, part: u32) -> i64 {
    /* Getting the input. */
    let filename = format!("../inputs/d{:02}.txt", day);
    let ex_filename = format!("../inputs/examples/d{:02}.txt", day);
    
    /* Input as string */
    let input_raw: String = std::fs::read_to_string(&filename).expect("file not found!");
    let ex_input_raw: String = std::fs::read_to_string(&ex_filename).expect("file not found!");

    /* Input as a vector of strings */
    let input_strs = string_to_str_vec(&input_raw);
    let ex_input_strs = string_to_str_vec(&ex_input_raw);

    /* Input as a vector of integers */
    let input_ints = string_to_int_vec(&input_raw);
    let ex_input_ints = string_to_int_vec(&ex_input_raw);

    let result = match day {
        1  => day01::solve(part, &input_ints, &ex_input_ints),
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

fn string_to_str_group_vec(input: &String) -> Vec<&str> {
    input.split("\n\n").collect::<Vec<&str>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day01() {
        assert_eq!(solve(1, 1), 1448);
        assert_eq!(solve(1, 2), 1471);
    }
}
