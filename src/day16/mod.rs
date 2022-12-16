use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, Lines};
use std::process::Termination;

use regex::Regex;

use crate::puzzle;

const START_VALVE: &str = "AA";


pub fn solve(lines: &mut Lines<BufReader<File>>, test: bool) {
    let all_lines = puzzle::read_all_lines(lines);

    let valves_map = parse_valves(&all_lines);

    let start_valve = valves_map.get(START_VALVE).expect("Did not find start valve in map");

    let mut valves_by_flow_rate: Vec<&Valve> = valves_map.values().collect();
    valves_by_flow_rate.sort_by(|a, b| a.flow_rate.cmp(&b.flow_rate));
    valves_by_flow_rate.reverse();

    println!("{:?}", valves_by_flow_rate);

    // println!("Loop {}", has_loop(&"A".to_string(), &vec!["A".to_string(), "B".to_string(), "C".to_string()]));
    // println!("Loop {}", has_loop(&"A".to_string(), &vec!["A".to_string(), "B".to_string(), "A".to_string()]));
    // println!("Loop {}", has_loop(&"B".to_string(), &vec!["A".to_string(), "B".to_string(), "A".to_string()]));


    let mut remaining_valves = HashSet::<String>::new();
    for (valve_name, _) in &valves_map {
        remaining_valves.insert(valve_name.clone());
    }

    println!("{}", approximate_possible_max_pressure_release(30, &valves_by_flow_rate, &remaining_valves.clone()));

    let max_release_forecast = trace_path(
        &start_valve,
        30,
        0,
        0,
        &valves_map,
        remaining_valves,
        &valves_by_flow_rate,
        Vec::<String>::with_capacity(30));

    println!("Part 1: Forecast maximum pressure release: {}", max_release_forecast);

    return;
}

fn approximate_possible_max_pressure_release(mut time_remaining: i32, valves_by_flow_rate: &Vec<&Valve>, remaining_valves: &HashSet<String>) -> i32 {
    let mut approximation = 0;

    for valve in valves_by_flow_rate {
        if time_remaining == 1 {
            break;
        }

        if !remaining_valves.contains(&valve.name) {
            continue;
        }

        approximation += (time_remaining - 1) * valve.flow_rate;

        time_remaining -= 2;
    }

    return approximation;
}

fn has_loop(name: &String, path: &Vec<String>) -> bool {
    // Ceiling division the hard way
    let len = path.len();

    'outer: for i in 1..=(len + 2 - 1) / 2 {
        if path[len - i] == *name {
            for j in 1..i {
                if path[len - j] != path[len - i - j] {
                    continue 'outer;
                }
            }

            return true;
        }
    }

    return false;
}

fn trace_path(cur_valve: &Valve, time_remaining: i32, release_forecast: i32, max_release_forecast: i32, map: &HashMap<String, Valve>, mut remaining_valves: HashSet<String>, valves_by_flow_rate: &Vec<&Valve>, mut path: Vec<String>) -> i32 {
    if time_remaining <= 1 { // || release_forecast + approximate_possible_max_pressure_release(time_remaining, valves_by_flow_rate, &remaining_valves) <= max_release_forecast
        return release_forecast;
    }

    if has_loop(&cur_valve.name, &path) {
        return release_forecast;
    }

    path.push(cur_valve.name.clone());

    // // TODO: Does not take already visited/still closed valves into account, therefore not valid
    // match traces.get(&(cur_valve.name.clone(), time_remaining)) {
    //     None => {}
    //     Some(remembered_max_forecast) => {
    //         return *remembered_max_forecast;
    //     }
    // }

    let mut max_forecast = release_forecast;

    // Option 1: open the valve in the current room, takes an additional minute
    if cur_valve.flow_rate > 0 && remaining_valves.contains(&cur_valve.name) {
        remaining_valves.remove(&cur_valve.name);

        let forecast_open = release_forecast + (time_remaining - 1) * cur_valve.flow_rate;

        for neighbor in &cur_valve.neighbors {
            let neighbor_valve = map.get(neighbor).expect("Neighbor valve not found in map");

            max_forecast = max(max_forecast, trace_path(neighbor_valve, time_remaining - 2, forecast_open, max_release_forecast, map, remaining_valves.clone(), valves_by_flow_rate, path.clone()))
        }
    }

    // Option 2: move on straight away to any of the linked valves
    for neighbor in &cur_valve.neighbors {
        let neighbor_valve = map.get(neighbor).expect("Neighbor valve not found in map");

        max_forecast = max(max_forecast, trace_path(neighbor_valve, time_remaining - 1, release_forecast, max_release_forecast, map, remaining_valves.clone(), valves_by_flow_rate, path.clone()))
    }

    // traces.insert((cur_valve.name.clone(), time_remaining), max_forecast);

    return max_forecast;
}

#[derive(Debug, Clone)]
struct Valve {
    neighbors: Vec<String>,
    name: String,
    flow_rate: i32,
}

impl Valve {
    fn new(name: String, flow_rate: i32) -> Valve {
        Valve {
            neighbors: vec![],
            name,
            flow_rate,
        }
    }

    fn add_neighbor(&mut self, neighbor: String) {
        self.neighbors.push(neighbor);
    }
}

fn parse_valves(lines: &Vec<String>) -> HashMap<String, Valve> {
    let mut valves_map = HashMap::<String, Valve>::new();

    let re = Regex::new(r"Valve (.*) has flow rate=(\d*); tunnels? leads? to valves? (.*)").expect("Failed to create RegEx");

    // First, create all valves
    for line in lines {
        let cap = re.captures(line).unwrap();

        let valve_name = cap.get(1).expect("Failed to match valve name").as_str();
        let flow_rate = cap.get(2).expect("Failed to match flow rate").as_str().parse().expect("Failed to parse flow rate");

        let valve = Valve::new(valve_name.to_string(), flow_rate);

        valves_map.insert(valve_name.to_string(), valve);
    }

    // Second, link them
    for line in lines {
        let cap = re.captures(line).unwrap();

        let valve_name = cap.get(1).expect("Failed to match valve name").as_str();
        let neighbors = cap.get(3).expect("Failed to match neighbors").as_str().split(", ");

        let mut neighbors: Vec<&str> = neighbors.collect();

        neighbors.sort_by(|a, b| {
            valves_map.get(*a).unwrap().flow_rate.cmp(&valves_map.get(*b).unwrap().flow_rate)
        });

        neighbors.reverse();

        let valve = valves_map.get_mut(valve_name).expect("No valve with name found");

        for neighbor in neighbors {
            valve.add_neighbor(neighbor.to_string());
        }
    }

    return valves_map;
}