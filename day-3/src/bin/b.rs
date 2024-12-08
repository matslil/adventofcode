// 125313914 too high

use tracing_subscriber::{filter, prelude::*};
use std::{fs::File, sync::Arc};
use tracing::{info, debug, warn};
// use itertools::Itertools;
// use itertools::FoldWhile::{Continue, Done};
use std::io::{BufRead, BufReader};
use std::iter::Peekable;

fn setup_tracing() {
    let stdout_log = tracing_subscriber::fmt::layer();

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

fn get_int(iter: &mut Peekable<impl Iterator<Item=char>>) -> Option<u32> {
    let _span0 = tracing::span!(tracing::Level::INFO, "get_int").entered();

    let mut result: Option<u32> = None;
    while let Some(chr) = iter.peek() {
        debug!("ch: {}", chr);
        if let Some(value) = chr.to_digit(10) {
            iter.next();
            if result == None {
                result = Some(value);
            } else {
                result = Some(result.unwrap() * 10 + value);
            }
        } else {
            break;
        }
    }
    if let Some(value) = result {
        debug!("{:?}", value);
    }
    result
}

fn match_char(iter: &mut Peekable<impl Iterator<Item=char>>, ch: char) -> bool
{
    let _span0 = tracing::span!(tracing::Level::INFO, "match_char", "{}", ch).entered();

    if let Some(peek_ch) = iter.peek() {
        debug!("ch: {}", ch);
        if *peek_ch == ch {
            iter.next();
            debug!("Found");
            return true;
        }
    }
    return false;
}

/// # Examples
///
/// ```
/// let mut input = vec!['d', 'd', 'o', '(', ')', '\n', 'd', 'o'].peekable();
/// let words = vec![['d', 'o', '(', ')']];
///
/// assert!(1 == find_any(input, &words));
/// assert!(None == find_any(input, &words));
/// ```
fn find_any(iter: &mut Peekable<impl Iterator<Item=char>>, words: &Vec<Vec<char>>) -> Option<usize> {
    let mut nr_matches = 0usize;
    let mut failed_words: Vec<bool> = Vec::new();

    let _span0 = tracing::span!(tracing::Level::INFO, "find_any").entered();

    for _ in 0..words.len() {
        failed_words.push(false);
    }

    loop {
        if let Some(ch) = iter.peek() {
            debug!("ch: {}", ch);
            let mut any_match = false;
            for idx in 0..words.len() {
                if !failed_words[idx] {
                    if *ch == words[idx][nr_matches] {
                        any_match = true;
                        if nr_matches == words[idx].len()-1 {
                            debug!("{}", idx);
                            iter.next();
                            return Some(idx);
                        }
                    } else {
                        failed_words[idx] = true;
                    }
                }
            }
            iter.next();
            if !any_match {
                nr_matches = 0;
                for entry in &mut failed_words {
                    *entry = false;
                }
            } else {
                nr_matches += 1;
            }
        } else {
            return None;
        }
    }
}

fn execute_next(iter: &mut Peekable<impl Iterator<Item=char>>, mul_enabled: &mut bool) -> Option<u32> {
    let _span0 = tracing::span!(tracing::Level::INFO, "execute_next", mul_enabled).entered();
    if let Some(instruction) = find_any(iter, &vec![vec!['m', 'u', 'l', '('], vec!['d', 'o', '(', ')'], vec!['d', 'o', 'n', '\'', 't', '(', ')']]) {
        match instruction {
            0 => {
                let lhs: u32;
                let rhs: u32;
                if let Some(value) = get_int(iter) {
                    lhs = value;
                } else {
                    return Some(0);
                }
                if !match_char(iter, ',') {
                    return Some(0);
                }
                if let Some(value) = get_int(iter) {
                    rhs = value;
                } else {
                    return Some(0);
                }
                if !match_char(iter, ')') {
                    return Some(0);
                }
                if *mul_enabled {
                    info!("mul({},{})", lhs, rhs);
                    return Some(lhs * rhs);
                } else {
                    info!("skipping mul({},{})", lhs, rhs);
                    return Some(0);
                }
            },
            1 => {
                info!("do()");
                *mul_enabled = true
            },
            2 => {
                info!("don't()");
                *mul_enabled = false
            },
            _ => {
                ()
            },
        }
        return Some(0);
    }
    return None;
}

fn get_answer(file: &str) -> u32 {
    let mut mul_enabled = true;
    let mut sum: u32 = 0;
    let mut input = BufReader::new(File::open(file).unwrap())
        .lines()
        .filter_map(Result::ok)
        .flat_map(|line| line.chars().collect::<Vec<_>>())
        .peekable();
    while let Some(value) = execute_next(&mut input, &mut mul_enabled) {
        sum += value;
    }

    sum
}

#[test]
fn test() {
    setup_tracing();
    assert_eq!(48, get_answer("test.b"));
}
