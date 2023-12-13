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

fn translate_pattern(level: usize, pattern: Vec<bool>, in_values: VecDeque<usize>) -> usize {
    let mut count = 0usize;
    let mut values = in_values.clone();

    info!("{0:1$}translate_pattern({2}, {3:?}", "", level*4, pattern_string(&pattern), values);

    if values.len() == 0 {
        info!("{0:1$}translate_pattern() -> 0", "", level*4);
        return 0;
    }

    let value = values.pop_front().unwrap();

    if value > pattern.len() {
        info!("{0:1$}translate_pattern() -> 0", "", level*4);
        return 0;
    }

    let mut nr = 0usize;
    for trial in pattern.windows(value) {
        let mut pattern_iter = trial.iter();
        match pattern_iter.next() {
            None => {
                nr += 1;
                continue
            }
            Some(c) => if !c {
                nr += 1;
                continue
            } else {
                if pattern.len() > (nr + value) {
                    return translate_pattern(
                        level + 1,
                        pattern[(nr + value + 1)..].to_vec(),
                        values.clone()
                    )
                }
            },
        };
        nr += 1;
    }

    if pattern[..value].into_iter().fold(false, |acc, &e| acc || e) && values.len() == 0 {
        info!("{0:1$}translate_pattern() -> {2}", "", level*4, count);
        return count;
    }
    info!("{0:1$}translate_pattern() -> 0", "", level*4);
    0
}

fn translate(patterns: Vec<Vec<bool>>, values: VecDeque<usize>) -> usize {
    let mut count = 0usize;
    for pattern in patterns {
        let pattern_count = translate_pattern(0, pattern.clone(), values.clone());
        count += pattern_count;
        info!("translate: +{} ({})", pattern_count, count);
    }
    count
}
fn get_answer(file: &str) -> usize {
    setup_tracing();
    let mut count = 0usize;

    for line in BufReader::new(File::open(file).unwrap())
        .lines()
        .map(|e| e.unwrap()) {
            let parts:Vec<&str> = line.as_str()
                .split_whitespace()
                .collect();
            let patterns: Vec<Vec<bool>> = parts[0]
                .split('.')
                .filter(|&s| s != "")
                .map(|s| s
                    .chars()
                    .map(|c| c == '?')
                    .collect::<Vec<bool>>())
                .collect::<Vec<Vec<bool>>>();
            let values: VecDeque<usize> = parts[1]
                .split(",")
                .map(|v| v
                    .parse::<usize>()
                    .unwrap())
                .collect::<Vec<usize>>().into();
            count += translate(patterns, values);
        }
    count
}

#[test]
fn test() {
    assert_eq!(21, get_answer("test"));
}
