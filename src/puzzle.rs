use std::borrow::Borrow;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub trait Puzzle {
    fn group_n_lines(&self) -> usize {
        return 1
    }
    fn solve(&self, lines: &Vec<String>) -> i32;
}

pub struct Puzzler {
    puzzles: Vec<Box<dyn Puzzle>>
}

impl Puzzler {
    pub fn new() -> Puzzler {
        Puzzler{ puzzles: vec![] }
    }

    pub fn add_puzzle(&mut self, boxed_puzzle: Box<dyn Puzzle>) {
        self.puzzles.push(boxed_puzzle)
    }

    pub fn run_puzzle_for_day(&self, day: usize) {
        // let day: usize = (day - 1).try_into().unwrap();

        if day - 1 >= self.puzzles.len() {
            panic!("Invalid day ('{}')!", day);
        }

        run_puzzle(self.puzzles[day - 1].borrow(), day);
    }

    pub fn run_latest_puzzle(&self) {
        match self.puzzles.last() {
            Some(p)=> {
                let day = self.puzzles.len();

                run_puzzle(p.borrow(), day);
            }
            None => {println!("No puzzle has been added yet")}
        }
    }
}

fn run_puzzle(puzzle: &dyn Puzzle, day: usize) {
    println!("=> Running puzzle for day {}", day);

    let path = format!("./src/day{:0>2}/", day);

    let file = path.to_owned() +  "input_test";
    let test_result = _run_puzzle(puzzle, file.as_str());

    println!("Test result: {}", test_result);

    let file = path.to_owned() +  "input_full";
    let actual_result = _run_puzzle(puzzle, file.as_str());

    println!("Actual result: {}", actual_result);
}

fn _run_puzzle(puzzle: &dyn Puzzle, input_file: &str) -> i32 {
    let mut result = 0;
    let mut i: usize = 1;

    let lines = read_lines(input_file).expect("Failed to read lines from file");

    let mut list = Vec::<String>::new();

    for line in lines {
        let l = line.expect("Failed to read line");

        list.push(l);

        if i % puzzle.group_n_lines() == 0 {
            result += puzzle.solve(&list);

            list.truncate(0);
        }

        i += 1;
    }

    return result;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}