use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, Lines};

pub fn solve(lines: &mut Lines<BufReader<File>>, test: bool) {
    let mut elves = HashSet::<Coord>::from_iter(parse_field(lines));

    let mut part1 = elves.clone();

    for i in 0..10 {
        simulate_round(&mut part1, i);
    }


    let area = compute_bounding_box_area(&elves);

    println!("Part 1: empty ground tiles: {}", area - elves.len() as i32);

    // Part 2

    let mut round = 0;

    loop {
        if !simulate_round(&mut elves, round) {
            break;
        }

        round += 1;
    }

    println!("Part 2: elves stop moving after {} rounds", round + 1);
}

fn render(coords_set: &HashSet<Coord>) {
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;

    for coord in coords_set {
        min_x = min(min_x, coord.x);
        max_x = max(max_x, coord.x);
        min_y = min(min_y, coord.y);
        max_y = max(max_y, coord.y);
    }

    let width = (max_x - min_x) as usize + 1;
    let height = (max_y - min_y) as usize + 1;

    for y in 0..height {
        for x in 0..width {
            if coords_set.contains(&Coord{x: x as i32 + min_x, y: y as i32 + min_y}) {
                print!("#");

                continue;
            }

            print!(".")
        }

        println!();
    }
}


fn compute_bounding_box_area(coords_set: &HashSet<Coord>) -> i32 {
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;

    for coord in coords_set {
        min_x = min(min_x, coord.x);
        max_x = max(max_x, coord.x);
        min_y = min(min_y, coord.y);
        max_y = max(max_y, coord.y);
    }

    (max_x - min_x + 1) * (max_y - min_y + 1)
}

fn simulate_round(coords_set: &mut HashSet<Coord>, round: usize) -> bool {
    let elves_that_need_to_move = coords_set.iter().filter(|elf| {
        elf.adjacent_coords().iter().any(|adjacent_coord| coords_set.contains(adjacent_coord))
    });

    let mut propositions = HashMap::<Coord, (Coord, bool)>::with_capacity(coords_set.len());

    elves_that_need_to_move.for_each(|elf| {
        match find_valid_move(elf, &coords_set, round) {
            None => {}
            Some(new_coord) => {
                if propositions.contains_key(&new_coord) {
                    propositions.remove(&new_coord);

                    propositions.insert(new_coord, (*elf, false));
                } else {
                    propositions.insert(new_coord, (*elf, true));
                }
            }
        }
    });

    let mut changes = 0;

    for (proposed_position, (current_position, valid_proposition)) in propositions {
        if !valid_proposition {
            continue;
        }

        changes += 1;

        coords_set.remove(&current_position);

        coords_set.insert(proposed_position);
    }

    changes > 0
}

fn find_valid_move(elf_pos: &Coord, coords_set: &HashSet<Coord>, round: usize) -> Option<Coord> {
    let is_free = |x_offset: i32, y_offset: i32| {
        !coords_set.contains(&elf_pos.move_rel(x_offset, y_offset))
    };

    for i in 0..4 {
        let a = ((round % 4) + i) % 4;

        match a {
            0 => {
                // println!("--> 0");
                if is_free(0, -1) && is_free(-1, -1) && is_free(1, -1) {
                    // println!("0");
                    return Some(elf_pos.move_up(1));
                }
            }
            1 => {
                // println!("--> 1");
                if is_free(0, 1) && is_free(-1, 1) && is_free(1, 1) {
                    // println!("1");
                    return Some(elf_pos.move_down(1));
                }
            }
            2 => {
                // println!("--> 3");
                if is_free(-1, -1) && is_free(-1, 0) && is_free(-1, 1) {
                    // println!("3");
                    return Some(elf_pos.move_left(1));
                }
            }
            3 => {
                // println!("--> 2");
                if is_free(1, -1) && is_free(1, 0) && is_free(1, 1) {
                    // println!("2");
                    return Some(elf_pos.move_right(1));
                }
            }
            _ => { panic!("This should never happen") }
        }
    }

    return None;
}

fn parse_field(lines: &mut Lines<BufReader<File>>) -> Vec<Coord> {
    let mut vec = Vec::<Coord>::new();

    for (y, line) in lines.enumerate() {
        for (x, c) in line.expect("Failed to read line").chars().enumerate() {
            if c == '#' {
                vec.push( Coord {x: x as i32, y: y as i32 });
            }
        }
    }

    return vec;
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn move_right(&self, n: i32) -> Self {
        return Coord {
            x: self.x + n,
            y: self.y,
        };
    }

    fn move_left(&self, n: i32) -> Self {
        return Coord {
            x: self.x - n,
            y: self.y,
        };
    }

    fn move_down(&self, n: i32) -> Self {
        return Coord {
            x: self.x,
            y: self.y + n,
        };
    }

    fn move_up(&self, n: i32) -> Self {
        return Coord {
            x: self.x,
            y: self.y - n,
        };
    }

    fn move_rel(&self, x: i32, y: i32) -> Self {
        Coord { x: self.x + x, y: self.y + y }
    }

    fn adjacent_coords(&self) -> Vec<Self> {
        vec![
            self.move_rel(-1, -1),
            self.move_rel(0, -1),
            self.move_rel(1, -1),
            self.move_rel(-1, 0),
            self.move_rel(1, 0),
            self.move_rel(-1, 1),
            self.move_rel(0, 1),
            self.move_rel(1, 1),
        ]
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}
