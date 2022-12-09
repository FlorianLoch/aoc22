use std::collections::{HashMap};
use std::fs::File;
use std::io::{BufReader, Lines};

pub fn solve(lines: &mut Lines<BufReader<File>>) {
    let mut dir_map = HashMap::<String, u32>::new();

    // First line is always `$ cd /`, drop it.
    let _ = lines.next().expect("There is no first line");

    process_dir(lines, String::from("/"), &mut dir_map);

    let mut total_size: u32 = 0;
    for (_, size) in &dir_map {
        if size <= &100_000 {
            total_size += size;
        }
    }

    println!("\tTotal size of directories <= 100_000: {}", total_size);

    const DEVICE_TOTAL_SIZE: u32 = 70_000_000;
    const SPACE_REQUIRED: u32 = 30_000_000;

    let free_space = DEVICE_TOTAL_SIZE - dir_map.get("/").expect("Root dir not found");


    let mut smallest_sufficient_size = 0;
    for (_, size) in &dir_map {
        if free_space + size >= SPACE_REQUIRED && (size < &smallest_sufficient_size || smallest_sufficient_size == 0) {
            smallest_sufficient_size = size.clone()
        }
    }

    println!("\tSize of smallest directory sufficient to delete: {}", smallest_sufficient_size);
}

fn process_dir(lines: &mut Lines<BufReader<File>>, cwd: String, dir_map: &mut HashMap<String, u32>) -> u32 {
    let mut cur_size: u32 = 0;

    loop {
        let l = match lines.next() {
            Some(l) => l.expect("Could not read next line"),
            None => break
        };

        if l.starts_with("$ cd") {
            let path = l.trim_start_matches("$ cd ");

            if path == ".." {
                break;
            }

            let mut new_cwd = cwd.clone();
            new_cwd.push_str(path);
            new_cwd.push('/');

            cur_size += process_dir(lines, new_cwd, dir_map);

            continue;
        }

        // We do not need to actually handle check for `$ ls`, nor for `dir` entries
        if l.starts_with("$") || l.starts_with("dir") {
            continue;
        }

        // Must be a file then
        let file_size: u32 = match l.split_whitespace().next() {
            None => panic!("File entry has no field"),
            Some(v) => v.to_owned().parse().expect("Cannot parse file size")
        };

        cur_size += file_size;
    }

    dir_map.insert(cwd, cur_size);

    return cur_size
}
