use std::fs::File;
use std::io::{BufReader, BufRead};
use scanf::sscanf;
use std::collections::HashMap;

fn main() {
    const INPUT: &str = "input";
    println!("{}", get_answer(INPUT));
}

#[derive(Debug, Clone, Default)]
struct Valve {
    flow_rate: u64,
    next: Vec<String>,
}

type Valves = HashMap<String,Valve>;

fn get_answer(file: &str) -> u64 {
    let mut valves = Valves::new();
    for line in BufReader::new(File::open(file).unwrap()).lines().map(|x| x.unwrap()) {
        let (valve_str, valve) = parse_line(&line);
        valves.insert(valve_str, valve);
    }
    calculate_released(&valves, &valves.get("AA").unwrap(), 0, 0, 0)
}

fn calculate_released(valves: &Valves, start: &Valve, minute: u32, flow_rate: u64, released: u64) -> u64 {
    println!("Minute: {}, flow_rate: {}, released: {}", minute, flow_rate, released);
    let mut new_minute = minute + 1;
    let mut max_released: u64 = 0;
    for open_valve in [true, false] {
        if open_valve && start.flow_rate > 0 {
            new_minute += 1;
            for next in &start.next {
                let Some(next_valve) = valves.get(next) else {
                    panic!("{}: Could not find valve!", next);
                };
                let new_released = calculate_released(
                    valves, &next_valve, new_minute, flow_rate + start.flow_rate, released + flow_rate + flow_rate);
                if new_released > max_released {
                    max_released = new_released;
                }
            }
        } else {
            for next in &start.next {
                let Some(next_valve) = valves.get(next) else {
                    panic!("{}: Could not find valve!", next);
                };
                let new_released = calculate_released(
                    valves, &next_valve, new_minute, flow_rate, released + flow_rate);
                if new_released > max_released {
                    max_released = new_released;
                }
            }
        }
    }
    max_released
}

// Valve VR has flow rate=11; tunnels lead to valves LH, KV, BP
fn parse_line(line: &str) -> (String, Valve) {
    println!("{}", line);
    let parts: Vec<&str> = line.split("; ").collect();
    let mut valve_idx = String::new();
    let mut flow_rate: u64 = 0;

    println!("{:?}", parts);
    sscanf!(parts[0], "Valve {} has flow rate={}", valve_idx, flow_rate).expect(parts[0]);

    let mut valve = Valve { flow_rate: flow_rate, next: Default::default() };
    let words: Vec<&str> = parts[1].split(" ").collect();
    for next in words {
        let chars: Vec<char> = next.chars().collect();
        println!("{} -> {:?}", next, chars);
        if chars[0] < 'A' || chars[0] > 'Z' {
            continue;
        }
        let next_str = format!("{}{}", chars[0], chars[1]);
        valve.next.push(next_str);
    }
    (valve_idx, valve)
}

#[cfg(test)]
#[test]
fn test_input() {
    const INPUT_FILE: &str = "test";

    assert_eq!(get_answer(INPUT_FILE), 1651 as u64);
}
