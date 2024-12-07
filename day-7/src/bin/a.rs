use tracing_subscriber::{filter, prelude::*};
use std::{fs::File, sync::Arc};
use tracing::{info, debug};
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
}

fn main() {
    setup_tracing();
    info!("{:?}", get_answer("input"));
}

fn calibration_result(expected: usize, acc: usize, rest: &[usize]) -> usize {
    let _span = tracing::span!(tracing::Level::DEBUG, "calibration_result", "{}, {}, {:?}", expected, acc, rest).entered();

    if rest.len() == 0 {
        if acc == expected {
//            debug!("Match!");
            return expected;
        } else {
//            debug!("No match");
            return 0;
        }
    }

    if acc > expected {
        return 0;
    }

    let mult = calibration_result(expected, acc * rest[0], &rest[1..]);
    if mult == expected {
        return expected;
    }
    calibration_result(expected, acc + rest[0], &rest[1..])
}

fn get_answer(file: &str) -> usize {
    let input: Vec<Vec<usize>> = BufReader::new(File::open(file).unwrap()).lines()
        .filter_map(Result::ok)
        .map(|line| line
            .split(&[':', ' '])
            .filter(|str| !str.is_empty())
            .map(|val_str| { debug!("{}", val_str); val_str.parse::<usize>().unwrap() }).collect()
        )
        .collect();

    let mut total_calibration_result = 0usize;
    for test in input {
        total_calibration_result += calibration_result(test[0], test[1], &test[2..]);
    }

    info!("{:?}: Using as input data", file);
    total_calibration_result
}

#[test]
fn test() {
    setup_tracing();
    assert_eq!(3749, get_answer("test.a"));
}
