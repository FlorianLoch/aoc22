use std::fs::File;
use std::io::{BufReader, Lines};
use std::ops::Range;

pub fn solve(lines: &mut Lines<BufReader<File>>) {
    let mut sum = 0;

    for line in lines {
        let s = line.expect("Failed to read line");
        let ranges: Vec<&str> = s.split(",").collect();

        let r1 = parse_range(ranges[0]);
        let r2 = parse_range(ranges[1]);

        sum += if does_range_contain_range(&r1, &r2) || does_range_contain_range(&r2, &r1) {
            1
        } else {
            0
        }
    }

    println!("\tSum: {}", sum)
}

fn does_range_contain_range(r1: &Range::<u32>, r2: &Range::<u32>) -> bool {
    // r1.start <= r2.start && r1.end >= r2.end

    r1.start <= r2.start && r1.end >= r2.start
}

fn parse_range(s: &str) -> Range<u32> {
    let parts: Vec<&str> = s.split("-").collect();

    let start = parts[0].trim().parse().expect("Failed to parse start of range");
    let end: u32 = parts[1].trim().parse().expect("Failed to parse start of range");

    return start..end;
}
