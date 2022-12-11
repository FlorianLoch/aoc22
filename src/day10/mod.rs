use std::fs::File;
use std::io::{BufReader, Lines};

pub fn solve(lines: &mut Lines<BufReader<File>>, _: bool) {
    let mut signal_strengths = Vec::<i32>::new();
    let mut sum_signal_strengths = 0;

    let mut screen = String::from("");

    let mut cycle = 1;
    let mut reg_x = 1;

    let mut check = |cycle: i32, reg_x: i32| {
        if cycle % 40 == 20 {
            signal_strengths.push(reg_x * cycle);

            sum_signal_strengths += reg_x * cycle;
        }

        let cur_pixel = (cycle - 1) % 40;

        if reg_x + 1 == cur_pixel || reg_x == cur_pixel || reg_x - 1 == cur_pixel {
            screen.push('#');
        } else {
            screen.push('.');
        }

        if cycle % 40 == 0 {
            screen.push('\n');
        }
    };

    for line in lines {
        let s = line.expect("Failed to read line");
        let splits: Vec<&str> = s.split_whitespace().collect();

        check(cycle, reg_x);

        if splits[0] != "noop" {
            let inc: i32 = splits[1].parse().expect("Failed to parse increment");

            cycle += 1;

            check(cycle, reg_x);

            reg_x += inc;
        }

        cycle += 1;
    }

    println!("Signal strengths: {:?}", signal_strengths);
    println!("Cycles done: {}", cycle);
    println!("Summed up signal strengths: {}", sum_signal_strengths);

    println!("{}", screen);
}