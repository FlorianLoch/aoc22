use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, Lines};

use regex::Regex;

use crate::puzzle;

pub fn solve(lines: &mut Lines<BufReader<File>>, test: bool) {
    let all_lines = puzzle::read_all_lines(lines);

    let (sensors_beacons, boundaries) = parse_signals(&all_lines);

    let x_offset = (boundaries.min_x * -1) as usize;
    let y_offset = (boundaries.min_y * -1) as usize;

    let width = (boundaries.max_x - boundaries.min_x + 1) as usize;
    let height = (boundaries.max_y - boundaries.min_y + 1) as usize;

    println!("Width: {}, Height: {}, X_Offset: {}, Y_Offset: {}", width, height, x_offset, y_offset);

    let mut sensors_and_beacons_coords = HashSet::<Coord>::new();

    for sb in &sensors_beacons {
        sensors_and_beacons_coords.insert(sb.0);
        sensors_and_beacons_coords.insert(sb.1);
    }

    if test {
        let mut map = Map::new(width, height, x_offset, y_offset);

        for pair in &sensors_beacons {
            map.add_sensor_and_beacon(pair.0, pair.1);
        }

        println!("{}", map.to_string());

        println!("Part 1: (SLOW) Positions guaranteed not to contain a beacon in y=10: {}", map.count_in_row(10, Thing::Nothing));

        let non_beacon_positions = count_non_beacon_positions_faster(10, &boundaries, &sensors_beacons, &sensors_and_beacons_coords);

        println!("Part 1: (FAST) Positions guaranteed not to contain a beacon in y=10: {}", non_beacon_positions);

        let possible_position = find_possible_unknown_beacon_positions_in_area_fast(&Boundaries { min_x: 0, max_x: 20, min_y: 0, max_y: 20 }, &sensors_beacons, &sensors_and_beacons_coords, true)[0];

        let tuning_frequency = (possible_position.x as u128) * (4_000_000 as u128) + (possible_position.y as u128);

        println!("Part 2: Tuning frequency of distress beacon: {}", tuning_frequency);
    } else {
        let non_beacon_positions = count_non_beacon_positions_faster(2000000, &boundaries, &sensors_beacons, &sensors_and_beacons_coords);

        println!("Positions guaranteed not to contain a beacon in y=2000000: {}", non_beacon_positions);

        let area = Boundaries {
            min_x: 0,
            max_x: 4_000_000,
            min_y: 0,
            max_y: 4_000_000,
        };

        let possible_position = find_possible_unknown_beacon_positions_in_area_fast(&area, &sensors_beacons, &sensors_and_beacons_coords, true)[0];

        let tuning_frequency = (possible_position.x as u128) * (4_000_000 as u128) + (possible_position.y as u128);

        println!("Part 2: Tuning frequency of distress beacon: {}", tuning_frequency);
    }

    return;
}

fn count_non_beacon_positions_faster(y: i32, boundaries: &Boundaries, sensor_beacons: &Vec<(Coord, Coord)>, sensors_and_beacons_coords: &HashSet<Coord>) -> i32 {
    let width = boundaries.max_x - boundaries.min_x + 1;

    let unknown_positions_count = find_possible_unknown_beacon_positions_in_area_fast(&Boundaries{
        min_x: boundaries.min_x,
        max_x: boundaries.max_x,
        min_y: y,
        max_y: y,
    }, sensor_beacons, sensors_and_beacons_coords, false).len();

    let mut beacons_in_row = HashSet::<Coord>::new();

    for sb in sensor_beacons {
        if sb.1.y == y {
            beacons_in_row.insert(sb.1);
        }
    }

    return width - unknown_positions_count as i32 - beacons_in_row.len() as i32;
}

fn find_possible_unknown_beacon_positions_in_area_fast(area: &Boundaries, sensor_beacons: &Vec<(Coord, Coord)>, sensors_and_beacons_coords: &HashSet<Coord>, short_circuit: bool) -> Vec<Coord> {
    // TODO: Prune unrelated sensors that do not touch the given area

    let mut possible_positions = Vec::<Coord>::new();

    for y in area.min_y..=area.max_y {
        let mut x = area.min_x;

        loop {
            if x > area.max_x {
                break;
            }

            let current_pos = Coord { x, y };

            if sensors_and_beacons_coords.contains(&current_pos) {
                // Improvement: Make this a map linking sensor to its strength/its beacon in order
                // to jump further than just by 1
                x += 1;

                continue;
            }

            let mut max_remaining_strength = -1;

            for sb in sensor_beacons {
                let strength = sb.0.manhattan_distance_to(sb.1);
                let mut remaining_strength = strength - sb.0.manhattan_distance_to(current_pos);

                if remaining_strength < 0 {
                    continue;
                }

                if sb.0.x > current_pos.x {
                    let distance = sb.0.manhattan_distance_to(current_pos);

                    remaining_strength = (distance - (sb.0.y - current_pos.y).abs()) * 2;
                }

                max_remaining_strength = max(max_remaining_strength, remaining_strength);
            }

            if max_remaining_strength >= 0 {
                x += max_remaining_strength + 1;

                continue;
            }

            possible_positions.push(current_pos);

            if short_circuit {
                return possible_positions;
            }

            x += 1;
        }
    }

    return possible_positions;
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn manhattan_distance_to(&self, other: Self) -> i32 {
        return (self.x - other.x).abs() + (self.y - other.y).abs();
    }

    fn move_relative(&self, x: i32, y: i32) -> Self {
        return Coord {
            x: self.x + x,
            y: self.y + y,
        };
    }
}

struct Boundaries {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

fn parse_signals(lines: &Vec<String>) -> (Vec<(Coord, Coord)>, Boundaries) {
    let mut sensors_beacons = Vec::<(Coord, Coord)>::new();

    // These initial values could lead to issues in case the input is, e.g., very far to the "right".
    // That would cause a too big boundary box.
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;

    let re = Regex::new(r"Sensor at x=([-\d]*), y=([-\d]*): closest beacon is at x=([-\d]*), y=([-\d]*)").expect("Failed to create RegEx");


    for (i, line) in lines.iter().enumerate() {
        let cap = re.captures(line);

        if cap.is_none() {
            panic!("Did not find sensor and beacon position in line")
        }

        let cap = cap.unwrap();

        let sensor_x = cap[1].parse().expect("Failed to parse sensor's x value");
        let sensor_y = cap[2].parse().expect("Failed to parse sensor's y value");
        let beacon_x = cap[3].parse().expect("Failed to parse beacon's x value");
        let beacon_y = cap[4].parse().expect("Failed to parse beacon's y value");

        let sensor = Coord { x: sensor_x, y: sensor_y };
        let beacon = Coord { x: beacon_x, y: beacon_y };

        sensors_beacons.push((sensor, beacon));

        let distance = sensor.manhattan_distance_to(beacon);

        // Set the initial values
        if i == 0 {
            min_x = sensor_x - distance;
            min_y = sensor_y - distance;
            max_x = sensor_x + distance;
            max_y = sensor_y + distance;

            continue;
        }

        min_x = min(min_x, sensor_x - distance);
        min_y = min(min_y, sensor_y - distance);
        max_x = max(max_x, sensor_x + distance);
        max_y = max(max_y, sensor_y + distance);
    }

    return (sensors_beacons, Boundaries { min_x, max_x, min_y, max_y });
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Thing {
    Beacon,
    Sensor,
    Nothing,
    Unknown,
}

struct Map {
    map: Vec<Thing>,
    x_offset: usize,
    y_offset: usize,
    width: usize,
    height: usize,
}

impl Map {
    fn new(width: usize, height: usize, x_offset: usize, y_offset: usize) -> Self {
        // let map: Vec<Thing> = (0..height * width).map(|_| Thing::Unknown).collect();

        let mut map = Vec::<Thing>::with_capacity(height * width);

        for _ in 0..height * width {
            map.push(Thing::Unknown);
        }

        return Map { map, x_offset, y_offset, width, height };
    }

    fn add_sensor_and_beacon(&mut self, sensor: Coord, beacon: Coord) {
        let distance = sensor.manhattan_distance_to(beacon);

        for i in -distance..=distance {
            for j in -distance..=distance {
                let c = sensor.move_relative(i, j);

                if c.manhattan_distance_to(sensor) <= distance {
                    if self.at(c) == Thing::Unknown {
                        self.set(c, Thing::Nothing);
                    }
                }
            }
        }

        self.set(sensor, Thing::Sensor);
        self.set(beacon, Thing::Beacon);
    }

    fn to_string(&self) -> String {
        let mut str_buf = String::new();

        for i in 0..self.height {
            str_buf.push_str(&(i as i32 - self.y_offset as i32).to_string());
            str_buf.push('\t');

            for j in 0..self.width {
                str_buf.push(match self.map[i * self.width + j] {
                    Thing::Beacon => { 'B' }
                    Thing::Sensor => { 'S' }
                    Thing::Nothing => { '#' }
                    Thing::Unknown => { '.' }
                })
            }

            str_buf.push('\n');
        }

        return str_buf;
    }

    fn at(&self, c: Coord) -> Thing {
        let y = (c.y + self.y_offset as i32) as usize;
        let x = (c.x + self.x_offset as i32) as usize;

        return self.map[y * self.width + x];
    }

    fn set(&mut self, at: Coord, thing: Thing) {
        let y = (at.y + self.y_offset as i32) as usize;
        let x = (at.x + self.x_offset as i32) as usize;

        self.map[y * self.width + x] = thing;
    }

    fn count_in_row(&self, y: i32, what: Thing) -> i32 {
        let mut hits = 0;

        for x in 0..self.width as i32 {
            if self.at(Coord { x: x - self.x_offset as i32, y }) == what {
                hits += 1;
            }
        }

        return hits;
    }
}
