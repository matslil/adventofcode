use tracing_subscriber::{filter, prelude::*};
use std::{fs::File, sync::Arc};
use tracing::{info, debug, warn};
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::cmp::min;
use tracing::instrument;

fn setup_tracing() {
    let stdout_log = tracing_subscriber::fmt::layer();

    let file = File::create("debug.log");
    let file = match file  {Ok(file) => file,Err(error) => panic!("Error: {:?}",error),};
    let debug_log = tracing_subscriber::fmt::layer()
        .with_writer(Arc::new(file));

    tracing_subscriber::registry()
        .with(
            stdout_log
                .with_filter(filter::LevelFilter::INFO)
                .and_then(debug_log)
        )
        .init();
}

fn main() {
    setup_tracing();
    info!("{:?}", get_answer("input"));
}

fn can_display(towels: &HashMap<char, Vec<Vec<char>>>, display: &Vec<char>, idx: usize) -> usize {
    let mut result = 0usize;
    if idx >= display.len() {
//        info!("Match");
        return 1;
    }
    let chars_left = display.len() - idx;
    if let Some(towel_list) = towels.get(&display[idx]) {
        for towel in towel_list {
            let _span = tracing::span!(tracing::Level::INFO, "", "{:?}", towel).entered();
            if towel.len() > chars_left {
                continue;
            }
            let mut matched = true;
            for cmp_idx in 0..min(chars_left, towel.len()) {
                if towel[cmp_idx] != display[idx + cmp_idx] {
                    matched = false;
                    break;
                }
            }
            if !matched {
                continue;
            }
            result += can_display(towels, display, idx + towel.len());
        }
    }
    result
}

fn fix_rust_fucking_bug(gr_display: &Vec<char>) -> String {
    let mut result = String::new();
    for ch in gr_display {
        result.push(*ch);
    }
    result
}

fn get_answer(file: &str) -> usize {
    let input: Vec<Vec<String>> = BufReader::new(File::open(file).unwrap()).lines()
        .filter_map(Result::ok)
        .fold(vec![Vec::new()], |mut acc, line| {
            if line.is_empty() {
                acc.push(Vec::new());
            } else {
                if let Some(last) = acc.last_mut() {
                    last.push(line);
                }
            }
            acc
        });
    let towels: Vec<Vec<char>> = input[0][0].split(',')
        .map(|towel| towel.trim().chars().collect())
        .collect();

    let mut towel_map: HashMap<char, Vec<Vec<char>>> = HashMap::new();
    for towel in towels {
        let mut handled_chars: Vec<char> = Vec::new();
        let ch = towel[0];
        if !handled_chars.contains(&ch) {
            if let Some(mapped_towel) = towel_map.get_mut(&ch) {
                mapped_towel.push(towel.clone());
            } else {
                towel_map.insert(ch, vec![towel.clone()]);
            }
            handled_chars.push(ch);
        }
    }

    let displays: Vec<Vec<char>> = input[1].iter()
        .map(|display| display.trim().chars().collect())
        .collect();

    let mut result = 0usize;
    for gr_display in displays {
        info!("{:?}", gr_display);
        let _span = tracing::span!(tracing::Level::INFO, "Display", "{:?}", fix_rust_fucking_bug(&gr_display)).entered();
        result += can_display(&towel_map, &gr_display, 0);
    }
    return result;
}

#[test]
fn test() {
    setup_tracing();
    assert_eq!(16, get_answer("test.a"));
}
