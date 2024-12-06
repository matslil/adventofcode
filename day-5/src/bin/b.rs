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

fn item_is_valid(item: u32, rest: &[u32], rules: &Vec<(u32, u32)>) -> bool {
    for check in rest {
        for rule in rules {
            if item == rule.1 && *check == rule.0 {
                return false;
            }
        }
    }
    true
}

fn list_is_valid(list: &Vec<u32>, rules: &Vec<(u32, u32)>) -> bool {
    for index in 0..list.len()-1 {
        if !item_is_valid(list[index], &list[index+1..], rules) {
            return false;
        }
    }

    true
}

fn swap_if_needed(items: &mut [u32], rules: &Vec<(u32, u32)>) {
    for rule in rules {
        if items[0] == rule.1 && items[1] == rule.0 {
            let temp = items[0];
            items[0] = items[1];
            items[1] = temp;
            break;
        }
    }
}

fn bubble_sort_list(list: &mut Vec<u32>, rules: &Vec<(u32, u32)>) {
    for _tries in 0..list.len()-1 {
        for index in 0..list.len()-1 {
            swap_if_needed(&mut list[index..=index+1], rules);
        }
    }
}

fn get_answer(file: &str) -> u32 {
    let mut input: Vec<Vec<String>> = BufReader::new(File::open(file).unwrap()).lines()
        .filter_map(Result::ok)
        .fold(vec![Vec::new()], |mut acc, line| {
            if line.is_empty() {
                acc.push(Vec::new()); // Start a new group on an empty line
            } else {
                if let Some(last) = acc.last_mut() {
                    last.push(line); // Add the line to the last group
                }
            }
            acc
        })
        .into_iter()
        .filter(|group| !group.is_empty()) // Remove empty groups if needed
        .collect();

    let lists: Vec<Vec<u32>> = input.pop().unwrap().into_iter()
        .map(|l| l.split(',').
            map(|str| str.parse::<u32>().unwrap())
            .collect::<Vec<_>>())
        .collect();

    let rules: Vec<(u32, u32)> = input.pop().unwrap().into_iter()
        .take_while(|l| l.len() > 0)
        .map(|l| {
            let mut pair_iter = l.split('|').map(|str| str.parse::<u32>().unwrap());
            (pair_iter.next().unwrap(), pair_iter.next().unwrap())
        })
        .collect();

    let mut result: u32 = 0;
    for mut list in lists {
        if !list_is_valid(&list, &rules) {
            bubble_sort_list(&mut list, &rules);
            let middle = list[(list.len()-1) / 2];
            debug!("Valid: {:?}: {}", list, middle);
            result += middle;
        }
    }

    info!("{:?}: Using as input data", file);
    return result;
}

#[test]
fn test() {
    setup_tracing();
    assert_eq!(123, get_answer("test.a"));
}
