// 168465905 too low
//
use tracing_subscriber::{filter, prelude::*};
use std::{fs::File, sync::Arc};
use tracing::{info, debug, warn};
// use itertools::Itertools;
// use itertools::FoldWhile::{Continue, Done};
use std::io::{BufRead, BufReader};
use std::iter::Peekable;

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

fn get_int(iter: &mut Peekable<impl Iterator<Item=char>>) -> Option<u32> {
    let mut result: Option<u32> = None;
    while let Some(chr) = iter.peek() {
        debug!("get_int(): Peek: {}", chr);
        if let Some(value) = chr.to_digit(10) {
            debug!("Was int");
            iter.next();
            if result == None {
                result = Some(value);
            } else {
                result = Some(result.unwrap() * 10 + value);
            }
        } else {
            debug!("No int");
            break;
        }
    }
    debug!("Got int: {:?}", result);
    result
}

fn match_str(iter: &mut Peekable<impl Iterator<Item=char>>, s: &str) -> bool
{
    let mut iter_match = s.chars();
    while let Some(ch) = iter.peek() {
        debug!("match_str(): Checking {}", ch);
        if let Some(ch_match) = iter_match.next() {
            if *ch != ch_match {
                return false;
            }
        } else {
            debug!("Found string");
            return true;
        }
        iter.next();
    }
    if iter_match.next() == None {
        true
    } else {
        false
    }
}

fn find_char(iter: &mut Peekable<impl Iterator<Item=char>>, c: char) -> bool {
    while let Some(ch) = iter.peek() {
        if *ch == c {
            return true;
        }
        iter.next();
    }
    false
}

fn get_mult(iter: &mut Peekable<impl Iterator<Item=char>>) -> Option<u32> {
    let lhs: u32;
    let rhs: u32;

    debug!("get_mult() called");
    if !find_char(iter, 'm') {
        return None;
    }
    debug!("Found first char");
    if !match_str(iter, "mul(") {
        return Some(0);
    }
    if let Some(value) = get_int(iter) {
        lhs = value;
    } else {
        return Some(0);
    }
    if !match_str(iter, ",") {
        return Some(0);
    }
    if let Some(value) = get_int(iter) {
        rhs = value;
    } else {
        return Some(0);
    }
    if !match_str(iter, ")") {
        return Some(0);
    }
    debug!("{} * {} = {}", lhs, rhs, lhs*rhs);
    return Some(lhs * rhs);
}

fn get_answer(file: &str) -> u32 {
    let mut sum: u32 = 0;
    let mut input = BufReader::new(File::open(file).unwrap())
        .lines()
        .flat_map(|line| {
            match line {
                Ok(line_content) => line_content.chars().collect::<Vec<_>>(), // Collect to avoid borrowing
                Err(_) => Vec::new(), // Handle potential errors gracefully        .peekable();
            }
        }).peekable();
    while let Some(value) = get_mult(&mut input) {
        sum += value;
    }

    sum
}

#[test]
fn test() {
    setup_tracing();
    assert_eq!(161, get_answer("test.a"));
}
