use tracing::{self, info};
use tracing_subscriber::{filter, prelude::*};
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::sync::Arc;
use std::collections::VecDeque;

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
    println!("{:?}", get_answer("input"));
}

fn pattern_string(pattern: &Vec<bool>) -> String {
    let mut string = String::new();
    for joker in pattern {
        string.push(match joker {
            true => '?',
            false => '#',
        });
    }
    string
}

fn translate_pattern(pattern: Vec<bool>, values: VecDeque<usize>) -> (usize, Vec<bool>, VecDeque<usize>) {
    let mut count = 0usize;

    info!("translate_pattern({}, {:?}", pattern_string(&pattern), values);

    if values.len() == 0 {
        info!("translate_pattern() -> 0");
        return (0, pattern, VecDeque::new());
    }

    let value = values.pop_front().unwrap();

    if value < pattern.len() {
        if pattern.into_iter().fold(true, |acc, e| acc && e) {
            // Cannot do this, since there are non-jokers left
            //
            info!("translate_pattern() -> 0");
            return (0, pattern, values);
        }
    }

    if value == pattern.len() {
        if pattern.into_iter().fold(true, |acc, e| acc && e) {
            // Cannot do this, since there are non-jokers left
            return (0, pattern, values);
        }
        info!("translate_pattern() -> 1");
        return (1, Vec::new(), values);
    }

    for (nr, trial) in pattern.windows(value).enumerate() {
        let pattern_iter = pattern[(nr+value)..].to_vec().into_iter();
        let (pattern_count, remaining_pattern, remaining_values) = match pattern_iter.next() {
            None => continue,
            Some(c) => if !c {
                continue
            } else {
                translate_pattern(pattern_iter.collect(), values)
            },
        };
        if ! pattern.into_iter().fold(true, |acc, e| acc && e) && values.len() == 0 {
            count += pattern_count;
        }
    }

    info!("translate_pattern() -> {}", count);
    (count, pattern, values)
}

fn translate(patterns: Vec<Vec<bool>>, values: VecDeque<usize>) -> usize {
    let mut count = 0usize;
    for pattern in patterns {
        let (pattern_count, remaining_pattern, remaining_values) = translate_pattern(pattern, values);
        if ! pattern.into_iter().fold(true, |acc, e| acc && e) && values.len() == 0 {
            count += pattern_count;
        }
    }
    count
}
fn get_answer(file: &str) -> usize {
    setup_tracing();
    let mut count = 0usize;

    for line in BufReader::new(File::open(file).unwrap())
        .lines()
        .map(|e| e.unwrap()) {
            count += line
                .split_whitespace()
                .map(|pair| (pair[0], pair[1]
                        .into_iter()
                        .split(",")
                        .map(|v| v
                            .parse::<usize>()
                            .collect::<Vec<usize>>())))
                .fold(0, |acc, e| acc + translate(
                        e.0
                        .into_iter()
                        .split(".")
                        .filter(|s| s != "")
                        .map(|s| s
                            .chars()
                            .map(|c| c == '?')
                            .collect::<Vec<bool>>())
                        .collect::<Vec<Vec<bool>>>(),
                        e.1.into()));
        }
    count
}

#[test]
fn test() {
    assert_eq!(21, get_answer("test"));
}
