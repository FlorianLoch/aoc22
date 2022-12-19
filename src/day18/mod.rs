use std::cmp::max;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, Lines};

pub fn solve(lines: &mut Lines<BufReader<File>>, test: bool) {
    let mut cubes: Vec<Cube> = lines.map(|line| {
        Cube::from_text(&line.expect("Failed to read line"))
    }).collect();

    // Move all cubes by 1,1,1 in order to get them away from the border; this required for the
    // algorithm in part 2 and does not affect computations for part 1
    for i in 0..cubes.len() {
        cubes[i] = cubes[i].move_rel(1, 1, 1);
    }

    let cubes_set: HashSet<&Cube> = cubes.iter().collect();

    let covered_sides = cubes.iter().fold(0, |acc, cube| {
        acc + cube.potentially_adjacent_cubes().iter().filter(|other| cubes_set.contains(other)).count()
    });

    // Vs.: Not sure which is nicer though...
    // let mut covered_sides = 0;
    //
    // for cube in &cubes {
    //     for other in &cube.potentially_adjacent_cubes() {
    //         if cubes_set.contains(other) {
    //             covered_sides += 1;
    //         }
    //     }
    // }

    let total_sides = cubes.len() * 6;

    println!("Part 1: Surface of droplet: {}", total_sides-covered_sides);

    let (max_x, max_y, max_z) = bounds(&cubes);

    let mut outsides = 0;

    let mut cubes_already_checked = HashSet::<Cube>::new();
    let mut cubes_to_check = Vec::<Cube>::new();

    // As we shifted all the droplet's cubes, these coordinates are guaranteed to not be part
    // of the droplet
    cubes_to_check.push(Cube::new(0, 0, 0));

    while !cubes_to_check.is_empty() {
        let cur_cube = cubes_to_check.pop().unwrap();

        if !cubes_already_checked.insert(cur_cube) {
            continue;
        }

        for adjacent in cur_cube.potentially_adjacent_cubes() {
            if cubes_set.contains(&adjacent) {
                outsides += 1;

                continue;
            }

            if adjacent.x > max_x + 1 || adjacent.y > max_y + 1 || adjacent.z > max_z + 1 {
                continue;
            }

            cubes_to_check.push(adjacent);
        }
    }

    println!("Part 2: Outer surface of droplet: {}", outsides);
}

#[derive(Hash, Debug, Eq, PartialEq, Copy, Clone)]
struct Cube {
    x: usize,
    y: usize,
    z: usize,
}

impl Cube {
    fn new(x: usize, y: usize, z: usize) -> Cube {
        Cube{ x, y, z }
    }

    fn move_rel(&self, x_offset: i32, y_offset: i32, z_offset: i32) -> Cube {
        Cube{
            x: (self.x as i32 + x_offset) as usize,
            y: (self.y as i32 + y_offset) as usize,
            z: (self.z as i32 + z_offset) as usize,
        }
    }

    fn from_text(s: &String) -> Cube {
        let splits: Vec<&str> = s.split(",").collect();

        if splits.len() != 3 {
            panic!("Cube coords not valid")
        }

        let x = splits[0].parse().expect("Failed to parse x");
        let y = splits[1].parse().expect("Failed to parse y");
        let z = splits[2].parse().expect("Failed to parse z");

        Self::new(x, y, z)
    }

    fn potentially_adjacent_cubes(&self) -> Vec<Cube> {
        let mut cubes = Vec::<Cube>::with_capacity(6);

        // on top
        cubes.push(self.move_rel(0, 1, 0));

        // below
        if self.y > 0 {
            cubes.push(self.move_rel(0, -1, 0));
        }

        // right
        cubes.push(self.move_rel(1, 0, 0));

        // left
        if self.x > 0 {
            cubes.push(self.move_rel(-1, 0, 0));
        }

        // behind
        cubes.push(self.move_rel(0, 0, 1));

        // in front
        if self.z > 0 {
            cubes.push(self.move_rel(0, 0, -1));
        }

        cubes
    }
}

fn bounds(cubes: &Vec<Cube>) -> (usize, usize, usize) {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_z = 0;

    for cube in cubes {
        max_x = max(max_x, cube.x);
        max_y = max(max_y, cube.y);
        max_z = max(max_z, cube.z);
    }

    (max_x, max_y, max_z)
}

