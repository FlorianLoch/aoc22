use std::collections::{HashMap, HashSet};
use crate::puzzle;

pub struct Day03 {}

impl puzzle::Puzzle for Day03 {
    fn group_n_lines(&self) -> usize {
        3
    }

    fn solve(&self, lines: &Vec<String>) -> i32 {
        return priority(find_group_id(lines));
    }
}

fn find_group_id(lines: &Vec<String>) -> char {
    let mut map = HashMap::<char, i8>::new();

    for line in lines {
        let mut per_elf_map = HashSet::<char>::new();

        for c in line.chars() {
            if !per_elf_map.insert(c) {
                continue;
            }

            let val = map.get(&c);

            if val.is_some() {
                if *val.unwrap() == 2 {
                    return c;
                }

                *map.get_mut(&c).unwrap() += 1;

                continue;
            }

            map.insert(c, 1);
        }
    }

    panic!("cannot find group id")
}

fn find_misplaced(line: String) -> char {
    let mut compartment1 = HashSet::<char>::new();

    let half = line.len() / 2;
    let mut i = 0;

    for c in line.chars() {
        if i < half {
            compartment1.insert(c);
        } else {
            if compartment1.contains(&c) {
                return c
            }
        }

        i += 1;
    }

    panic!("no duplicate found")
}

fn priority(c: char) -> i32 {
    let i = c as u32;

    if i >= 97 && i <= 122 {
        return (i - 96).try_into().unwrap();
    }

    if i >= 65 && i <= 90 {
        return (i - 65 + 27).try_into().unwrap();
    }

    panic!("invalid character given")
}