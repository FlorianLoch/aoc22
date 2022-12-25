use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, Lines};

pub fn solve(lines: &mut Lines<BufReader<File>>, test: bool) {
    let all_lines: Vec<String> = lines.map(|line| line.expect("Failed to read line")).collect();

    let (map, blizzards) = parse_map(&all_lines);

    let weatherman = BlizzardPrediction::new(&blizzards, map.width, map.height);

    // for i in 0..20 {
    //     render(&map, weatherman.predict_for_iteration(i));
    //     // println!();
    // }


    let min_iterations_to = find_path(map.start, 0, 0, 1000, &map, &weatherman, &mut HashMap::<(usize, Coord), usize>::new(), false);

    println!("Part 1: {}", min_iterations_to);

    let min_iterations_back = find_path(map.end, min_iterations_to + 1, 0,1000, &Map{
        width: map.width,
        height: map.height,
        start: map.end,
        end: map.start,
    }, &weatherman, &mut HashMap::<(usize, Coord), usize>::new(), true);

    let min_iterations_to_again = find_path(map.start, min_iterations_to + min_iterations_back + 2, 0, 1000, &map, &weatherman, &mut HashMap::<(usize, Coord), usize>::new(), false);

    println!("Part 2: {} + {} + {} = {}", min_iterations_to, min_iterations_back + 1, min_iterations_to_again + 1, min_iterations_to + min_iterations_back + min_iterations_to_again + 2)
}

fn render(map: &Map, forecast: &HashSet<Coord>) {
    for y in 0..map.height {
        for x in 0..map.width {
            if forecast.contains(&Coord { x: x + 1, y: y + 1 }) {
                print!("#");

                continue;
            }

            print!(".")
        }

        println!();
    }
}

fn find_path(cur_pos: Coord, offset: usize, iteration: usize, shortest_known_path: usize, map: &Map, weatherman: &BlizzardPrediction, cache: &mut HashMap<(usize, Coord), usize>, reverse: bool) -> usize {
    let cache_key = (iteration, cur_pos);

    match cache.get(&cache_key) {
        None => {}
        Some(val) => {
            return iteration + val;
        }
    }

    if iteration + cur_pos.distance_to(&map.end) >= shortest_known_path {
        return shortest_known_path;
    }

    if iteration >= shortest_known_path {
        return shortest_known_path;
    }

    if (!reverse && cur_pos.move_down() == map.end) || (reverse && cur_pos.move_up() == map.end) {
        return iteration + 1;
    }

    let prediction = weatherman.predict_for_iteration(iteration + offset + 1);

    let mut best_path = usize::MAX;

    if cur_pos == map.start {
        let first = if reverse {
            cur_pos.move_up()
        } else {
            cur_pos.move_down()
        };

        if !prediction.contains(&first) {
            best_path = min(best_path, find_path(first, offset,iteration + 1, min(best_path, shortest_known_path), map, weatherman, cache, reverse))
        }

        return min(best_path, find_path(first, offset, iteration + 1, min(best_path, shortest_known_path), map, weatherman, cache, reverse));
    }

    let mut next_positions = cur_pos.possible_next_positions();

    if reverse {
        next_positions.reverse()
    }

    for next in next_positions {
        if map.is_off(next) && next != map.start {
            continue;
        }

        if prediction.contains(&next) {
            continue;
        }

        best_path = min(best_path, find_path(next, offset, iteration + 1, min(best_path, shortest_known_path), map, weatherman, cache, reverse))
    }

    cache.insert(cache_key, best_path - iteration);

    return best_path;
}

fn parse_map(lines: &[String]) -> (Map, Vec<Blizzard>) {
    let mut blizzards = Vec::<Blizzard>::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '>' => { blizzards.push(Blizzard::new(Coord { x, y }, Direction::Right)) }
                '<' => { blizzards.push(Blizzard::new(Coord { x, y }, Direction::Left)) }
                '^' => { blizzards.push(Blizzard::new(Coord { x, y }, Direction::Up)) }
                'v' => { blizzards.push(Blizzard::new(Coord { x, y }, Direction::Down)) }
                _ => {}
            }
        }
    }

    let height = lines.len() - 2;
    let width = lines[0].len() - 2;
    let start = Coord { x: 1, y: 0 };
    let end = Coord { x: width, y: height + 1 };
    let map = Map { width, height, start, end };

    return (map, blizzards);
}

struct Map {
    width: usize,
    height: usize,
    start: Coord,
    end: Coord,
}

impl Map {
    fn is_off(&self, pos: Coord) -> bool {
        pos.x < 1 || pos.x > self.width || pos.y < 1 || pos.y > self.height
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn move_right(&self) -> Self {
        return Coord {
            x: self.x + 1,
            y: self.y,
        };
    }

    fn move_left(&self) -> Self {
        return Coord {
            x: self.x - 1,
            y: self.y,
        };
    }

    fn move_down(&self) -> Self {
        return Coord {
            x: self.x,
            y: self.y + 1,
        };
    }

    fn move_up(&self) -> Self {
        return Coord {
            x: self.x,
            y: self.y - 1,
        };
    }

    // TODO: Make this an iter
    fn possible_next_positions(&self) -> [Coord; 5] {
        return [self.move_down(), self.move_right(), self.move_up(), self.move_left(), *self];
    }

    // fn move_to(&self, dir: &Direction) -> Self {
    //     match dir {
    //         Direction::Right => self.move_right(1),
    //         Direction::Left => self.move_left(1),
    //         Direction::Up => self.move_up(1),
    //         Direction::Down => self.move_down(1)
    //     }
    // }

    fn distance_to(&self, other: &Self) -> usize {
        ((self.x as i32 - other.x as i32).abs() + (self.y as i32 - other.y as i32).abs()) as usize
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

struct Blizzard {
    pos: Coord,
    dir: Direction,
}

impl Blizzard {
    pub fn new(pos: Coord, dir: Direction) -> Self {
        Self { pos, dir }
    }
}

struct BlizzardPrediction {
    forecasts: Vec<HashSet<Coord>>,
    forecasts_repeat_after: usize,
}

impl BlizzardPrediction {
    fn new(blizzards: &[Blizzard], width: usize, height: usize) -> Self {
        let forecasts_repeat_after = height * width;

        let vec = Vec::<HashSet<Coord>>::from_iter((0..forecasts_repeat_after).map(|iteration| {
            HashSet::<Coord>::from_iter(blizzards.iter().map(|blizzard| {
                let (x, y) = match blizzard.dir {
                    Direction::Right => {
                        (((blizzard.pos.x - 1 + iteration) % width) + 1, blizzard.pos.y)
                    }
                    Direction::Left => {
                        ((((blizzard.pos.x as i32 - 1 - iteration as i32) % width as i32) + width as i32) as usize % width + 1, blizzard.pos.y)
                    }
                    Direction::Up => {
                        (blizzard.pos.x, (((blizzard.pos.y as i32 - 1 - iteration as i32) % height as i32) + height as i32) as usize % height + 1)
                    }
                    Direction::Down => {
                        (blizzard.pos.x, ((blizzard.pos.y - 1 + iteration) % height) + 1)
                    }
                };

                Coord { x, y }
            }))
        }));

        Self { forecasts: vec, forecasts_repeat_after }
    }

    fn predict_for_iteration(&self, n: usize) -> &HashSet<Coord> {
        self.forecasts.get(n % self.forecasts_repeat_after).unwrap()
    }
}