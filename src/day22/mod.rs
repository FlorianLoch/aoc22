use std::cmp::max;
use std::fs::File;
use std::io::{BufReader, Lines};
use std::ops::Not;

pub fn solve(lines: &mut Lines<BufReader<File>>, test: bool) {
    let all_lines: Vec<String> = lines.map(|line| line.expect("Failed to read line")).collect();

    let mut map = Map::from_text(&all_lines[0..all_lines.len() - 2], 4);
    let instructions = parse_instructions(all_lines.last().unwrap());

    if !test {
        return;
    }

    println!("{}", map.to_string());

    let code = map.follow_instructions(&instructions, true);

    println!("{}", map.to_string());
    println!("Part 1: code for final position: {}", code);
}

fn parse_instructions(s: &String) -> Vec<Instruction> {
    let mut vec = Vec::<Instruction>::new();

    let mut buf = String::new();

    let handle_steps_token = |buf: &mut String, vec: &mut Vec<Instruction>| {
        let steps = buf.parse().expect("Failed to parse steps");

        vec.push(Instruction::MOVE(steps));

        buf.clear();
    };

    for c in s.chars() {
        match c {
            'L' => {
                handle_steps_token(&mut buf, &mut vec);

                vec.push(Instruction::TURN(TurnDirection::COUNTER_CLOCKWISE))
            }
            'R' => {
                handle_steps_token(&mut buf, &mut vec);

                vec.push(Instruction::TURN(TurnDirection::CLOCKWISE))
            }
            _ => {
                buf.push(c);
            }
        }
    }

    if buf.is_empty().not() {
        handle_steps_token(&mut buf, &mut vec);
    }

    return vec;
}

type Tile = char;

const OUT: Tile = ' ';
const OPEN: Tile = '.';
const WALL: Tile = '#';
const VISITED_DOWN: Tile = 'v';
const VISITED_UP: Tile = '^';
const VISITED_LEFT: Tile = '<';
const VISITED_RIGHT: Tile = '>';

struct Map {
    map: Vec<Tile>,
    width: usize,
    cur_pos: Coord,
    cur_dir: Direction,
    block_size: usize,
}

impl Map {
    fn from_text(lines: &[String], block_size: usize) -> Map {
        // We need the width of the widest line as trailing whitespace is omitted in the input...
        let width: usize = lines.iter().fold(0, |max_val, line| max(max_val, line.len()));

        let mut map = Vec::<Tile>::new();

        if lines.len() == 0 {
            panic!("No lines given!")
        }

        for line in lines {
            for c in line.chars() {
                map.push(c);
            }

            // We need to fill the map row with emptiness
            for _ in 0..width - line.len() {
                map.push(' ');
            }
        }

        // Find the starting position
        let cur_pos = Coord {
            x: lines[0].find(OPEN).expect("Could not find OPEN field in first row") + 1,
            y: 1,
        };

        return Map {
            map,
            width,
            cur_pos,
            cur_dir: Direction::Right,
            block_size,
        };
    }

    fn at(&self, coord: &Coord) -> Tile {
        if coord.x == 0 || coord.y == 0 {
            panic!("Coords are outside of map. Map indices are 1-based.")
        }

        let idx = (coord.y - 1) * self.width + (coord.x - 1);

        if idx >= self.map.len() {
            panic!("Coords are outside of map.")
        }

        return self.map[idx];
    }

    fn set(&mut self, coord: &Coord, tile: Tile) {
        self.map[(coord.y - 1) * self.width + (coord.x - 1)] = tile;
    }

    fn is_off_map(&self, coord: &Coord) -> bool {
        let map_height = self.map.len() / self.width;

        coord.x == 0 || coord.y == 0 || coord.y - 1 >= map_height || coord.x - 1 >= self.width || self.at(coord) == OUT
    }

    fn follow_instructions(&mut self, instructions: &[Instruction], cube_mode: bool) -> usize {
        for instruction in instructions {
            match instruction {
                Instruction::MOVE(steps) => {
                    self.follow_instruction(*steps, cube_mode);
                }
                Instruction::TURN(direction) => {
                    self.cur_dir = self.cur_dir.turn(direction)
                }
            }
        }

        let code = 1000 * self.cur_pos.y + 4 * self.cur_pos.x + self.cur_dir.to_code();

        return code;
    }

    fn follow_instruction(&mut self, steps: usize, cube_mode: bool) {
        for i in 0..steps {
            let mut next = self.cur_pos.move_to(&self.cur_dir);

            // Wrap around in case we would leave the board
            if self.is_off_map(&next) {
                let (new_next, new_direction) = self.wrap_around(cube_mode);

                if self.at(&new_next) == WALL {
                    return;
                }

                next = new_next;

                self.cur_dir = new_direction;
            }

            let next_tile = self.at(&next);

            // Stop in case we hit a wall
            if next_tile == WALL {
                return;
            }

            let cur_dir_tile = self.cur_dir.to_tile();
            let cur_pos = self.cur_pos;

            self.set(&cur_pos, cur_dir_tile);

            self.cur_pos = next;
        }
    }

    fn wrap_around(&self, cube_mode: bool) -> (Coord, Direction) {
        if !cube_mode {
            return (self.find_opposite_edge(), self.cur_dir);
        }

        let cur_side = self.determine_side(&self.cur_pos);

        let x_in_block = (self.cur_pos.x - 1) % self.block_size;
        let y_in_block = (self.cur_pos.y - 1) % self.block_size;

        return match (cur_side, self.cur_dir) {
            (CubeSide::A, Direction::Up) => { (Coord { x: self.block_size - 1 - x_in_block + 1, y: self.block_size + 1 }, Direction::Down) }
            (CubeSide::A, Direction::Left) => { (Coord { x: self.block_size + y_in_block + 1, y: self.block_size + 1 }, Direction::Down) }
            (CubeSide::A, Direction::Right) => { (Coord { x: 4 * self.block_size + 1, y: 3 * self.block_size - 1 - y_in_block + 1 }, Direction::Left) }
            // A -> D is not special
            (CubeSide::E, Direction::Down) => { (Coord { x: self.block_size - 1 - x_in_block + 1, y: 2 * self.block_size + 1 }, Direction::Up) }
            (CubeSide::E, Direction::Left) => { (Coord { x: 2 * self.block_size - 1 - y_in_block + 1, y: 2 * self.block_size + 1 }, Direction::Up) }
            // E -> D is not special
            // E -> F is not special
            (CubeSide::F, Direction::Up) => { (Coord { x: 3 * self.block_size + 1, y: 2 * self.block_size - 1 - x_in_block + 1 }, Direction::Left) }
            (CubeSide::F, Direction::Right) => { (Coord { x: 3 * self.block_size + 1, y: self.block_size - 1 - y_in_block + 1 }, Direction::Left) }
            (CubeSide::F, Direction::Down) => { (Coord { x: 1, y: 2 * self.block_size - 1 - x_in_block + 1 }, Direction::Right) }

            (CubeSide::B, Direction::Up) => { (Coord { x: 3 * self.block_size - 1 - x_in_block + 1, y: 1 }, Direction::Down) }
            (CubeSide::B, Direction::Down) => { (Coord { x: 3 * self.block_size - 1 - x_in_block + 1, y: 3 * self.block_size + 1 }, Direction::Up) }
            (CubeSide::B, Direction::Left) => { (Coord { x: 3 * self.block_size - 1 + y_in_block + 1, y: 3 * self.block_size + 1}, Direction::Up) }

            (CubeSide::C, Direction::Up) => { (Coord { x: 2 * self.block_size + 1, y: x_in_block + 1 }, Direction::Right) }
            (CubeSide::C, Direction::Down) => { (Coord { x: 2 * self.block_size + 1, y: 3 * self.block_size - 1 - x_in_block + 1 }, Direction::Right) }

            (CubeSide::D, Direction::Right) => { (Coord { x: 4 * self.block_size - 1 - y_in_block + 1, y: 2 * self.block_size + 1}, Direction::Down) }

            (a, b) => {panic!("Unexpected Wrap aroung from {:?} to {:?}", a, b)}
        };
    }

    fn determine_side(&self, coord: &Coord) -> CubeSide {
        // --A-
        // BCD-
        // --EF
        let block_x = (coord.x - 1) / self.block_size;
        let block_y = (coord.y - 1) / self.block_size;

        if block_x == 0 && block_y == 0 {
            return CubeSide::A;
        }

        if block_y == 1 {
            if block_x == 0 {
                return CubeSide::B;
            }

            if block_x == 1 {
                return CubeSide::C;
            }

            if block_x == 2 {
                return CubeSide::D;
            }
        }

        if block_y == 2 {
            if block_x == 2 {
                return CubeSide::E;
            }

            if block_x == 3 {
                return CubeSide::F;
            }
        }

        panic!("Could not determine cube side, coords probably invalid")
    }

    fn find_opposite_edge(&self) -> Coord {
        let rev_direction = self.cur_dir.reverse();

        let mut cur_pos = self.cur_pos;

        loop {
            let next = cur_pos.move_to(&rev_direction);

            if self.is_off_map(&next) {
                return cur_pos;
            }

            cur_pos = next;
        }
    }

    fn to_string(&self) -> String {
        let mut buf = String::new();

        for i in 0..self.map.len() {
            buf.push(self.map[i]);

            if (i + 1) % self.width == 0 {
                buf.push('\n');
            }
        }

        return buf;
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Instruction {
    MOVE(usize),
    TURN(TurnDirection),
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum TurnDirection {
    CLOCKWISE,
    COUNTER_CLOCKWISE,
}

#[derive(Eq, PartialEq, Copy, Clone)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn move_right(&self, n: usize) -> Self {
        return Coord {
            x: self.x + n,
            y: self.y,
        };
    }

    fn move_left(&self, n: usize) -> Self {
        return Coord {
            x: self.x - n,
            y: self.y,
        };
    }

    fn move_down(&self, n: usize) -> Self {
        return Coord {
            x: self.x,
            y: self.y + n,
        };
    }

    fn move_up(&self, n: usize) -> Self {
        return Coord {
            x: self.x,
            y: self.y - n,
        };
    }

    fn move_to(&self, dir: &Direction) -> Self {
        match dir {
            Direction::Right => self.move_right(1),
            Direction::Left => self.move_left(1),
            Direction::Up => self.move_up(1),
            Direction::Down => self.move_down(1)
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    fn reverse(&self) -> Direction {
        match self {
            Direction::Right => { Direction::Left }
            Direction::Left => { Direction::Right }
            Direction::Up => { Direction::Down }
            Direction::Down => { Direction::Up }
        }
    }

    fn to_tile(&self) -> Tile {
        match self {
            Direction::Right => { VISITED_RIGHT }
            Direction::Left => { VISITED_LEFT }
            Direction::Up => { VISITED_UP }
            Direction::Down => { VISITED_DOWN }
        }
    }

    fn turn(&self, turn_dir: &TurnDirection) -> Direction {
        let dir = match self {
            Direction::Right => { Direction::Down }
            Direction::Left => { Direction::Up }
            Direction::Up => { Direction::Right }
            Direction::Down => { Direction::Left }
        };

        if turn_dir == &TurnDirection::COUNTER_CLOCKWISE {
            return dir.reverse();
        }

        return dir;
    }

    fn to_code(&self) -> usize {
        match self {
            Direction::Right => { 0 }
            Direction::Left => { 2 }
            Direction::Up => { 3 }
            Direction::Down => { 1 }
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum CubeSide {
    A,
    B,
    C,
    D,
    E,
    F,
}


