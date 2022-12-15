use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufReader, Lines};

use crate::day14::Element::{Air, Rock, Sand};
use crate::puzzle;

#[derive(Eq, PartialEq, Copy, Clone)]
enum Element {
    Rock,
    Sand,
    Air,
}

struct Map {
    map: Vec<Vec<Element>>,
    x_offset: usize,
    width: usize,
    height: usize,
}

impl Map {
    fn new(width: usize, height: usize, x_offset: usize) -> Self {
        let mut map = Vec::<Vec<Element>>::with_capacity(height);

        for _ in 0..height {
            let mut row = Vec::<Element>::with_capacity(width);

            for _ in 0..width {
                row.push(Air);
            }

            map.push(row)
        }

        return Map { map, x_offset, width, height };
    }

    fn draw_formation(&mut self, formation: Vec<Coord>) {
        for i in 0..formation.len() - 1 {
            let cur = &formation[i];
            let next = &formation[i + 1];

            if cur.x != next.x {
                self.draw_horizontal_formation(cur, next);

                continue;
            }

            self.draw_vertical_formation(cur, next);
        }
    }

    fn draw_horizontal_formation(&mut self, from: &Coord, to: &Coord) {
        let width = to.x - from.x;

        if width > 0 {
            for i in 0..=width {
                self.map[from.y as usize][(from.x + i) as usize - self.x_offset] = Rock;
            }

            return;
        }

        for i in 0..=width.abs() {
            self.map[from.y as usize][(from.x - i) as usize - self.x_offset] = Rock;
        }
    }

    fn draw_vertical_formation(&mut self, from: &Coord, to: &Coord) {
        let height = to.y - from.y;

        if height > 0 {
            for i in 0..=height {
                self.map[(from.y + i) as usize][from.x as usize - self.x_offset] = Rock;
            }

            return;
        }

        for i in 0..=height.abs() {
            self.map[(from.y - i) as usize][from.x as usize - self.x_offset] = Rock;
        }
    }

    fn render(&self) {
        for i in 0..self.height {
            let mut row_as_string = String::new();

            for j in 0..self.width {
                row_as_string.push(match self.map[i][j] {
                    Rock => { '#' }
                    Element::Sand => { '0' }
                    Air => { '.' }
                })
            }

            // TODO: print in main function
            println!("{}", row_as_string);
        }
    }

    fn simulate_sand(&mut self, source: Coord) -> bool {
        let mut pos_grain = source;

        'outer: loop {
          // Can the grain move down, diagonally to the left or diagonally to the right?
            let moves_to_check = vec![pos_grain.move_down(), pos_grain.move_diag_left(), pos_grain.move_diag_right()];

            for c in moves_to_check {
                if !self.coord_on_map(c) {
                    return false;
                }

                if self.at(c) == Air {
                    pos_grain = c;

                    continue 'outer;
                }
            }

            // Sand cannot move, it comes to rest
            self.set(pos_grain, Sand);

            return true;
        }
    }

    fn at(&self, c: Coord) -> Element {
        return self.map[c.y as usize][c.x as usize - self.x_offset];
    }

    fn set(&mut self, at: Coord, element: Element) {
        self.map[at.y as usize][at.x as usize - self.x_offset] = element;
    }

    fn coord_on_map(&self, c: Coord) -> bool {
        return (c.x as usize) >= self.x_offset && (c.x as usize) < self.x_offset + self.width && c.y >= 0 && (c.y as usize) < self.height;
    }
}

pub fn solve(lines: &mut Lines<BufReader<File>>, test: bool) {
    let all_lines = puzzle::read_all_lines(lines);

    let (formations, boundaries) = parse_rock_formations(&all_lines);

    // For simplicity we assume that min_y is always 0 (therefore, y_offset is also 0)
    let x_offset = boundaries.min_x as usize;
    let width = (boundaries.max_x - boundaries.min_x + 1) as usize;
    let height = boundaries.max_y as usize + 1;

    let mut map = Map::new(width, height, x_offset);

    for formation in formations {
        map.draw_formation(formation);
    }

    if test {
        map.render();
    }

    let mut grains_resting = 0;

    while map.simulate_sand(Coord{x: 500, y: 0}) {
        grains_resting += 1;
    }

    println!();

    if test {
        map.render();
    }

    println!("Grains resting: {}", grains_resting);

    return;
}

#[derive(Copy, Clone)]
struct Coord {
    // TODO: Replace i32 with usize
    x: i32,
    y: i32,
}

impl Coord {
    pub(crate) fn move_diag_left(&self) -> Self {
        return Coord {
            x: self.x - 1,
            y: self.y + 1
        };
    }
}

impl Coord {
    pub(crate) fn move_diag_right(&self) -> Self {
        return Coord {
            x: self.x + 1,
            y: self.y + 1
        };
    }
}

impl Coord {
    fn move_down(&self) -> Self {
        return Coord {
            x: self.x,
            y: self.y + 1,
        };
    }
}


struct Boundaries {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

fn parse_rock_formations(lines: &Vec<String>) -> (Vec<Vec<Coord>>, Boundaries) {
    let mut formations = Vec::<Vec<Coord>>::new();

    // 500 as initial value because that is the origin of sand and has to be part of the map
    let mut min_x = 500;
    let mut max_x = 500;
    let mut min_y = 0;
    let mut max_y = 0;

    for line in lines {
        let instructions = line.split(" -> ");

        let mut formation = Vec::<Coord>::new();

        for instruction in instructions {
            let mut coord = instruction.split(",");

            let x = coord.next().expect("No x coordinate found").parse().expect("Failed to parse x coordinate");
            let y = coord.next().expect("No y coordinate found").parse().expect("Failed to parse y coordinate");

            min_x = min(min_x, x);
            max_x = max(max_x, x);
            min_y = min(min_y, y);
            max_y = max(max_y, y);

            formation.push(Coord { x, y });
        }

        formations.push(formation);
    }

    return (formations, Boundaries { min_x, max_x, min_y, max_y });
}