use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, Lines};

pub fn solve(lines: &mut Lines<BufReader<File>>, _: bool) {
    for line in lines {
        let idx = check_line(&line.expect("Could not read line"));

        println!("\tSignal starts at: {}", idx)
    }
}

fn check_line(line: &String) -> usize {
    let mut set = Vec::<char>::new();

    let mut i = 0;

    for c in line.chars() {
        if add(&mut set, c) {
            return i + 1
        }

        i += 1;
    }

    return 0
}

fn add(v: &mut Vec::<char>, c: char) -> bool {
    v.push(c);

    if v.len() == 14 + 1 {
        v.remove(0);
    } else {
        return false
    }

    let mut set = HashSet::<char>::new();

    for i in 0..v.len() {
        if !set.insert(v[i]) {
            return false
        }
    }

    return true
}

