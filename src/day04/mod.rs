use std::ops::Range;
use crate::puzzle;

pub struct Day04 {}

impl puzzle::Puzzle for Day04 {
    fn group_n_lines(&self) -> usize {
        1
    }

    fn solve(&self, lines: &Vec<String>) -> i32 {
        let ranges: Vec<&str> = lines[0].split(",").collect();

        let r1 = parse_range(ranges[0]);
        let r2 = parse_range(ranges[1]);

        if does_range_contain_range(&r1, &r2) || does_range_contain_range(&r2, &r1) {
            return 1;
        }

        return 0;
    }
}

fn does_range_contain_range(r1: &Range::<u32>, r2: &Range::<u32>) -> bool {
    // r1.start <= r2.start && r1.end >= r2.end

    r1.start <= r2.start && r1.end >= r2.start
}

fn parse_range(s: &str) -> Range<u32> {
    let parts: Vec<&str> = s.split("-").collect();

    let start = parts[0].trim().parse().expect("Failed to parse start of range");
    let end: u32 = parts[1].trim().parse().expect("Failed to parse start of range");

    return start..end
}
