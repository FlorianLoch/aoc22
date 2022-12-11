extern crate core;

mod puzzle;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;

fn main() {
    let mut puzzler = puzzle::Puzzler::new();

    puzzler.add_puzzle(day01::solve);
    puzzler.add_puzzle(day02::solve);
    puzzler.add_puzzle(day03::solve);
    puzzler.add_puzzle(day04::solve);
    puzzler.add_puzzle(day05::solve);
    puzzler.add_puzzle(day06::solve);
    puzzler.add_puzzle(day07::solve);
    puzzler.add_puzzle(day08::solve);
    puzzler.add_puzzle(day09::solve);
    puzzler.add_puzzle(day10::solve);
    puzzler.add_puzzle(day11::solve);

    puzzler.run_latest_puzzle()
}