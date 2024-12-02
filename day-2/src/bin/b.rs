// use flexi_logger;
// use log::{info, warn};

use tracing_subscriber::{filter, prelude::*};
use std::{fs::File, sync::Arc};
use tracing::{info, debug};
use std::io::{BufRead, BufReader};
use std::collections::VecDeque;

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

    let mut report_nr: usize = 0;
    for report in reports {
        report_nr += 1;
        debug!("---- Report nr: {} ----", report_nr);
        let violations = count_violations(&report.try_into().unwrap(), None, None, 0);
        if violations <= 1 {
            debug!("Report is safe");
            nr_safe_reports += 1;
        } else {
            debug!("Report is unssafe: Violations: {}", violations);
        }
    }

    return nr_safe_reports;
}

fn count_violations(report_arg: &VecDeque<isize>, ascending_arg: Option<bool>, previous_arg: Option<isize>, violations_arg: usize) -> usize {
    let mut violations = violations_arg;
    let mut ascending = ascending_arg;
    let mut previous = previous_arg;
    let mut report = report_arg.clone();

    while let Some(level) = report.pop_front() {
        if previous == None {
            previous = Some(level);
        } else {
            debug!("({}, {})", previous.unwrap(), level);
            let diff = level.abs_diff(previous.unwrap());
            if diff < 1 || diff > 3 {
                debug!("Diff {}: Not safe", diff);
                violations += 1;
            }
            match ascending {
                None => ascending = Some(level > previous.unwrap()),
                Some(true) => if level < previous.unwrap() {
                    debug!("ascending -> descending: Not safe");
                    violations += 1;
                },
                Some(false) => if level > previous.unwrap() {
                    debug!("descending -> ascending: Not safe");
                    violations += 1;
                },
            }
            if violations > 1 {
                break;
            }
            if violations == 0 {
                previous = Some(level);
            } else {
                return std::cmp::min(
                    count_violations(&report, ascending, previous, violations),
                    if let Some(_) = report.pop_front() { count_violations(&report, ascending, previous, violations) } else { 0 }
                );
            }
        }
    }
    debug!("Returned violations: {}", violations);
    return violations;
}

#[test]
fn test() {
    setup_tracing();
    assert_eq!(4, get_answer("test.a"));
}
