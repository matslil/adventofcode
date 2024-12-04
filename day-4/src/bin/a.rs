// use flexi_logger;
// use log::{info, warn};

use tracing_subscriber::{filter, prelude::*};
use std::{fs::File, sync::Arc};
use tracing::{info, debug, warn};
use std::io::{BufRead, BufReader};
use std::cmp::{max, min};

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
                // Add an `INFO` filter to the stdout logging layer
                .with_filter(filter::LevelFilter::INFO)
                // Combine the filtered `stdout_log` layer with the
                // `debug_log` layer, producing a new `Layered` layer.
                .and_then(debug_log)
        )
        .init();

    debug!("Debug message example");
    info!("Info message example");
    warn!("Warning message example");
}

fn main() {
    setup_tracing();
    info!("{:?}", get_answer("input"));
}

fn is_match(slice: &[char], start: usize) -> bool
{
    let match_against = ['X', 'M', 'A', 'S'];
    for index in 0..4 {
        if slice[start + index] != match_against[index] {
            return false;
        }
    }
    true
}

fn nr_matches(slice: &[char]) -> usize {
    let mut result = 0usize;
    if slice.len() < 4 {
        return 0;
    }
    for start in 0..slice.len()-3 {
        if is_match(slice, start) {
            result += 1;
        }
    }
    result
}

fn get_answer(file: &str) -> usize {
    let input: Vec<Vec<char>> = BufReader::new(File::open(file).unwrap())
        .lines()
        .map(|line| {
            match line {
                Ok(line_content) => line_content.chars().collect::<Vec<char>>(), // Collect to avoid borrowing
                Err(_) => Vec::new(), // Handle potential errors gracefully        .peekable();
            }
        })
        .collect::<Vec<_>>();

    let mut all: Vec<Vec<char>> = Vec::new();

    let mut current: Vec<char> = Vec::new();

    // All rows
    for x in 0..input.len() {
        let _span = tracing::span!(tracing::Level::DEBUG, "Row", "{}", x).entered();
        for y in 0..input.len() {
            current.push(input[x][y]);
        }
        all.push(current.clone());
        debug!("Forward:  {:?}", all[all.len()-1]);
        all.push(current.iter().copied().rev().collect());
        debug!("Backward: {:?}", all[all.len()-1]);
        current.clear();
    }

    // All columns
    for y in 0..input.len() {
        let _span = tracing::span!(tracing::Level::DEBUG, "Column", "{}", y).entered();
        for x in 0..input.len() {
            current.push(input[x][y]);
        }
        all.push(current.clone());
        debug!("Forward:  {:?}", all[all.len()-1]);
        all.push(current.iter().copied().rev().collect());
        debug!("Backward: {:?}", all[all.len()-1]);
        current.clear();
    }

    // Diagonal 1
    for start_x in 0..input.len() {
        let _span = tracing::span!(tracing::Level::DEBUG, "Diagonal 1 x", "{}", start_x).entered();
        for index in 0..min(input.len(), input[0].len()) {
            if start_x + index >= min(input.len(), input[0].len()) {
                continue;
            }
            let x = start_x + index;
            let y = index;
            debug!("({},{})", x, y);
            current.push(input[x][y]);
        }
        all.push(current.clone());
        debug!("Forward:  {:?}", all[all.len()-1]);
        all.push(current.iter().copied().rev().collect());
        debug!("Backward: {:?}", all[all.len()-1]);
        current.clear();
    }

    for start_y in 0..input[0].len() {
        let _span = tracing::span!(tracing::Level::DEBUG, "Diagonal 1 y", "{}", start_y).entered();
        for index in 0..min(input.len(), input[0].len()) {
            if start_y + index >= min(input.len(), input[0].len()) {
                continue;
            }
            let x = index;
            let y = start_y + index;
            debug!("({},{})", x, y);
            current.push(input[x][y]);
        }
        all.push(current.clone());
        debug!("Forward:  {:?}", all[all.len()-1]);
        all.push(current.iter().copied().rev().collect());
        debug!("Backward: {:?}", all[all.len()-1]);
        current.clear();
    }

    // Diagonal 2

    for start_x in 0..input.len() {
        let _span = tracing::span!(tracing::Level::DEBUG, "Diagonal 2 x", "{}", start_x).entered();
        for index in (0..min(input.len(), input[0].len())).rev() {
            if start_x + index >= input.len() {
                continue;
            }
            let x = start_x + index;
            let y = index;
            debug!("({},{})", x, y);
            current.push(input[x][y]);
        }
        all.push(current.clone());
        debug!("Forward:  {:?}", all[all.len()-1]);
        all.push(current.iter().copied().rev().collect());
        debug!("Backward: {:?}", all[all.len()-1]);
        current.clear();
    }

    for start_y in 0..input[0].len() {
        let _span = tracing::span!(tracing::Level::DEBUG, "Diagonal 2 y", "{}", start_y).entered();
        for index in (0..min(input.len(), input[0].len())).rev() {
            if start_y + index >= input[0].len() {
                continue;
            }
            let x = index;
            let y = start_y + index;
            debug!("({},{})", x, y);
            current.push(input[x][y]);
        }
        all.push(current.clone());
        debug!("Forward:  {:?}", all[all.len()-1]);
        all.push(current.iter().copied().rev().collect());
        debug!("Backward: {:?}", all[all.len()-1]);
        current.clear();
    }

    let mut matches = 0usize;

    for trial in all {
        let trial_matches = nr_matches(&trial);
        debug!("{:?}: {} matches", trial, trial_matches);
        matches += trial_matches;
    }

    return matches;
}

#[test]
fn test() {
    setup_tracing();
    assert_eq!(18, get_answer("test.a"));
}
