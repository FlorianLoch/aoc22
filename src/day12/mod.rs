use std::cmp::{min, Ordering};
use std::fs::File;
use std::io::{BufReader, Lines};

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
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

    fn move_to(&self, dir: Direction) -> Self {
        match dir {
            Direction::Right => self.move_right(1),
            Direction::Left => self.move_left(1),
            Direction::Up => self.move_up(1),
            Direction::Down => self.move_down(1)
        }
    }

    fn euclidean_distance(&self, other: Self) -> f64 {
        return (((self.x as i64 - other.x as i64).pow(2) + (self.y as i64 - other.y as i64).pow(2)) as f64).sqrt();
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

struct Map {
    map: Vec<(char, i32)>,
    width: usize,
    height: usize,
}

impl Map {
    fn from_2d_vec(area: &Vec<Vec<char>>) -> Self {
        let height = area.len();
        if height == 0 {
            panic!("Given vector is empty")
        }

        let width = area[0].len();
        if width == 0 {
            panic!("First vector in vector is empty")
        }

        let mut map = Vec::with_capacity(height*width);

        for i in 0..area.len() {
            for j in 0..area[i].len() {
                map.push((area[i][j], i32::MAX));
            }
        }

        return Map{ map, width, height }
    }

    fn check_min_distance(&mut self, coord: Coord, distance: i32) -> bool {
        let current = self.at(coord);

        if distance >= current.1 {
            return false
        }

        self.map[coord.y*self.width+coord.x] = (current.0, distance);

        return true
    }

    fn level_at(&self, coord: Coord) ->  char {
        let (c, _) = self.at(coord);

        return c;
    }

    fn is_end_at(&self, coord: Coord) -> bool {
        return self.level_at(coord) == 'E';
    }

    fn at(&self, coord: Coord) -> (char, i32) {
        return self.map[coord.y*self.width+coord.x];
    }

    fn all_coords_with_level(&self, level: char) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();

        for i in 0..self.height {
            for j in 0..self.width {
                if self.map[i*self.width+j].0 == level {
                    coords.push(Coord{ x: j, y: i })
                }
            }
        }

        return coords;
    }
}

pub fn solve(lines: &mut Lines<BufReader<File>>, _: bool) {
    let mut area = Vec::<Vec<char>>::new();

    let mut start_position= Coord { x: 0, y: 0 };
    let mut end_position = Coord { x: 0, y: 0 };

    for (i, line) in lines.enumerate() {
        let mut row = Vec::<char>::new();

        for (j, c) in line.expect("Failed to read line").chars().enumerate() {
            row.push(c);

            if c == 'S' {
                start_position = Coord { x: j, y: i }
            }

            if c == 'E' {
                end_position = Coord { x: j, y: i }
            }
        }

        area.push(row);
    }

    start_position.y;

    let mut map = Map::from_2d_vec(&area);

    let min_distance_to_end = find_next(start_position, 0, &mut map, i32::MAX);

    println!("Part 1: Min steps required to get to the end: {}", min_distance_to_end);

    // PART 2
    let mut all_a_coords = map.all_coords_with_level('a');

    // Sort by distance to end, this should cause a little speed up
    // UPDATE: measured it, with the given data there is (almost) no benefit.
    all_a_coords.sort_by(|a, b| {
        let d_a = a.euclidean_distance(end_position);
        let d_b = b.euclidean_distance(end_position);

        return d_a.partial_cmp(&d_b).unwrap();
    });

    // By this we also handle considering 'S' as 'a'
    let mut min_distance_to_end_from_any_a = min_distance_to_end;

    for c in all_a_coords {
        let mut map = Map::from_2d_vec(&area);

        let d = find_next(c, 0, &mut map, min_distance_to_end_from_any_a);
        min_distance_to_end_from_any_a = min(d, min_distance_to_end_from_any_a)
    }

    println!("Part 2: Min steps required to get to the end from any 'a': {}", min_distance_to_end_from_any_a);
}

fn find_next(cur_pos: Coord, steps: i32, map: &mut Map, max_steps: i32) -> i32 {
    if steps == max_steps {
        return steps;
    }

    if !map.check_min_distance(cur_pos, steps) {
        return i32::MAX;
    }

    if map.is_end_at(cur_pos) {
        return steps;
    }

    let mut min_distance_to_end = i32::MAX; // defaults to unreachable

    let (left_pos, ok) = can_go(cur_pos, Direction::Left, &map);
    if ok {
        let distance = find_next(left_pos, steps + 1, map, max_steps);

        if distance >= 0 && distance < min_distance_to_end {
            min_distance_to_end = distance;
        }
    }

    let (right_pos, ok) = can_go(cur_pos, Direction::Right, &map);
    if ok {
        let distance = find_next(right_pos, steps + 1, map, max_steps);

        if distance >= 0 && distance < min_distance_to_end {
            min_distance_to_end = distance;
        }
    }

    let (up_pos, ok) = can_go(cur_pos, Direction::Up, &map);
    if ok {
        let distance = find_next(up_pos, steps + 1, map, max_steps);

        if distance >= 0 && distance < min_distance_to_end {
            min_distance_to_end = distance;
        }
    }

    let (down_pos, ok) = can_go(cur_pos, Direction::Down, &map);
    if ok {
        let distance = find_next(down_pos, steps + 1, map, max_steps);

        if distance >= 0 && distance < min_distance_to_end {
            min_distance_to_end = distance;
        }
    }

    return min_distance_to_end;
}

fn can_go(from: Coord, dir: Direction, map: &Map) -> (Coord, bool) {
    if dir == Direction::Left && from.x == 0 {
        return (from, false);
    }

    if dir == Direction::Up && from.y == 0 {
        return (from, false);
    }

    if dir == Direction::Right && from.x + 1 == map.width {
        return (from, false);
    }

    if dir == Direction::Down && from.y + 1 == map.height {
        return (from, false);
    }

    let to = from.move_to(dir);

    let mut from_level = map.level_at(from);
    let mut to_level = map.level_at(to);

    if from_level == 'S' {
        from_level = 'a';
    }

    if to_level == 'E' {
        to_level = 'z';
    }

    return (to, (to_level as i32 - from_level as i32) <= 1);
}
