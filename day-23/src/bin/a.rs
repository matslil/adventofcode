use tracing_subscriber::{filter, prelude::*};
use std::{fs::File, sync::Arc};
use tracing::{info, debug};
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use sorted_vec::SortedSet;
use itertools::Itertools;
fn setup_tracing() {
    let stdout_log = tracing_subscriber::fmt::layer()
        .pretty();

    // A layer that logs events to a file.
    let file = File::create("debug.log");
    let file = match file  {Ok(file) => file,Err(error) => panic!("Error: {:?}",error),};
    let debug_log = tracing_subscriber::fmt::layer()
        .with_writer(Arc::new(file));

    tracing_subscriber::registry()
        .with(
            stdout_log
                .with_filter(filter::LevelFilter::INFO)
                .and_then(debug_log)
        )
        .init();
}

fn main() {
    setup_tracing();
    info!("{:?}", get_answer("input"));
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
struct Computer
{
    name: [char;2],
}

impl std::fmt::Display for Computer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.name[0], self.name[1])
    }
}

impl Computer {
    fn from(s: &str) -> Self {
        let mut i = s.chars();
        Self { name: [i.next().unwrap(), i.next().unwrap()] }
    }
}

fn get_answer(file: &str) -> usize {
    let mut tnodes: Vec<Computer> = Vec::new();
    let input: HashMap<Computer, Vec<Computer>> = BufReader::new(File::open(file).unwrap()).lines()
        .filter_map(Result::ok)
        .map(|l| l.split('-')
            .map(|w| Computer::from(w))
            .collect())
        .fold(HashMap::new(), |mut map: HashMap<Computer, Vec<Computer>>, entry: Vec<Computer>| {
            if entry[0].name[0] == 't' {
                tnodes.push(entry[0].clone());
            }
            if let Some(existing) = map.get_mut(&entry[0]) {
                existing.push(entry[1].clone());
            } else {
                map.insert(entry[0].clone(), vec![entry[1]]);
            }
            if let Some(existing) = map.get_mut(&entry[1]) {
                existing.push(entry[0].clone());
            } else {
                map.insert(entry[1].clone(), vec![entry[0]]);
            }
            map
        });


    let mut set3: Vec<SortedSet<Computer>> = Vec::new();

    for tnode in &tnodes {
        let mut all = input[tnode].clone();
        all.push(*tnode);
        for win in all.into_iter().combinations(3) {
            let sorted_win: SortedSet<Computer> = win.to_vec().into();
            if win.contains(&tnode) && !set3.contains(&sorted_win) {
                set3.push(sorted_win);
            }
        }
    }

    let mut checked: Vec<SortedSet<Computer>> = Vec::new();
    let mut result = 0usize;
    for set in set3 {
        debug!("{},{},{}", set[0], set[1], set[2]);
        if !checked.contains(&set) {
            checked.push(set);
            debug!("|-- Added");
            result += 1;
        }
    }

    return result;
}

#[test]
fn test() {
    setup_tracing();
    assert_eq!(7, get_answer("test.a"));
}
