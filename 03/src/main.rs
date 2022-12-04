use core::panic;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{HashSet, HashMap};

fn main() {
    let mut _sum = 0;

    // if let Ok(lines) = read_lines("./input") {
    //     for line in lines {
    //         if let Ok(l) = line {
    //             _sum += handle_line(l)
    //         }
    //     }
    // }

    let mut a = 0;

    if let Ok(lines) = read_lines("./input") {
        let mut list = Vec::<String>::new();

        for line in lines {
            if let Ok(l) = line {
                list.push(l);

                if a % 3 == 2 {
                    _sum += handle_3_lines(&list);

                    // println!("list: \n{:?}", list);

                    list = Vec::<String>::new();
                }
            }

            a += 1;
        }
    }

    println!("Total score {}", _sum)
}

// fn handle_line(line: String) -> i32 {
//     return priority(find_misplaced(line));
// }

fn handle_3_lines(lines: &Vec<String>) -> i32 {
    return priority(find_group_id(lines))
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

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
