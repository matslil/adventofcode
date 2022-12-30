use std::fs::File;
use std::io::{BufReader, BufRead};
use scanf::sscanf;
use std::collections::HashMap;
use slab_tree::{TreeBuilder, NodeId};
use std::iter;
use std::fmt::Display;
use std::fmt;
use std::ops;
use slab_tree::behaviors::RemoveBehavior::*;
use std::collections::VecDeque;

type ValveId = [u8; 2];

const START_VALVE: ValveId = ['A' as u8, 'A' as u8];

const END_MINUTE: u32 = 30;

fn main() {
    const INPUT: &str = "input";
    println!("{}", get_answer(INPUT));
}

#[derive(Debug, Clone, Default)]
struct Valve {
    flow_rate: u64,
    next: Vec<ValveId>,
}

type Valves = HashMap<ValveId,Valve>;

#[derive(Debug, Clone)]
struct TreeNode {
    path: String,
    minute: u32,
    current: Vec<u64>,
    flow: u64,
}

impl Display for TreeNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ {}: minute: {}, current: {:?}, flow: {} }}", self.path, self.minute, self.current, self.flow)
    }
}

struct ValveArray(pub Vec<TreeNode>);

impl ops::Deref for ValveArray {
    type Target = Vec<TreeNode>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for ValveArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.iter().fold(Ok(()), |result, album| {
            result.and_then(|_| writeln!(f, "{}", album))
        })
    }
}

struct QueueNode {
    node_id: NodeId,
    valve_id: ValveId,
}

fn get_answer(file: &str) -> u64 {
    let mut valves = Valves::new();
    for line in BufReader::new(File::open(file).unwrap()).lines().map(|x| x.unwrap()) {
        let (valve_str, valve) = parse_line(&line);
        valves.insert(valve_str, valve);
    }

    let mut tree = TreeBuilder::new().with_root(TreeNode {
        path: START_VALVE.into_iter().map(|c| c as char).collect::<String>(),
        minute: 0,
        current: vec![0],
        flow: 0
    }).build();
    let mut todo = VecDeque::new();
    let root_idx = tree.root_id().unwrap();
    todo.push_front(QueueNode {
        node_id: root_idx,
        valve_id: START_VALVE,
    });

    println!("{:?}", valves);

    let mut count = 0;

    while let Some(todo_entry) = todo.pop_back() {
        let tree_node_wrapped = tree.get_mut(todo_entry.node_id);

        if tree_node_wrapped.is_none() {
            // Removed
            continue;
        }

        let mut tree_node = tree_node_wrapped.unwrap();
//        println!("{}Popped: {}: {:?}", iter::repeat(' ').take(tree_node.data().minute as usize).collect::<String>(), valve_id_str(&todo_entry.valve_id), tree_node.data());
        for open_valve in [true, false] {
            for valve_id in &valves.get(&todo_entry.valve_id).unwrap().next {
                let (current, flow) = if open_valve {
                    (vec![tree_node.data().flow + tree_node.data().current.last().unwrap(), 2 * tree_node.data().flow + tree_node.data().current.last().unwrap()],
                     tree_node.data().flow + valves.get(&todo_entry.valve_id).unwrap().flow_rate)
                } else {
                    (vec![tree_node.data().flow + tree_node.data().current.last().unwrap()],
                     tree_node.data().flow)
                };
                let minute = if open_valve { tree_node.data().minute + 2 } else { tree_node.data().minute + 1 };
                let mut path: String;
                if let Some(mut parent) = tree_node.parent() {
                    path = (*valve_id).into_iter().map(|c| c as char).collect::<String>();
                    path.push_str(":");
                    path.push_str(&parent.data().path.clone());
                } else {
                    path = (*valve_id).into_iter().map(|c| c as char).collect::<String>();
                }

                if open_valve {
                    if minute > (END_MINUTE + 1) {
                        continue;
                    }
                } else {
                    if minute > END_MINUTE {
                        continue;
                    }
                }
                let node_flow = tree_node.data().flow;
                if let Some(node) = tree_node.as_ref().parent() {
                    if node.data().flow == node_flow {
                        if let Some(node) = node.parent() {
                            if node.data().flow == node_flow {
//                                println!("Cutting branch");
                                continue;
                            }
                        }
                    }
                }
                let new_tree_node = TreeNode {
                    path,
                    minute,
                    current,
                    flow
                };
//                println!("{}Pushed: {}: {:?}", iter::repeat(' ').take(tree_node.data().minute as usize).collect::<String>(), valve_id_str(&valve_id),  new_tree_node);
                let node_id = tree_node.append(new_tree_node).node_id();
                todo.push_front(QueueNode { node_id, valve_id: *valve_id });
            }
        }
        count += 1;
        if count % 100 == 0 {
            let mut items_removed = 0;
            while remove_non_performing_node(&mut tree) { items_removed += 1 };
            println!("Count: {}, Queue size: {}, Removed: {}", count, todo.len(), items_removed);
        }
    }

    let all_nodes = tree.root().unwrap().traverse_level_order().collect::<Vec<_>>();
    let all_data = ValveArray(all_nodes.iter().map(|n| n.data().clone()).collect::<Vec<TreeNode>>());
    println!("{:#}", all_data);
    let data = all_nodes.into_iter().filter(|n| n.data().minute == END_MINUTE + n.data().current.len() as u32 - 1).map(|n| n.data().current.clone()).flatten().collect::<Vec<_>>();
    println!("{:?}", data);
    data.into_iter().max().unwrap()
}

fn remove_non_performing_node(tree: &mut slab_tree::Tree<TreeNode>) -> bool {
    let mut selected: Option<NodeId> = Option::None;
    let mut max_current = 0_u64;
    let mut min_current = u64::MAX;

    if let Some(root) = tree.root() {
        let max_minute = root.traverse_level_order().map(|n| n.data().minute).max().unwrap();
        let filtered_current = root.traverse_level_order().filter(|n| n.data().minute == max_minute);

        for tree_node in filtered_current {
            let node = tree_node.data();
            let max = *node.current.iter().max().unwrap();
            let min = *node.current.iter().min().unwrap();
            if max > max_current {
                max_current = max;
            }
            if min < min_current {
                min_current = min;
                selected = Some(tree_node.node_id());
            }
        }

//        println!("{} {}", max_current, min_current);

        if selected.is_some() && max_current > 100 && (max_current * 5) > (min_current * 6) {
  //          println!("Removing one");
            tree.remove(selected.unwrap(), DropChildren);
            return true;
        }
    }
    false
}

fn valve_id_str(valve_id : &ValveId) -> String {
    format!("{}{}", valve_id[0] as char, valve_id[1] as char)
}

// Valve VR has flow rate=11; tunnels lead to valves LH, KV, BP
fn parse_line(line: &str) -> (ValveId, Valve) {
    println!("{}", line);
    let parts: Vec<&str> = line.split("; ").collect();
    let mut valve_idx = String::new();
    let mut flow_rate: u64 = 0;

    println!("{:?}", parts);
    sscanf!(parts[0], "Valve {} has flow rate={}", valve_idx, flow_rate).expect(parts[0]);

    let valve_idx_chars = valve_idx.chars().collect::<Vec<_>>();
    let mut valve = Valve { flow_rate: flow_rate, next: Default::default() };
    let words: Vec<&str> = parts[1].split(" ").collect();
    for next in words {
        let chars: Vec<char> = next.chars().collect();
        println!("{} -> {:?}", next, chars);
        if chars[0] < 'A' || chars[0] > 'Z' {
            continue;
        }
        valve.next.push([chars[0] as u8, chars[1] as u8]);
    }
    ([valve_idx_chars[0] as u8, valve_idx_chars[1] as u8], valve)
}

#[cfg(test)]
#[test]
fn test_input() {
    const INPUT_FILE: &str = "test";

    assert_eq!(get_answer(INPUT_FILE), 1651 as u64);
}
