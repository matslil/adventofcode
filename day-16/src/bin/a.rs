use std::fs::File;
use std::io::{BufReader, BufRead};
use scanf::sscanf;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct ValveId(u8, u8);

impl fmt::Debug for ValveId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.0 as char, self.1 as char)
    }
}

const START_VALVE: ValveId = ValveId('A' as u8, 'A' as u8);

const END_MINUTE: u32 = 30;

fn main() {
    const INPUT: &str = "input";
    println!("{}", get_answer(INPUT));
}

#[derive(Debug, Clone, Default)]
struct Valve {
    flow: u64,
    next: Vec<ValveId>,
}

type Valves = HashMap<ValveId,Valve>;

#[derive(Debug, Eq)]
struct QueueNode {
    minute: u32,
    valve_id: ValveId,
    current: u64,
    flow: u64,
    valves_opened: Vec<ValveId>,
    path: String,
}

impl PartialEq for QueueNode {
    fn eq(&self, rhs: &Self) -> bool {
        self.valve_id == rhs.valve_id && self.minute == rhs.minute && self.current == rhs.current && self.flow == rhs.flow && self.valves_opened == rhs.valves_opened
    }
}

impl PartialOrd for QueueNode {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Ord for QueueNode {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.minute.cmp(&rhs.minute)
    }
}

fn get_answer(file: &str) -> u64 {
    let mut valves = Valves::new();
    for line in BufReader::new(File::open(file).unwrap()).lines().map(|x| x.unwrap()) {
        let (valve_str, valve) = parse_line(&line);
        valves.insert(valve_str, valve);
    }

    let mut todo = Vec::new();
    todo.push(QueueNode {
        valve_id: START_VALVE,
        minute: 1,
        current: 0,
        flow: 0,
        valves_opened: Vec::new(),
        path: valve_id_str(START_VALVE),
    });

    println!("{:?}", valves);

    let mut count = 0;

    while let Some(todo_entry) = todo.pop() {
        if todo_entry.minute == END_MINUTE {
            break;
        }
        println!("\nPopped: {:?}", todo_entry);
        for valve_id in valves.get(&todo_entry.valve_id).unwrap().next.clone() {
            let mut current = todo_entry.current + todo_entry.flow;
            let mut flow = todo_entry.flow;
            let mut minute = todo_entry.minute + 1;

            if minute > END_MINUTE {
                continue;
            }

            let new_entry = QueueNode {
                valve_id,
                minute,
                current,
                flow,
                valves_opened: todo_entry.valves_opened.clone(),
                path: format!("{}:{:?}", todo_entry.path, valve_id),
            };

            println!("Pushed: {:?}", new_entry);

            todo.push(new_entry);

            let entry_flow = valves.get(&todo_entry.valve_id).unwrap().flow;
//            println!("{:?}: Flow = {}", todo_entry.valve_id, entry_flow);

            current += flow;
            flow += entry_flow;
            minute += 1;

            if entry_flow == 0 || todo_entry.valves_opened.contains(&todo_entry.valve_id) || minute > END_MINUTE {
                continue;
            }

            let mut valves_opened = todo_entry.valves_opened.clone();
            valves_opened.sort();
            valves_opened.push(todo_entry.valve_id);

            let new_entry = QueueNode {
                valve_id,
                minute,
                current,
                flow,
                valves_opened,
                path: format!("{}:{:?}", todo_entry.path, valve_id),
            };

//            println!("Opening valve on {:?}", valve_id_str(todo_entry.valve_id));

            println!("Pushed: {:?}", new_entry);

            todo.push(new_entry);
        }

        let size_before = todo.len();
        todo.sort();
        todo.reverse();
        todo.dedup();
        println!("Removed {} duplicates", size_before - todo.len());

        count += 1;
        if count % 100 == 0 {
            let mut items_removed = 0;
            while remove_non_performing_node(&mut todo) { items_removed += 1 };
            println!("Count: {}, Queue size: {}, Removed: {}", count, todo.len(), items_removed);
            let lens = todo.iter().map(|e| e.valves_opened.len()).collect::<Vec<_>>();
            if lens.into_iter().fold(true, |acc, e| if acc && e == valves.len() { true } else { false }) {
                break;
            }
        }
    }

    for (idx, node) in todo.iter().enumerate() {
        if node.minute != END_MINUTE {
            println!("{}: {:?}", idx, node);
        }
    }

    let mut currents = Vec::new();

    for entry in todo {
        if entry.minute < END_MINUTE {
            // Assume we break before finished because all valves are opened
            let minutes_left = END_MINUTE - entry.minute;
            let current = entry.current + entry.flow * minutes_left as u64;
            currents.push(current);
        } else {
            currents.push(entry.current);
        }
    }

    currents.into_iter().max().unwrap()
}

fn remove_non_performing_node(queue: &mut Vec<QueueNode>) -> bool {
    let mut selected: Option<usize> = Option::None;
    let mut max_current = 0_u64;
    let mut min_current = u64::MAX;

    for (idx, node) in queue.iter().enumerate() {
        if node.current > max_current {
            max_current = node.current;
        }
        if node.current < min_current {
            min_current = node.current;
            selected = Some(idx);
        }

        if selected.is_some() {
            if max_current > 1000 {
                if (max_current * 4) > (min_current * 5) {
                    queue.remove(selected.unwrap());
                    return true;
                }
            } else if max_current > 100 {
                if (max_current * 3) > (min_current * 4) {
                    queue.remove(selected.unwrap());
                    return true;
                }
            }
        }
    }
    false
}

fn valve_id_str(valve_id : ValveId) -> String {
    format!("{}{}", valve_id.0 as char, valve_id.1 as char)
}

// Valve VR has flow rate=11; tunnels lead to valves LH, KV, BP
fn parse_line(line: &str) -> (ValveId, Valve) {
    println!("{}", line);
    let parts: Vec<&str> = line.split("; ").collect();
    let mut valve_idx = String::new();
    let mut flow: u64 = 0;

    println!("{:?}", parts);
    sscanf!(parts[0], "Valve {} has flow rate={}", valve_idx, flow).expect(parts[0]);

    let valve_idx_chars = valve_idx.chars().collect::<Vec<_>>();
    let mut valve = Valve { flow, next: Default::default() };
    let words: Vec<&str> = parts[1].split(" ").collect();
    for next in words {
        let chars: Vec<char> = next.chars().collect();
        println!("{} -> {:?}", next, chars);
        if chars[0] < 'A' || chars[0] > 'Z' {
            continue;
        }
        valve.next.push(ValveId(chars[0] as u8, chars[1] as u8));
    }
    (ValveId(valve_idx_chars[0] as u8, valve_idx_chars[1] as u8), valve)
}

#[cfg(test)]
#[test]
fn test_input() {
    const INPUT_FILE: &str = "test";

    // AA:DD:CC:BB:AA:II:JJ:II:AA:DD:EE:FF:GG:HH:GG:FF:EE:DD:CC:
    assert_eq!(get_answer(INPUT_FILE), 1651 as u64);
}
