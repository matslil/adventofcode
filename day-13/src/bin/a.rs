// 14677 too low
// 28167 too low

use tracing::{self, info};
use tracing_subscriber::{filter, prelude::*};
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::sync::Arc;
use grid::Grid;
use std::iter::zip;
use itertools::interleave;
use rounded_div::RoundedDiv;

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

// Returns number of columns to the left of reflect point
fn reflection_col(grid: &Grid<bool>) -> usize {
    let split = grid.cols().rounded_div(2);
    for col in interleave(split..1, (split+1)..grid.cols()) {
        let mut a_match = true;
        for cols in zip((0..col).rev(), col..grid.cols()) {
            if ! zip(grid.iter_col(cols.0), grid.iter_col(cols.1))
                .fold(true, |acc, e| { acc && (e.0 == e.1) })
            {
                a_match = false;
                break;
            }
        }
        if a_match {
            return col;
        }
    }

    0
}

fn reflection_row(grid: &Grid<bool>) -> usize {
    let split = grid.rows().rounded_div(2);
    for row in interleave(split..1, (split+1)..grid.rows()) {
        let mut a_match = true;
        for rows in zip((0..row).rev(), row..grid.rows()) {
            if ! zip(grid.iter_row(rows.0), grid.iter_row(rows.1))
                .fold(true, |acc, e| { acc && (e.0 == e.1) })
            {
                a_match = false;
                break;
            }
        }
        if a_match {
            return row;
        }
    }

    0
}

fn get_grid(iter: &mut dyn Iterator<Item=String>) -> Grid<bool> {
    let mut grid: Grid<bool> = Grid::new(0, 0);

    for line in iter {
        if line == "" {
            return grid;
        }
        grid.push_row(line.chars().map(|e| e == '#').collect());
    }

    grid
}

fn grid_as_str(grid: &Grid<bool>) -> String {
    let mut string = String::new();
    for row in 0..grid.rows() {
        string.push('\n');
        for col in 0..grid.cols() {
            string.push(if grid[(row, col)] { '#' } else { '.' });
        }
    }
    string
}

fn get_answer(file: &str) -> usize {
    setup_tracing();
    let mut grids: Vec<Grid<bool>> = Vec::new();

    let mut iter = BufReader::new(File::open(file).unwrap())
        .lines()
        .map(|e| e.unwrap())
        .peekable();

    while iter.peek().is_some() {
        grids.push(get_grid(&mut iter));
//        info!("{}", grid_as_str(&grids[grids.len()-1]));
    }

    grids.into_iter().map(|grid| reflection_col(&grid) + (100 * reflection_row(&grid))).sum()
}

#[test]
fn test() {
    assert_eq!(405, get_answer("test"));
}
