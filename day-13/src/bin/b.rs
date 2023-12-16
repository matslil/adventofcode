// 59099 too high

use tracing::{self, info};
use tracing_subscriber::{filter, prelude::*};
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::sync::Arc;
use grid::Grid;
use std::iter::zip;
use itertools::Itertools;
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

fn grid_line(line: Vec<bool>) -> String {
    let mut string = String::new();
    for entry in line {
        string.push(if entry {
            '#'
        } else {
            '.'
        })
    }
    string
}

// Returns number of columns to the left of reflect point
fn reflection_col(grid: &Grid<bool>) -> usize {
    info!("refelction_col");
    let split = grid.cols().rounded_div(2);
    for col in (1..split).rev().interleave(split..grid.cols()) {
        info!("Try split at col {}", col);
        let mut diff = 0usize;
        for cols in zip((0..col).rev(), col..grid.cols()) {
            let col0 = grid.iter_col(cols.0).map(|e| *e).collect::<Vec<bool>>();
            let col1 = grid.iter_col(cols.1).map(|e| *e).collect::<Vec<bool>>();
            let added = zip(grid.iter_col(cols.0), grid.iter_col(cols.1))
                .fold(0, |acc, e| { acc + if e.0 == e.1 { 0 } else { 1 } });
            diff += added;
            info!("({}) {} <=> {} ({}) +{} ({})", cols.0, grid_line(col0), grid_line(col1), cols.1, added, diff);
        }
        if diff == 1 {
            info!("Found col {}", col);
            return col;
        }
    }

    0
}

fn reflection_row(grid: &Grid<bool>) -> usize {
    info!("reflection_row");
    let split = grid.rows().rounded_div(2);
    for row in (1..split).rev().interleave(split..grid.rows()) {
        info!("Try split at row {}", row);
        let mut diff = 0usize;
        for rows in zip((0..row).rev(), row..grid.rows()) {
            let row0 = grid.iter_row(rows.0).map(|e| *e).collect::<Vec<_>>();
            let row1 = grid.iter_row(rows.1).map(|e| *e).collect::<Vec<_>>();
            let added = zip(grid.iter_row(rows.0), grid.iter_row(rows.1))
                .fold(0, |acc, e| { acc + if e.0 == e.1 { 0 } else { 1 } });
            diff += added;
            info!("({}) {} <=> {} ({}) +{} ({})", rows.0, grid_line(row0), grid_line(row1), rows.1, added, diff);
        }
        if diff == 1{
            info!("Found row {}", row);
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

    let mut count = 0usize;
    for (idx, grid) in grids.into_iter().enumerate() {
        info!("=== Map {} ===", idx);
        let col = reflection_col(&grid);
        let row = reflection_row(&grid);
        assert!(col != 0 || row != 0);
        if col > row {
            info!("Chosen col: {}", col);
            count += col;
        } else {
            info!("Chosen row: {}", row);
            count += row * 100;
        }
    }

    count
}

#[test]
fn test() {
    assert_eq!(400, get_answer("test"));
}
