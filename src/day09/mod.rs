use std::collections::HashSet;
use std::fs::File;
use std::hash::{Hash};
use std::io::{BufReader, Lines};

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn move_right(&mut self, n: i32) {
        self.x += n;
    }

    fn move_left(&mut self, n: i32) {
        self.x -= n;
    }

    fn move_up(&mut self, n: i32) {
        self.y += n;
    }

    fn move_down(&mut self, n: i32) {
        self.y -= n;
    }

    fn distance(&self, other: Coord) -> (i32, i32) {
        return (other.x - self.x, other.y - self.y);
    }

    fn move_coord(&mut self, m: &Move) {
        match m {
            Move::Right => self.move_right(1),
            Move::Left => self.move_left(1),
            Move::Up => self.move_up(1),
            Move::Down => self.move_down(1)
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Move {
    Right,
    Left,
    Up,
    Down,
}

fn parse_move(line: String) -> Vec<Move> {
    let splits = line.split_whitespace().collect::<Vec<&str>>();

    let direction = match *splits.get(0).expect("No direction given in line") {
        "R" => Move::Right,
        "L" => Move::Left,
        "U" => Move::Up,
        "D" => Move::Down,
        &_ => { panic!("Invalid direction given.") }
    };

    let count: i32 = splits.get(1).expect("No count given in line").parse().expect("Failed to parse count");

    let mut moves = Vec::<Move>::new();

    for _ in 0..count {
        moves.push(direction);
    }

    return moves;
}

pub fn solve(lines: &mut Lines<BufReader<File>>) {
    let mut moves = Vec::<Move>::new();

    for line in lines {
        moves.append(&mut parse_move(line.expect("Failed to read line")));
    }

    solve1(&moves);
    solve2(&moves);
}

fn solve1(moves: &Vec<Move>) {
    let mut visited_coords = HashSet::<Coord>::new();

    // We assume starting at Coord(0, 0).
    // Doesn't really matter for the puzzle as we are just interesting in the sum of visited coords,
    // not in their actual position.
    // Therefore, we also do not need an actual map of the area.

    let mut cur_head_coord = Coord { x: 0, y: 0 };
    let mut cur_tail_coord = Coord { x: 0, y: 0 };

    visited_coords.insert(cur_tail_coord);

    for m in moves {
        cur_head_coord.move_coord(m);

        move_tail_according_to_head(&mut cur_tail_coord, cur_head_coord);

        visited_coords.insert(cur_tail_coord);
    }

    println!("\tPart 1: Coords visited at least once: {}", visited_coords.len());
}

fn solve2(moves: &Vec<Move>) {
    let mut visited_coords = HashSet::<Coord>::new();

    let mut knots = Vec::<Coord>::new();

    const TAIL: usize = 0;
    const HEAD: usize = 9;

    for _ in 0..HEAD+1 {
        knots.push(Coord{x: 0, y: 0})
    }

    visited_coords.insert(knots[TAIL]);

    for m in moves {
        let head = knots.get_mut(HEAD).unwrap();
        head.move_coord(m);

        for i in 0..HEAD {
            let head = *knots.get(HEAD-i).unwrap();
            let knot = knots.get_mut(HEAD-i-1).unwrap();

            move_tail_according_to_head(knot, head);
        }

        visited_coords.insert(knots[TAIL]);
    }

    println!("\tPart 2: Coords visited at least once: {}", visited_coords.len());
}

fn move_tail_according_to_head(tail: &mut Coord, head: Coord) {
    let (d_x, d_y) = tail.distance(head);

    if d_x.abs() <= 1 && d_y.abs() <= 1 {
        return;
    }

    if d_x > 0 {
        tail.move_right(1);
    }

    if d_x < 0 {
        tail.move_left(1);
    }

    if d_y > 0 {
        tail.move_up(1);
    }

    if d_y < 0 {
        tail.move_down(1);
    }
}