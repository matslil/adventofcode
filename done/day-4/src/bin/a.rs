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
}

fn main() {
    setup_tracing();
    println!("{:?}", get_answer("input"));
}

fn get_answer(file: &str) -> usize {
    let mut sum: usize = 0;
    for line in BufReader::new(File::open(file).unwrap()).lines().map(|e| e.unwrap()) {
        sum += parse_line(&line);
    }
    sum
}

fn parse_line(line: &str) -> usize {
    info!("{}", line);
    let split_colon: Vec<&str> = line.split(":").map(|e| e.trim()).collect();
    let card_nr: usize = split_colon[0].split_whitespace().skip(1).next().unwrap().parse().unwrap();
    let split_bar: Vec<&str> = split_colon[1].split("|").map(|e| e.trim()).collect();
    let winning: Vec<usize> = split_bar[0].split_whitespace().map(|e| e.parse::<usize>().unwrap()).collect();
    let have: Vec<usize> = split_bar[1].split_whitespace().map(|e| e.parse::<usize>().unwrap()).collect();

    let mut matches: usize = 0;
    for test in &have {
        if winning.contains(test) {
            if matches == 0 {
                matches = 1;
            } else {
                matches *= 2;
            }
        }
    }
    info!("Card {}: Nr winning numbers: {}", card_nr, matches);
    matches
}

#[test]
fn test() {
    setup_tracing();
    assert_eq!(13, get_answer("test"));
}
