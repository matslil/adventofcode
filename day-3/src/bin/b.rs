use tracing::{self, info};
use tracing_subscriber::{filter, prelude::*};
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::sync::Arc;
use std::collections::HashMap;

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
    let mut games: Vec<usize> = Vec::new();
    for line in BufReader::new(File::open(file).unwrap()).lines().map(|x| x.unwrap()) {
        games.push(parse_line(&line));
    }
    games.into_iter().sum()
}

fn parse_line(line: &String) -> usize {
    info!("{}", line);
}

#[test]
fn test() {
    setup_tracing();
    assert_eq!(2286, get_answer("test.a"));
}
