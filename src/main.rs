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
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

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
    puzzler.add_puzzle(day12::solve);
    puzzler.add_puzzle(day13::solve);
    puzzler.add_puzzle(day14::solve);
    puzzler.add_puzzle(day15::solve);
    puzzler.add_puzzle(day16::solve);
    puzzler.add_puzzle(day17::solve);
    puzzler.add_puzzle(day18::solve);
    puzzler.add_puzzle(day19::solve);
    puzzler.add_puzzle(day20::solve);
    puzzler.add_puzzle(day21::solve);
    puzzler.add_puzzle(day22::solve);
    puzzler.add_puzzle(day23::solve);
    puzzler.add_puzzle(day24::solve);
    puzzler.add_puzzle(day25::solve);

    // puzzler.run_latest_puzzle();
    puzzler.run_puzzle_for_day(24);
}