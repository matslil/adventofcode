use tracing::{self, info};
use tracing_subscriber::{filter, prelude::*};
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::sync::Arc;

fn setup_tracing() {
    let stdout_log = tracing_subscriber::fmt::layer()
        .pretty();

    // A layer that logs events to a file.
    let file = File::create("debug.log");
    let file = match file  {Ok(file) => file,Err(error) => panic!("Error: {:?}",error),};
    let debug_log = tracing_subscriber::fmt::layer()
        .with_writer(Arc::new(file))
        .with_ansi(false);

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
}

fn main() {
    setup_tracing();
    println!("{:?}", get_answer("input"));
}

fn all_zeroes(input: &Vec<isize>) -> bool {
    for item in input {
        if *item != 0 {
            return false;
        }
    }
    true
}

fn calculate(input: Vec<isize>) -> isize {
    if all_zeroes(&input) {
        0
    } else {
        input.last().unwrap() + calculate(input.windows(2)
            .map(|e| e[1] - e[0])
            .collect::<Vec<isize>>())
    }
}

fn parse_line(line: &str) -> isize {
    calculate(line
        .split_whitespace()
        .map(|e| e.parse::<isize>().unwrap())
        .rev()
        .collect()
    )
}

fn get_answer(file: &str) -> isize {
    let iter = BufReader::new(File::open(file).unwrap()).lines().map(|e| e.unwrap());

    iter.map(|e| parse_line(&e)).sum()
}

#[test]
fn test() {
    setup_tracing();
    assert_eq!(2, get_answer("test"));
}
