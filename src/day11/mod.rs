use std::fs::File;
use std::io::{BufReader, Lines};

#[derive(Debug)]
struct Monkey {
    items_stack: Vec<i64>,
    inspections_performed: i64,
    divisible_by: i64,
    throw_targets: [usize; 2],

    // returns the new worry level
    inspect_fn: fn(worry_level: i64) -> i64,
}

impl Monkey {
    fn add_item(&mut self, worry_value: i64) {
        self.items_stack.push(worry_value);
    }

    // returns the index/name of the next monkey
    fn throw_to(&self, worry_level: i64) -> usize {
        if worry_level % self.divisible_by == 0 {
            return self.throw_targets[0];
        }

        return self.throw_targets[1];
    }

    fn inspect(&self, mut worry_value: i64, use_modulo: bool, lcd: i64) -> i64 {
        worry_value = (self.inspect_fn)(worry_value);

        if use_modulo {
            worry_value %= lcd;
        }

        return worry_value;
    }
}

pub fn solve(_: &mut Lines<BufReader<File>>, test_run: bool) {
    // We do not need the input today

    let part_1 = _solve(&mut if test_run { get_test_monkeys() } else { get_serious_monkeys() }, 20, false);
    println!("Part 1: monkey business: {}", part_1);

    let part_2 = _solve(&mut if test_run { get_test_monkeys() } else { get_serious_monkeys() }, 10_000, true);
    println!("Part 2: monkey business: {}", part_2);
}

// Either relax worries using division (part 1) or using modulo (part 2)
fn _solve(monkeys: &mut Vec<Monkey>, rounds: i32, use_modulo: bool) -> i64 {
    let lcd = lcd(&monkeys);

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            for _ in 0..monkeys[i].items_stack.len() {
                let item = monkeys[i].items_stack[0];
                
                monkeys[i].items_stack.remove(0);

                let mut worry_level = monkeys[i].inspect(item, use_modulo, lcd);

                // Only in part 1
                if !use_modulo {
                    worry_level /= 3;
                }

                let to = monkeys[i].throw_to(worry_level);

                monkeys[to].add_item(worry_level);

                monkeys[i].inspections_performed += 1;
            }
        }
    }

    monkeys.sort_by_key(|x| { x.inspections_performed });
    monkeys.reverse();

    return monkeys[0].inspections_performed * monkeys[1].inspections_performed;
}

fn lcd(monkeys: &Vec<Monkey>) -> i64 {
    let mut lcd = 1;

    for monkey in monkeys {
        lcd *= monkey.divisible_by;
    }

    return lcd;
}

fn get_serious_monkeys() -> Vec<Monkey> {
    let mut monkeys = Vec::<Monkey>::new();

    monkeys.push(Monkey {
        items_stack: vec![78, 53, 89, 51, 52, 59, 58, 85],
        inspect_fn: |worry_value| { worry_value * 3 },
        divisible_by: 5,
        throw_targets: [2, 7],
        inspections_performed: 0,
    });

    monkeys.push(Monkey {
        items_stack: vec![64],
        inspect_fn: |worry_value| { worry_value + 7 },
        divisible_by: 2,
        throw_targets: [3, 6],
        inspections_performed: 0,
    });

    monkeys.push(Monkey {
        items_stack: vec![71, 93, 65, 82],
        inspect_fn: |worry_value| { worry_value + 5 },
        divisible_by: 13,
        throw_targets: [5, 4],
        inspections_performed: 0,
    });

    monkeys.push(Monkey {
        items_stack: vec![67, 73, 95, 75, 56, 74],
        inspect_fn: |worry_value| { worry_value + 8 },
        divisible_by: 19,
        throw_targets: [6, 0],
        inspections_performed: 0,
    });

    monkeys.push(Monkey {
        items_stack: vec![85, 91, 90],
        inspect_fn: |worry_value| { worry_value + 4 },
        divisible_by: 11,
        throw_targets: [3, 1],
        inspections_performed: 0,
    });

    monkeys.push(Monkey {
        items_stack: vec![67, 96, 69, 55, 70, 83, 62],
        inspect_fn: |worry_value| { worry_value * 2 },
        divisible_by: 3,
        throw_targets: [4, 1],
        inspections_performed: 0,
    });

    monkeys.push(Monkey {
        items_stack: vec![53, 86, 98, 70, 64],
        inspect_fn: |worry_value| { worry_value + 6 },
        divisible_by: 7,
        throw_targets: [7, 0],
        inspections_performed: 0,
    });

    monkeys.push(Monkey {
        items_stack: vec![88, 64],
        inspect_fn: |worry_value| { worry_value * worry_value },
        divisible_by: 17,
        throw_targets: [2, 5],
        inspections_performed: 0,
    });

    return monkeys;
}

fn get_test_monkeys() -> Vec<Monkey> {
    let mut monkeys = Vec::<Monkey>::new();

    monkeys.push(Monkey {
        inspect_fn: |worry_value| { worry_value * 19 },
        items_stack: vec![79, 98],
        divisible_by: 23,
        throw_targets: [2, 3],
        inspections_performed: 0,
    });

    monkeys.push(Monkey {
        inspect_fn: |worry_value| { worry_value + 6 },
        items_stack: vec![54, 65, 75, 74],
        divisible_by: 19,
        throw_targets: [2, 0],
        inspections_performed: 0,
    });

    monkeys.push(Monkey {
        inspect_fn: |worry_value| { worry_value * worry_value },
        items_stack: vec![79, 60, 97],
        divisible_by: 13,
        throw_targets: [1, 3],
        inspections_performed: 0,
    });

    monkeys.push(Monkey {
        inspect_fn: |worry_value| { worry_value + 3 },
        items_stack: vec![74],
        divisible_by: 17,
        throw_targets: [0, 1],
        inspections_performed: 0,
    });

    return monkeys;
}