use tracing_subscriber::{filter, prelude::*};
use std::{fs::File, sync::Arc};
use tracing::{info, debug, warn};
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn setup_tracing() {
    let stdout_log = tracing_subscriber::fmt::layer()
        .pretty();
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

fn shortest_path(from: char, to: char, base_path: &Vec<char>) -> Vec<char> {
    let mut curr = from;
    let mut path = Vec::new();

    if curr == to {
        return base_path.clone().append(&path);
    }

    
}

fn get_answer(file: &str) -> usize {
    let directional_keypad = HashMap::from([
        ('A', vec!['^',  '>']),
        ('^', vec!['A', 'v']),
        ('>', vec!['A', 'v']),
        ('<', vec!['v']),
    ]);

    let numeric_keypad = HashMap::from([
        ('A', vec!['0', '3']),
        ('0', vec!['A', '2']),
        ('1', vec!['2', '4']),
        ('2', vec!['0', '1', '3', '5']),
        ('3', vec!['A', '2', '6']),
        ('4', vec!['1', '5', '7']),
        ('5', vec!['2', '4', '6', '8']),
        ('6', vec!['3', '5', '9']),
        ('7', vec!['4', '8']),
        ('8', vec!['5', '7', '9']),
        ('9', vec!['6', '8']),
    ]);

    let mut input: Vec<Vec<char>> = BufReader::new(File::open(file).unwrap()).lines()
        .filter_map(Result::ok)
        .map(|line| line.chars().collect())
        .collect();
    ;
    return 1;
}

#[test]
fn test() {
    setup_tracing();
    assert_eq!(4, get_answer("test.a"));
}
