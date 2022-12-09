use std::fs::File;
use std::io::{BufReader, Lines};

pub fn solve(lines: &mut Lines<BufReader<File>>) {
    let mut stacks = Vec::<Vec<char>>::new();

    // Test data
    // stacks.push(vec!['Z', 'N']);
    // stacks.push(vec!['M', 'C', 'D']);
    // stacks.push(vec!['P']);

    // Full data
    stacks.push(vec!['J', 'H', 'P', 'M', 'S', 'F', 'N', 'V']);
    stacks.push(vec!['S', 'R', 'L', 'M', 'J', 'D', 'Q']);
    stacks.push(vec!['N', 'Q', 'D', 'H', 'C', 'S', 'W', 'B']);
    stacks.push(vec!['R', 'S', 'C', 'L']);
    stacks.push(vec!['M', 'V', 'T', 'P', 'F', 'B']);
    stacks.push(vec!['T', 'R', 'Q', 'N', 'C']);
    stacks.push(vec!['G', 'V', 'R']);
    stacks.push(vec!['C', 'Z', 'S', 'P', 'D', 'L', 'R']);
    stacks.push(vec!['D', 'S', 'J', 'V', 'G', 'P', 'B', 'F']);

    let mut stack_lines = Vec::<String>::new();

    let mut in_parse_stack_mode = true;

    for line in lines {
        let line= line.expect("Failed to read line");
        if line == "" {
            in_parse_stack_mode = false;

            continue;
        }

        if in_parse_stack_mode {
            stack_lines.push(line);
        } else {
            let replaced = line.replace("move", "");
            let trimmed = replaced.trim();
            let instructions: Vec<&str> = trimmed.split(" ").collect();

            let count: usize = String::from(instructions[0]).parse().expect("Failed to parse count");
            let from: usize = String::from(instructions[2]).parse::<usize>().expect("Failed to parse from") - 1;
            let to: usize = String::from(instructions[4]).parse::<usize>().expect("Failed to parse to") - 1;

            println!("\tLine: {:?}", line);
            println!("\tInstructions: {} from {} to {}", count, from, to);
            println!("\tFrom stack ({}): {:?}", stacks[from].len(), stacks[from]);

            for i in 0..count {
                let from_index = stacks[from].len() + i - count;
                let copy = stacks[from][from_index].to_owned();
                stacks[to].push(copy);
                stacks[from].remove(from_index);
            }
        }
    }

    let mut i = 1;

    println!("\tStackheads: ");

    print!("\t");

    for stack in stacks {
        print!("{}", match stack.last() {
            Some(c) => {c.to_string()}
            None => "".to_string()
        });

        i += 1;
    }

    print!("\n");
}

// fn parse_stacks(lines: &mut Vec<String>) -> Vec<Vec<char>> {
//     let mut stacks = Vec::<Vec<char>>::new();
//
//     lines.reverse();
//     lines.remove(0);
//
//     for line in lines {
//         let line = line.replace("[", "").replace("]", "")
//
//         let mut i = 0;
//
//         for c in line {
//
//
//             i += 1;
//         }
//     }
//
//     lines.first().expect("Could not get first line")
//
//
// }