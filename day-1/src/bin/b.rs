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
    println!("{}", get_answer("input"));
}

fn get_answer(file: &str) -> usize {
    let mut sum: usize = 0;
    for line in BufReader::new(File::open(file).unwrap()).lines().map(|x| x.unwrap()) {
        sum += parse_line(&line);
    }
    sum
}

trait StringUtils {
    fn substring(&self, start: usize, len: usize) -> &str;
}

impl StringUtils for str {
    fn substring(&self, start: usize, len: usize) -> &str {
        let mut char_pos = 0;
        let mut byte_start = 0;
        let mut it = self.chars();
        loop {
            if char_pos == start { break; }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_start += c.len_utf8();
            }
            else { break; }
        }
        char_pos = 0;
        let mut byte_end = byte_start;
        loop {
            if char_pos == len { break; }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_end += c.len_utf8();
            }
            else { break; }
        }
        &self[byte_start..byte_end]
    }
}

fn string_to_num(line: &str) -> Option<usize> {
    let numbers:HashMap<&str, usize> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    info!("string_to_num({})", line);
    for (trial, number) in &numbers {
        if line.starts_with(trial) {
            return Some(*number);
        }
    }
    None
}

fn parse_line(line: &String) -> usize {
    info!("{}", line);
    let mut first:Option<usize> = None;
    let mut last:Option<usize> = None;
    let mut string = line.chars();

    for (idx, ch) in line.chars().enumerate() {
        if let Some(num) = ch.to_digit(10) {
            if first == None {
                first = Some(num.try_into().unwrap());
            }
            last = Some(num.try_into().unwrap());
        } else if let Some(num) = string_to_num(
            &line[idx..]) {
            if first == None {
                first = Some(num.try_into().unwrap());
            }
            last = Some(num.try_into().unwrap());
        }
    }
    info!("first: {:?}, last: {:?}", first, last);
    (first.unwrap().to_string() + &last.unwrap().to_string()).parse::<usize>().unwrap()
}

#[test]
fn test() {
    setup_tracing();
    assert_eq!(get_answer("test.b"), 281);
}
