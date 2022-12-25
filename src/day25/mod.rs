use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufReader, Lines};

pub fn solve(lines: &mut Lines<BufReader<File>>, test: bool) {
    let all_lines: Vec<String> = lines.map(|line| line.expect("Failed to read line")).collect();
    let mut sum = 0;

    for line in all_lines {
        sum += from_snafu(&line);
        // println!("{} ==> {} ==> {}", line, from_snafu(&line), to_snafu(from_snafu(&line)));
    }

    println!("Part 1: Sum {}, SNAFU: {}", sum, to_snafu(sum));
}

fn from_snafu(snafu: &String) -> i64 {
    let mut sum: i64 = 0;

    for (i, c) in snafu.chars().rev().enumerate() {
        sum += 5_i64.pow(i as u32) * match c {
            '0' => { 0 }
            '1' => { 1 }
            '2' => { 2 }
            '-' => { -1 }
            '=' => { -2 }
            _ => { panic!("Invalid character in SNAFU") }
        };
    }

    return sum;
}

fn to_snafu(mut num: i64) -> String {
    let mut digits = Vec::<i64>::new();

    let mut pow: i32 = 0;

    while (5 as i64).pow(pow as u32) < num {
        pow += 1;
    }

    let mut digits_inserted = 0;

    while num > 0 {
        let a = (5 as i64).pow(pow as u32);
        let f = num / a;

        num %= a;

        if f > 2 {
            for i in 0..=digits_inserted {
                if i == digits_inserted {
                    digits.insert(0, 1);

                    digits_inserted += 1;

                    break;
                } else {
                    if digits[digits_inserted - i - 1] < 2 {
                        digits[digits_inserted - i - 1] += 1;

                        break;
                    }

                    digits[digits_inserted - i - 1] = -2;
                }

            }

            digits.push(f-5);

            digits_inserted += 1;

            pow -= 1;

            continue;
        }

        if f > 0 || digits.len() > 0 {
            digits.push(f);

            digits_inserted += 1;
        }

        pow -= 1;
    }

    let mut s = String::new();

    for d in digits {
        s.push(match d {
            -2 => {'='}
            -1 => {'-'}
            0 => {'0'}
            1 => {'1'}
            2 => {'2'}
            _ => {panic!("Failed to convert to SNAFU")}
        });
    }

    return s;
}