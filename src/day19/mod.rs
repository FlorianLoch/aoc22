use std::cmp::max;
use std::fs::File;
use std::io::{BufReader, Lines};

use regex::Regex;

pub fn solve(lines: &mut Lines<BufReader<File>>, test: bool) {
    let blueprints: Vec<Blueprint> = lines.map(|line| {
        Blueprint::from_text(&line.expect("Failed to read line"))
    }).collect();

    let mut sum_quality_levels = 0;

    for (idx, blueprint) in blueprints.iter().enumerate() {
        let geode_production = blueprint.simulate(24, &Robots{
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
        }, 0, 0, 0, 0);

        println!("Blueprint {}: Geode production: {}", idx + 1, geode_production);

        sum_quality_levels += (idx + 1) * geode_production as usize;
    }

    println!("Part 1: Sum of blueprints' quality levels: {}", sum_quality_levels);
}

#[derive(Debug, Copy, Clone)]
struct Blueprint {
    costs_ore_robot: Costs,
    costs_clay_robot: Costs,
    costs_obsidian_robot: Costs,
    costs_geode_robot: Costs,
}

impl Blueprint {
    fn from_text(s: &String) -> Blueprint {
        let re = Regex::new(r"Blueprint [\d]+: Each ore robot costs ([\d]+) ore\. Each clay robot costs ([\d]+)* ore\. Each obsidian robot costs ([\d]+) ore and ([\d]+) clay\. Each geode robot costs ([\d]+) ore and ([\d]+) obsidian\.").expect("Failed to create RegEx");

        let captures = re.captures(s);

        if captures.is_none() {
            panic!("Failed to match line with RegEx");
        }

        let captures = captures.unwrap();

        return Blueprint {
            costs_ore_robot: Costs {
                ore: captures[1].parse().expect("Failed to parse ore costs for ore robot"),
                clay: 0,
                obsidian: 0,
            },
            costs_clay_robot: Costs {
                ore: captures[2].parse().expect("Failed to parse ore costs for clay robot"),
                clay: 0,
                obsidian: 0,
            },
            costs_obsidian_robot: Costs {
                ore: captures[3].parse().expect("Failed to parse ore costs for obsidian robot"),
                clay: captures[4].parse().expect("Failed to parse clay costs for obsidian robot"),
                obsidian: 0,
            },
            costs_geode_robot: Costs {
                ore: captures[5].parse().expect("Failed to parse ore costs for geode robot"),
                clay: 0,
                obsidian: captures[6].parse().expect("Failed to parse obsidian costs for geode robot"),
            },
        };
    }

    fn simulate(&self, mut minutes_remaining: usize, robots_available: &Robots, ore_available: i32, clay_available: i32, obsidian_available: i32, geodes_collected: i32) -> i32 {
        // ore_available += robots_available.ore_robots;
        // clay_available += robots_available.clay_robots;
        // obsidian_available += robots_available.obsidian_robots;
        // geodes_collected += robots_available.geode_robots;

        if minutes_remaining <= 1 || (minutes_remaining <= 2 && robots_available.geode_robots == 0 && (ore_available < self.costs_geode_robot.ore || obsidian_available < self.costs_geode_robot.obsidian)) {
            return geodes_collected + minutes_remaining as i32 * robots_available.geode_robots;
        }

        if minutes_remaining == 1 {
            return geodes_collected + robots_available.geode_robots;
        }

        minutes_remaining -= 1;

        let mut max_geode_production = self.simulate(minutes_remaining, robots_available, ore_available+robots_available.ore_robots, clay_available+robots_available.clay_robots, obsidian_available+robots_available.obsidian_robots, geodes_collected+robots_available.geode_robots);

        if ore_available >= self.costs_ore_robot.ore {
            let mut robots_available_next_minute = *robots_available;

            robots_available_next_minute.ore_robots += 1;

            max_geode_production = max(max_geode_production, self.simulate(minutes_remaining, &robots_available_next_minute, ore_available+robots_available.ore_robots-self.costs_ore_robot.ore, clay_available+robots_available.clay_robots, obsidian_available+robots_available.obsidian_robots, geodes_collected+robots_available.geode_robots));
        }

        if ore_available >= self.costs_clay_robot.ore {
            let mut robots_available_next_minute = *robots_available;

            robots_available_next_minute.clay_robots += 1;

            max_geode_production = max(max_geode_production, self.simulate(minutes_remaining, &robots_available_next_minute, ore_available+robots_available.ore_robots-self.costs_clay_robot.ore, clay_available+robots_available.clay_robots, obsidian_available+robots_available.obsidian_robots, geodes_collected+robots_available.geode_robots));
        }

        // if ore_available >= self.costs_obsidian_robot.ore && clay_available >= self.costs_obsidian_robot.clay && minutes_remaining as i32 - 2 > self.costs_geode_robot.obsidian {
        if ore_available >= self.costs_obsidian_robot.ore && clay_available >= self.costs_obsidian_robot.clay {
            let mut robots_available_next_minute = *robots_available;

            robots_available_next_minute.obsidian_robots += 1;

            max_geode_production = max(max_geode_production, self.simulate(minutes_remaining, &robots_available_next_minute, ore_available+robots_available.ore_robots-self.costs_obsidian_robot.ore, clay_available+robots_available.clay_robots-self.costs_obsidian_robot.clay, obsidian_available+robots_available.obsidian_robots, geodes_collected+robots_available.geode_robots));
        }

        if ore_available >= self.costs_geode_robot.ore && obsidian_available >= self.costs_geode_robot.obsidian {
            let mut robots_available_next_minute = *robots_available;

            robots_available_next_minute.geode_robots += 1;

            max_geode_production = max(max_geode_production, self.simulate(minutes_remaining, &robots_available_next_minute, ore_available+robots_available.ore_robots-self.costs_geode_robot.ore, clay_available+robots_available.clay_robots, obsidian_available+robots_available.obsidian_robots-self.costs_geode_robot.obsidian, geodes_collected+robots_available.geode_robots));
        }

        return max_geode_production;
    }
}

#[derive(Debug, Copy, Clone)]
struct Robots {
    ore_robots: i32,
    clay_robots: i32,
    obsidian_robots: i32,
    geode_robots: i32,
}

#[derive(Debug, Copy, Clone)]
struct Costs {
    ore: i32,
    clay: i32,
    obsidian: i32,
}



