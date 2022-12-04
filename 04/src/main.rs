use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::ops::Range;

fn main() {
    let mut _sum = 0;

    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(l) = line {
                _sum += handle_line(l)
            }
        }
    }

    println!("Total score {}", _sum)
}

fn handle_line(line: String) -> i32 {
    let ranges: Vec<&str> = line.split(",").collect();

    let r1 = parse_range(ranges[0]);
    let r2 = parse_range(ranges[1]);

    if does_range_contain_range(&r1, &r2) || does_range_contain_range(&r2, &r1) {
        return 1;
    }

    return 0;
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

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
