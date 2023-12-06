use tracing::{self, info};
use tracing_subscriber::{filter, prelude::*};
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::sync::Arc;
use std::iter::zip;

fn parse_numbers(line: &str) -> Vec<usize> {
    let parts: Vec<&str> = line.split(":").map(|e| e.trim()).collect();
    let mut number_str = String::new();
    for part_str in parts[1].split_whitespace() {
        number_str += part_str;
    }
    info!("number_str: {}", number_str);
    let mut number: Vec<usize> = Vec::new();
    number.push(number_str.parse::<usize>().unwrap());
    number
}

fn distance(race_time: usize, press_time: usize) -> usize {
    // Speed is the same as press_time
    let race_time = race_time - press_time;
    race_time * press_time
}

// Do a binary search to find a possible win given race time and winning distance,
// returning the press time needed to win.
fn binary_search(race_time: usize, start_press_time: usize, end_press_time: usize, win_distance: usize, search_for_win: bool) -> Option<usize> {
//    info!("binary_search(race_time: {}, start_press_time: {}, end_press_time: {}, win_distance: {}, search_for_win: {})", race_time, start_press_time, end_press_time, win_distance, search_for_win);
    if start_press_time >= end_press_time {
        return None;
    }
    let press_time = ((end_press_time - start_press_time) / 2) + start_press_time;
    if (distance(race_time, press_time) > win_distance) == search_for_win {
        return Some(press_time);
    }
    if let Some(t) = binary_search(race_time, start_press_time, press_time.saturating_sub(1), win_distance, search_for_win) {
        return Some(t);
    }
    binary_search(race_time, press_time + 1, end_press_time, win_distance, search_for_win)
}

fn wins(race_time: usize, win_distance: usize) -> usize {
    let win = binary_search(race_time, 0, race_time, win_distance, true).unwrap();
    let mut first_lost = binary_search(race_time, 0, win, win_distance, false).unwrap();
    loop {
        info!("first_lost: {}", first_lost);
        if let Some(lower_lost) = binary_search(race_time, first_lost, win, win_distance, false) {
            if first_lost != lower_lost {
                first_lost = lower_lost;
                continue;
            }
        }
        let mut last_lost = binary_search(race_time, win, race_time, win_distance, false).unwrap();
        loop {
            info!("last_lost: {}", last_lost);
            if let Some(higher_lost) = binary_search(race_time, win, last_lost, win_distance, false) {
                if last_lost != higher_lost {
                    last_lost = higher_lost;
                    continue;
                }
            }
            return last_lost - first_lost - 1;
        }
    }
}

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
    println!("{:?}", get_answer("input"));
}

fn get_answer(file: &str) -> usize {
    let mut sum: usize = 1;
    let mut iter = BufReader::new(File::open(file).unwrap()).lines().map(|e| e.unwrap());

    let times = parse_numbers(&iter.next().unwrap());
    let distances = parse_numbers(&iter.next().unwrap());

    for time_dist in zip(times, distances) {
        info!("Time: {}, win distance: {}", time_dist.0, time_dist.1);
        let nr_wins = wins(time_dist.0, time_dist.1);
        info!("time_dist: {:?}, nr_wins: {}", time_dist, nr_wins);
        sum *= nr_wins;
    }
    sum
}

#[test]
fn test() {
    setup_tracing();
    assert_eq!(71503, get_answer("test"));
}
