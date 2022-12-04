mod puzzle;
mod day01;
mod day02;
mod day03;
mod day04;

fn main() {
    let mut puzzler = puzzle::Puzzler::new();

    puzzler.add_puzzle(Box::new(day01::Day01{}));
    puzzler.add_puzzle(Box::new(day02::Day02{}));
    puzzler.add_puzzle(Box::new(day03::Day03{}));
    puzzler.add_puzzle(Box::new(day04::Day04{}));

    // puzzler.run_puzzle_for_day(1);
    puzzler.run_puzzle_for_day(2);
    puzzler.run_puzzle_for_day(3);
    puzzler.run_puzzle_for_day(4);
}