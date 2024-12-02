// use flexi_logger;
// use log::{info, warn};

use tracing_subscriber::{filter, prelude::*};
use std::{fs::File, sync::Arc};
use tracing::{info, debug};
use std::io::{BufRead, BufReader};
use itertools::Itertools;
use itertools::FoldWhile::{Continue, Done};

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
        if report_valid(&report) {
            nr_safe_reports += 1;
        } else {
            for remove_idx in 0..report.len() {
                let mut report_modified = report.clone();
                report_modified.remove(remove_idx);
                if report_valid(&report_modified) {
                    nr_safe_reports += 1;
                    break;
                }
            }
        }
    }

    return nr_safe_reports;
}

fn report_valid(report: &Vec<isize>) -> bool
{
    let _tracing_span = tracing::span!(tracing::Level::DEBUG, "report_valid", "report: {:?}", report).entered();
    let mut ascending: Option<bool> = None;

    let result = report.windows(2)
        .fold_while(true, | _, value | {
            debug!("({}, {})", value[0], value[1]);
            if ! (1..=3).contains(&value[0].abs_diff(value[1]))
            {
                debug!("Diff too high");
                Done(false)
            } else {
                if let Some(asc) = ascending {
                    if (value[0] > value[1]) == asc {
                        Continue(true)
                    } else {
                        debug!("Ascending violation");
                        Done(false)
                    }
                } else {
                    ascending = Some(value[0] > value[1]);
                    Continue(true)
                }
            }
        })
    .into_inner();

    debug!("report_valid() -> {}", result);
    result
}

#[test]
fn test() {
    setup_tracing();
    assert_eq!(4, get_answer("test.a"));
}
