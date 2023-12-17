pub fn solve(part: u32, input: &Vec<&str>, sample: &Vec<&str>) -> i64 {
    assert_eq!(part1(&sample), 35, "Error, sample problem is not solved");
    //assert_eq!(part2(&sample), 46, "Error, sample problem is not solved");

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

    let mut vals: Vec<Range> = parse_seeds(&data[0]);

    for mapping in &data[1..] {
        vals = map_data(mapping, vals);
    }
    
    total
}

fn map_data(mapping: &str, vals: Vec<Range>) -> Vec<Range> {
    let mut new_vals = vec![];

    let lines = mapping.split("\n");
    for rule in lines.skip(1) {
        println!("New rule: {:?}", rule);
        let parts = rule.split(" ").map(|s| s.parse::<i64>()).collect();
        let dst = parts[0];
        let src = parts[1];
        let size = parts[2];
        let src_end = src + size - 1;
        
        for each in vals {
            if each.start >= src && each.start <= src_end {
                // start is in rule
                if each.get_end() <= src_end {
                    new_vals.push(Range::new(each.start - src + dst , each.len));
                } else {
                    new_vals.push(Range::new(each.start - src + dst , each.len));
                    new_vals.push(Range::new(each.start - src + dst , each.len));
                }
            } else if each.get_end() >= src && each.get_end() <= src_end {
                // end is in rule
            } else if src >= each.start && src <= each.get_end() {
                // rule start is within the intervall
            } else if src_end >= each.start && src_end <= each.get_end() {
                // rule end is within the intervall
            }
        }
    }
    new_vals
}

fn parse_seeds(input: &str) -> Vec<Range> {
    let parts: Vec<_> = input.split(": ").collect();
    let numbs: Vec<i64> = parts[1]
        .split(" ")
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    println!("seeds: {:?}", numbs);

    let mut out = vec![];
    let mut i = 0;
    while i < numbs.len() {
        out.push(Range::new(numbs[i], numbs[i+1]));
        i += 2;
    }
    out
}

struct Range {
    start: i64,
    len: i64,
}

impl Range {
    fn new(start: i64, len: i64) -> Range {
        Range {start, len}
    }

    fn get_end(&self) -> i64 {
        self.start + self.len - 1
    }

    /// Split with the input as start of second range
    fn split(self, new_start:i64) -> (Range, Range) {
        if new_start <= self.start {
            panic!("Split is wrong");
        }

        let len1 = new_start - self.start;
        let len2 = self.len - len1;

        (Range::new(self.start, len1), Range::new(new_start, len2)) 
    }

    fn print(&self) {
        println!("Range start: {} length: {}", self.start, self.len);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split() {
        let r = Range::new(2, 3); // 2 3 4
        let (r1, r2) = r.split(3);
        assert_eq!(r1.start, 2);
        assert_eq!(r2.start, 3);
        assert_eq!(r1.len, 1);
        assert_eq!(r2.len, 2);
    }
}


