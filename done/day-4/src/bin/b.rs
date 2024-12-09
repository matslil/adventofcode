// 1938 too low

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

// (0,0) (1,0) (2,0)
// (0,1) (1,1) (2,1)
// (0,2) (1,2) (2,2)
fn cross_found(g: &Vec<Vec<char>>) -> usize
{
    let mut matches = 0usize;
    if g[1][1] != 'A' {
        return 0;
    }
    if ((g[0][0] == 'M' && g[2][2] == 'S') ||
        (g[0][0] == 'S' && g[2][2] == 'M')) &&
        ((g[2][0] == 'M' && g[0][2] == 'S') ||
         (g[2][0] == 'S' && g[0][2] == 'M')) {
            matches += 1;
    }

    matches
}

fn check_slice(grid: &Vec<Vec<char>>, pos: (usize, usize)) -> usize {
    let mut grid_slice: Vec<Vec<char>> = Vec::new();

    for x in pos.0..pos.0+3 {
        let mut line: Vec<char> = Vec::new();
        for y in pos.1..pos.1+3 {
            line.push(grid[y][x]);
        }
        grid_slice.push(line);
    }

    cross_found(&grid_slice)
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

    let xsize = input[0].len();
    let ysize = input.len();

    let mut matches = 0usize;

    for x in 0..xsize-2 {
        for y in 0..ysize-2 {
            matches += check_slice(&input, (x, y));
        }
    }

    return matches;
}

#[test]
fn test() {
    setup_tracing();
    assert_eq!(9, get_answer("test.a"));
}
