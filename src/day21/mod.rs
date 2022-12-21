use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Lines};

use regex::Regex;

pub fn solve(lines: &mut Lines<BufReader<File>>, test: bool) {
    let mut monkeys_list = Vec::<String>::new();
    let mut monkeys_map = HashMap::<String, Monkey>::new();

    for line in lines {
        let (name, monkey) = Monkey::from_text(&line.expect("Failed to read line"));

        monkeys_list.push(name.clone());
        monkeys_map.insert(name, monkey);
    }

    loop {
        for i in 0..monkeys_list.len() {
            let name = &monkeys_list[i];

            let monkey = monkeys_map.get(name).expect("Monkey not in map!?!");

            match monkey {
                Monkey::Calculator { 0: in_a, 1: in_b, 2: op } => {
                    let in_a_monkey = monkeys_map.get(in_a).expect("Monkey not in map!?!");
                    let in_b_monkey = monkeys_map.get(in_b).expect("Monkey not in map!?!");

                    match (in_a_monkey, in_b_monkey) {
                        (Monkey::Yeller { 0: in_a_num }, Monkey::Yeller { 0: in_b_num }) => {
                            let result = match op {
                                Operation::Add => { in_a_num + in_b_num }
                                Operation::Sub => { in_a_num - in_b_num }
                                Operation::Multiply => { in_a_num * in_b_num }
                                Operation::Divide => { in_a_num / in_b_num }
                            };

                            monkeys_map.insert(name.clone(), Monkey::Yeller(result));
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        // Check whether we are done
        match monkeys_map.get("root").expect("Monkey 'root' not in map!?!") {
            Monkey::Yeller(number) => {
                println!("Part 1: Root monkey will yell: {}", number);

                break;
            }
            _ => {}
        }
    }

    if !test {
        return;
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum Operation {
    Add,
    Sub,
    Multiply,
    Divide,
}

impl Operation {
    fn from_text(s: &str) -> Operation {
        return match s {
            "+" => { Operation::Add }
            "-" => { Operation::Sub }
            "*" => { Operation::Multiply }
            "/" => { Operation::Divide }
            _ => {
                panic!("Invalid operation")
            }
        };
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum Monkey {
    Calculator(String, String, Operation),
    Yeller(i128),
}

impl Monkey {
    fn from_text(line: &String) -> (String, Monkey) {
        let re = Regex::new(r"^(\w{4}): ((\w{4}) (.) (\w{4})|(\d*))$").expect("Failed to create RegEx");

        let captures = re.captures(line).unwrap();

        let name = captures.get(1).expect("Failed to match name").as_str().to_string();

        // If there is an operator it must be a Calculator monkey
        if captures.get(4).is_some() {
            let in_a = captures.get(3).expect("Failed to match input A");
            let op = captures.get(4).expect("Failed to match operator");
            let in_b = captures.get(5).expect("Failed to match input B");

            return (name, Monkey::Calculator(in_a.as_str().to_string(), in_b.as_str().to_string(), Operation::from_text(op.as_str())));
        }

        let num = captures.get(2).expect("Failed to match number");

        return (name, Monkey::Yeller(num.as_str().parse().expect("Failed to parse number")));
    }
}