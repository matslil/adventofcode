// use flexi_logger;
// use log::{info, warn};

use tracing_subscriber::{filter, prelude::*};
use std::{fs::File, sync::Arc};
use tracing::info;
use std::io::{BufRead, BufReader};
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
    info!("{:?}", get_answer("input"));
}

fn get_answer(file: &str) -> isize {
    let lists: (Vec<isize>, Vec<isize>) = BufReader::new(File::open(file).unwrap())
        .lines()
        .map(|line| {
            line.unwrap().split_whitespace().map(|e| e.parse::<isize>().unwrap()).next_tuple::<(isize, isize)>().unwrap()
        })
        .unzip();

    let result = lists.0.into_iter()
        .fold(0, | acc, val |
            acc + val * lists.1.clone().into_iter()
            .fold(0, | nr_matches, val2 | nr_matches + if val == val2 { 1 } else { 0 })
        );

    return result;
}

#[test]
fn test() {
    setup_tracing();
    assert_eq!(31, get_answer("test.a"));
}
