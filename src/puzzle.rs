use io::BufReader;
use std::fs::File;
use std::io::{self, BufRead, Lines};
use std::path::Path;

type PuzzleFn = fn (lines: &mut Lines<BufReader<File>>);

pub struct Puzzler {
    puzzles: Vec<PuzzleFn>
}

#[allow(dead_code)]
impl Puzzler {
    pub fn new() -> Puzzler {
        Puzzler { puzzles: vec![] }
    }

    pub fn add_puzzle(&mut self, puzzle: PuzzleFn) {
        self.puzzles.push(puzzle)
    }

    pub fn run_puzzle_for_day(&self, day: usize) {
        if day - 1 >= self.puzzles.len() {
            panic!("Invalid day ('{}')!", day);
        }

        run_puzzle(self.puzzles[day - 1], day);
    }

    pub fn run_latest_puzzle(&self) {
        match self.puzzles.last() {
            Some(p) => {
                let day = self.puzzles.len();

                println!("=> Running puzzle for day {}", day);

                run_puzzle(*p, day);
            }
            None => { println!("No puzzle has been added yet") }
        }
    }

    pub fn run_all_puzzles(&self) {
        if self.puzzles.len() == 0 {
            println!("No puzzle has been added yet");

            return;
        }

        for i in 0..self.puzzles.len() {
            self.run_puzzle_for_day(i+1)
        }
    }
}

fn run_puzzle(puzzle: PuzzleFn, day: usize) {
    let path = format!("./src/day{:0>2}/", day);

    println!("==> Day {}", day);
    println!("--> With test input");

    _run_puzzle(puzzle, &path, "input_test");

    println!("--> With actual input");

    _run_puzzle(puzzle, &path, "input_full");
}

fn _run_puzzle(puzzle: PuzzleFn, input_path: &str, input_file: &str) {
    puzzle(&mut read_lines(input_path.to_owned() + input_file).expect("Failed to read file"));
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

pub fn read_n_lines(lines: &mut Lines<BufReader<File>>, n: usize) -> Vec<String> {
    let mut vec = Vec::<String>::new();

    for _ in 0..n {
        match lines.next() {
            None => return vec,
            Some(line) => vec.push(line.expect("Failed to read line"))
        }
    }

    return vec;
}