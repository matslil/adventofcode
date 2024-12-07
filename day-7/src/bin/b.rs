// use flexi_logger;
// use log::{info, warn};

use tracing_subscriber::{filter, prelude::*};
use std::{fs::File, sync::Arc};
use tracing::{info, debug, warn};
use std::io::{BufRead, BufReader};

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

fn get_answer(file: &str) -> usize {
    let mut input: Vec<Vec<String>> = BufReader::new(File::open(file).unwrap()).lines();
    info!("{:?}: Using as input data", file);
    return 1;
}

#[test]
fn test() {
    setup_tracing();
    assert_eq!(4, get_answer("test.1"));
    assert_eq!(8, get_answer("test.2"));
}
