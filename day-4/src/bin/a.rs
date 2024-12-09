// 2577 too low
// 2606 too high

use tracing_subscriber::{filter, prelude::*};
use std::{fs::File, sync::Arc};
use tracing::{info, debug, warn};
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

    debug!("Debug message example");
    info!("Info message example");
    warn!("Warning message example");
}

fn main() {
    setup_tracing();
    info!("{:?}", get_answer("input"));
}

fn is_match(slice: &[char], start: usize) -> bool
{
    let match_against = ['X', 'M', 'A', 'S'];
    for index in match_against.into_iter().enumerate() {
        if slice[start + index.0] != index.1 {
            return false;
        }
    }
    true
}

fn nr_matches(slice: &[char]) -> usize {
    let mut result = 0usize;
    if slice.len() < 4 {
        return 0;
    }
    for start in 0..slice.len()-3 {
        if is_match(slice, start) {
            result += 1;
        }
    }
    result
}

fn get_answer(file: &str) -> usize {
    let input: Vec<Vec<char>> = BufReader::new(File::open(file).unwrap())
        .lines()
        .map(|line| {
            match line {
                Ok(line_content) => line_content.chars().collect::<Vec<char>>(), // Collect to avoid borrowing
                Err(_) => Vec::new(), // Handle potential errors gracefully        .peekable();
            }
        })
        .collect::<Vec<_>>();

    debug!("Dimension: {} x {}", input.len(), input[0].len());
    let mut all: Vec<Vec<char>> = Vec::new();


    let mut pos: Vec<Vec<(usize, usize)>> = Vec::new();

    // All rows
    for x in 0..input.len() {
        let _span = tracing::span!(tracing::Level::DEBUG, "Row", "{}", x).entered();
        let mut current: Vec<char> = Vec::new();
        let mut pos_add: Vec<(usize, usize)> = Vec::new();
        for y in 0..input[0].len() {
            current.push(input[x][y]);
            pos_add.push((x, y));
        }
        pos.push(pos_add.clone().into_iter().rev().collect());
        debug!("{}: {:?}", pos.len() - 1, pos[pos.len()-1]);
        pos.push(pos_add);
        debug!("{}: {:?}", pos.len() - 1, pos[pos.len()-1]);
        all.push(current.iter().copied().rev().collect());
        all.push(current);
    }

    // All columns
    for y in 0..input[0].len() {
        let _span = tracing::span!(tracing::Level::DEBUG, "Column", "{}", y).entered();
        let mut current: Vec<char> = Vec::new();
        let mut pos_add: Vec<(usize, usize)> = Vec::new();
        for x in 0..input.len() {
            current.push(input[x][y]);
            pos_add.push((x, y));
        }
        pos.push(pos_add.clone().into_iter().rev().collect());
        debug!("{}: {:?}", pos.len() - 1, pos[pos.len()-1]);
        pos.push(pos_add);
        debug!("{}: {:?}", pos.len() - 1, pos[pos.len()-1]);
        all.push(current.iter().copied().rev().collect());
        all.push(current);
    }

    let xsize = input.len();
    let ysize = input[0].len();

    // Diagonal 1
    for start_x in 0..xsize {
        let _span = tracing::span!(tracing::Level::DEBUG, "Diagonal 1", "start_x: {}", start_x).entered();
        let mut current: Vec<char> = Vec::new();
        let mut pos_add: Vec<(usize, usize)> = Vec::new();
        for y in 0..ysize {
            let x = start_x + y;
            if x >= xsize {
                break;
            }
            debug!("({},{})", x, y);
            current.push(input[x][y]);
            pos_add.push((x, y));
        }
        if current.len() >= 4 {
            pos.push(pos_add.clone().into_iter().rev().collect());
            debug!("{}: {:?}", pos.len() - 1, pos[pos.len()-1]);
            pos.push(pos_add);
            debug!("{}: {:?}", pos.len() - 1, pos[pos.len()-1]);
            all.push(current.iter().copied().rev().collect());
            all.push(current);
        }
    }

    for start_y in 1..ysize {
        let _span = tracing::span!(tracing::Level::DEBUG, "Diagonal 1", "start_y: {}", start_y).entered();
        let mut current: Vec<char> = Vec::new();
        let mut pos_add: Vec<(usize, usize)> = Vec::new();
        for x in 0..xsize {
            let y = start_y + x;
            if y >= xsize {
                break;
            }
            debug!("({},{})", x, y);
            current.push(input[x][y]);
            pos_add.push((x, y));
        }
        if current.len() >= 4 {
            pos.push(pos_add.clone().into_iter().rev().collect());
            debug!("{}: {:?}", pos.len() - 1, pos[pos.len()-1]);
            pos.push(pos_add);
            debug!("{}: {:?}", pos.len() - 1, pos[pos.len()-1]);
            all.push(current.iter().copied().rev().collect());
            all.push(current);
        }
    }

    // Diagonal 2

    for start_x in 0..xsize {
        let _span = tracing::span!(tracing::Level::DEBUG, "Diagonal 2", "start_x: {}", start_x).entered();
        let mut current: Vec<char> = Vec::new();
        let mut pos_add: Vec<(usize, usize)> = Vec::new();
        for y in (0..ysize).rev() {
            let x = start_x + (xsize - y - 1);
            if x >= xsize {
                break;
            }
            debug!("({},{})", x, y);
            current.push(input[x][y]);
            pos_add.push((x, y));
        }
        if current.len() >= 4 {
            pos.push(pos_add.clone().into_iter().rev().collect());
            debug!("{}: {:?}", pos.len() - 1, pos[pos.len()-1]);
            pos.push(pos_add);
            debug!("{}: {:?}", pos.len() - 1, pos[pos.len()-1]);
            all.push(current.iter().copied().rev().collect());
            all.push(current);
        }
    }

    for start_y in (0..ysize-1).rev() {
        let _span = tracing::span!(tracing::Level::DEBUG, "Diagonal 2", "start_y: {}", start_y).entered();
        let mut current: Vec<char> = Vec::new();
        let mut pos_add: Vec<(usize, usize)> = Vec::new();
        for x in 0..xsize {
            if x > start_y {
                debug!("Stopped");
                break;
            }
            let y = start_y - x;
            debug!("({},{})", x, y);
            current.push(input[x][y]);
            pos_add.push((x, y));
        }
        if current.len() >= 4 {
            pos.push(pos_add.clone().into_iter().rev().collect());
            debug!("{}: {:?}", pos.len() - 1, pos[pos.len()-1]);
            pos.push(pos_add);
            debug!("{}: {:?}", pos.len() - 1, pos[pos.len()-1]);
            all.push(current.iter().copied().rev().collect());
            all.push(current);
        }
    }

    let mut matches = 0usize;

    if all.len() != pos.len() {
        debug!("What???");
    }

    for idx in 0..pos.len() {
        debug!("{:?}", pos[idx]);
        for cmp_idx in 0..pos.len() {
            if idx == cmp_idx {
                continue;
            }
            if pos[idx] == pos[cmp_idx] {
                debug!("---- Duplicate detected: idx: {}, cmpk_idx: {}, {:?}", idx, cmp_idx, pos[idx]);
            }
        }
    }

    for trial in all {
        let trial_matches = nr_matches(&trial);
        debug!("{:?}: {} matches", trial, trial_matches);
        matches += trial_matches;
    }

//    assert!(matches > 2577 && matches < 2606);

    return matches;
}

#[test]
fn test() {
    setup_tracing();
    assert_eq!(18, get_answer("test.a"));
}
