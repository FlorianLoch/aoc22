use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Lines};

use crate::day17::RockType::{HBar, L, Plus, Square, VBar};
use crate::puzzle;

pub fn solve(lines: &mut Lines<BufReader<File>>, test: bool) {
    let all_lines = puzzle::read_all_lines(lines);
    let jet_pattern = all_lines.first().expect("Failed to get jet pattern");

    let mut map = Map::new(7);

    let mut jet = Jet::new(jet_pattern.clone());

    simulate_n_rocks(0, 2022, &mut jet, &mut map);

    println!("Part 1: Height of tower after 2022 rocks: {}", map.height());

    const TOTAL_ROCKS: usize = 1_000_000_000_000;

    let mut map = Map::new(7);

    let mut jet = Jet::new(jet_pattern.clone());

    let k_1 = simulate_rocks_until_repetition(0, &mut jet, &mut map);
    let h_1 = map.height();

    println!("Part 2: k_1={}, h_1={}", k_1, h_1);

    let k_2 = simulate_rocks_until_repetition(k_1, &mut jet, &mut map) - k_1;
    let h_2 = map.height() - h_1;

    println!("Part 2: k_2={}, h_2={}", k_2, h_2);

    let repetitions = (TOTAL_ROCKS - k_1) / k_2;
    let h_2_n = repetitions * h_2;

    let k_n = (TOTAL_ROCKS - k_1) % k_2; // remaining rocks

    println!("Part 2: k_n={}", k_n);

    assert_eq!(k_1 + k_2 * repetitions + k_n, TOTAL_ROCKS);

    simulate_n_rocks(k_1 + k_2 * repetitions, k_n, &mut jet, &mut map);
    let h_n = map.height() - h_2 - h_1;

    let h_total = h_1 + h_2_n + h_n;

    println!("Part 2: Height of tower after 1_000_000_000_000 iterations: {}", h_total);

    return;
}

fn simulate_rocks_until_repetition(offset: usize, jet: &mut Jet, map: &mut Map) -> usize {
    let mut i: usize = offset;

    let mut jet_idx_mapping = Vec::<(bool, usize, usize)>::new();

    for _ in 0..jet.pattern.len() {
        jet_idx_mapping.push((false, 0, 0))
    }

    loop {
        if jet_idx_mapping[jet.idx].0 && jet_idx_mapping[jet.idx].1 == i % 5 {
            return i;
        }

        jet_idx_mapping[jet.idx] = (true, i % 5, i);

        simulate_rock(i, jet, map);

        i += 1;
    }
}

fn simulate_n_rocks(offset: usize, n: usize, jet: &mut Jet, map: &mut Map) {
    for i in 0..n {
        simulate_rock(i + offset, jet, map);
    }
}

fn simulate_rock(i: usize, jet: &mut Jet, map: &mut Map) {
    let rock_type: RockType = match i % 5 {
        0 => { HBar }
        1 => { Plus }
        2 => { L }
        3 => { VBar }
        4 => { Square }
        _ => { panic!("This should never happen") }
    };

    let mut rock = Rock::new(Coord::new(2, 0), rock_type);

    map.add_new_rows(3 + rock.height());

    loop {
        let direction = jet.next().unwrap();

        let moved = rock.coord.move_by_jet(direction, rock.width(), 7);
        if map.fits_in(&moved, &rock) {
            rock.move_to(moved);
        }

        let fallen = rock.coord.fall();
        if map.fits_in(&fallen, &rock) {
            rock.move_to(fallen);
        } else {
            map.add_rock(&rock.coord, &rock);

            break;
        }
    }

    map.cleanup();
}

struct Jet {
    pattern: String,
    idx: usize,
}

impl Jet {
    fn new(pattern: String) -> Jet {
        return Jet {
            pattern,
            idx: 0,
        };
    }
}

impl Iterator for Jet {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.pattern.chars().nth(self.idx);

        self.idx += 1;

        if self.idx == self.pattern.len() {
            self.idx = 0;
        }

        return c;
    }
}

#[derive(Copy, Clone)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn new(x: usize, y: usize) -> Self {
        return Coord { x, y };
    }

    fn move_relative(&self, other: &Self) -> Self {
        return Coord {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }

    fn move_by_jet(&self, direction: char, rock_width: usize, max_width: usize) -> Self {
        let x_offset = if direction == '<' {
            if self.x > 0 { -1 } else { 0 }
        } else {
            if self.x + rock_width < max_width { 1 } else { 0 }
        };

        return Coord {
            x: ((self.x as i32) + x_offset) as usize,
            y: self.y,
        };
    }

    fn fall(&self) -> Self {
        return Coord {
            x: self.x,
            y: self.y + 1,
        };
    }
}

struct Rock {
    coord: Coord,
    kind: RockType,
}

impl Rock {
    fn new(coord: Coord, kind: RockType) -> Self {
        return Rock { coord, kind };
    }

    fn move_to(&mut self, to: Coord) {
        self.coord = to;
    }

    fn occupied_rel_coords(&self) -> Vec<Coord> {
        match self.kind {
            RockType::HBar => {
                vec![Coord::new(0, 0), Coord::new(1, 0), Coord::new(2, 0), Coord::new(3, 0)]
            }
            RockType::Plus => {
                vec![Coord::new(1, 0), Coord::new(0, 1), Coord::new(1, 1), Coord::new(2, 1), Coord::new(1, 2)]
            }
            RockType::L => {
                vec![Coord::new(2, 0), Coord::new(2, 1), Coord::new(0, 2), Coord::new(1, 2), Coord::new(2, 2)]
            }
            RockType::VBar => {
                vec![Coord::new(0, 0), Coord::new(0, 1), Coord::new(0, 2), Coord::new(0, 3)]
            }
            RockType::Square => {
                vec![Coord::new(0, 0), Coord::new(1, 0), Coord::new(0, 1), Coord::new(1, 1)]
            }
        }
    }

    fn height(&self) -> usize {
        match self.kind {
            HBar => { 1 }
            Plus => { 3 }
            L => { 3 }
            VBar => { 4 }
            Square => { 2 }
        }
    }

    fn width(&self) -> usize {
        match self.kind {
            HBar => { 4 }
            Plus => { 3 }
            L => { 3 }
            VBar => { 1 }
            Square => { 2 }
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum RockType {
    HBar,
    Plus,
    L,
    VBar,
    Square,
}

struct Map {
    map: Vec<bool>,
    width: usize,
}

impl Map {
    fn new(width: usize) -> Self {
        return Map { map: Vec::<bool>::new(), width };
    }

    fn add_new_rows(&mut self, n: usize) {
        for _ in 0..n * self.width {
            self.map.insert(0, false)
        }
    }

    fn add_rock(&mut self, at: &Coord, rock: &Rock) {
        for coord in rock.occupied_rel_coords() {
            self.set(coord.move_relative(&at));
        }
    }

    fn fits_in(&self, at: &Coord, rock: &Rock) -> bool {
        for coord in rock.occupied_rel_coords() {
            let c = coord.move_relative(&at);

            if c.x >= self.width {
                return false;
            }

            if c.y >= self.height() {
                return false;
            }

            if self.at(c) {
                return false;
            }
        }

        return true;
    }

    fn height(&self) -> usize {
        return self.map.len() / self.width;
    }

    fn to_string(&self) -> String {
        let mut str_buf = String::new();

        for i in 0..self.map.len() {
            if i % self.width == 0 && i != 0 {
                str_buf.push('\n');
            }

            if self.map[i] {
                str_buf.push('@')
            } else {
                str_buf.push('.')
            }
        }

        return str_buf;
    }

    fn at(&self, c: Coord) -> bool {
        return self.map[c.y * self.width + c.x];
    }

    fn set(&mut self, c: Coord) {
        self.map[c.y * self.width + c.x] = true;
    }

    fn cleanup(&mut self) {
        while self.height() > 0 {
            for x in 0..self.width {
                if self.map[x] {
                    return;
                }
            }

            for x in 0..self.width {
                self.map.remove(0);
            }
        }
    }
}
