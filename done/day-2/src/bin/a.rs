// use flexi_logger;
// use log::{info, warn};

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

fn get_answer(file: &str) -> usize {
    let reports: Vec<Vec<isize>> = BufReader::new(File::open(file).unwrap())
    .lines()
    .map(|line| {
        line.unwrap().split_whitespace().map(|e| e.parse::<isize>().unwrap()).collect::<Vec<_>>()
    })
    .collect();

    let mut nr_safe_reports: usize = 0;

    for report in reports {
        let mut ascending: Option<bool> = None;
        let mut previous: Option<isize> = None;
        let mut safe: bool = true;
        let mut report_nr: usize = 0;
        for level in report {
            report_nr += 1;
            debug!("---- Report nr: {} ----", report_nr);
            if previous == None {
                previous = Some(level);
            } else {
                debug!("({}, {})", previous.unwrap(), level);
                let diff = level.abs_diff(previous.unwrap());
                if diff < 1 || diff > 3 {
                    debug!("Diff {}: Not safe", diff);
                    safe = false;
                }
                match ascending {
                    None => ascending = Some(level > previous.unwrap()),
                    Some(true) => if level < previous.unwrap() {
                        debug!("ascending -> descending: Not safe");
                        safe = false;
                    },
                    Some(false) => if level > previous.unwrap() {
                        debug!("descending -> ascending: Not safe");
                        safe = false;
                    },
                }
                previous = Some(level);
                if !safe { break; };
            }
        }
        if safe {
            nr_safe_reports += 1;
        }
    }

    return nr_safe_reports;
}

#[test]
fn test() {
    setup_tracing();
    assert_eq!(2, get_answer("test.a"));
}
