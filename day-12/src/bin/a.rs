use tracing::{self, info};
use tracing_subscriber::{filter, prelude::*};
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::sync::Arc;
use itertools::Itertools;

fn print_map(map: &Vec<Vec<bool>>) {
    let mut map_str: String = String::new();

    for row in map {
        map_str.push('\n');
        for cell in row {
            map_str.push(if *cell { '#' } else { '.' });
        }
    }
    info!("{}", map_str);
}

fn get_galaxies(file: &str) -> Vec<(usize, usize)> {
    let map: Vec<Vec<bool>> = BufReader::new(File::open(file).unwrap())
        .lines()
        .map(|e| e
            .unwrap()
            .chars()
            .map(|e| if e == '#' { true } else { false })
            .collect::<Vec<bool>>()
        )
        .collect::<Vec<Vec<bool>>>();

    print_map(&map);

    let mut empty_rows: Vec<usize> = Vec::new();
    let mut empty_cols: Vec<usize> = Vec::new();
    for (y, row) in map.iter().enumerate() {
        let mut row_empty = true;
        for cell in row {
            if *cell {
                row_empty = false;
                break;
            }
        }
        if row_empty {
            empty_rows.push(y);
        }
    }
    for x in 0..map[0].len() {
        let mut col_empty = true;
        for y in 0..map.len() {
            if map[y][x] {
                col_empty = false;
                break;
            }
        }
        if col_empty {
            empty_cols.push(x);
        }
    }

    let mut expanded_map: Vec<Vec<bool>> = Vec::new();

    let mut empty_row: Vec<bool> = Vec::new();
    empty_row.resize(map[0].len() + empty_cols.len(), false);

    for (y, row) in map.into_iter().enumerate() {
        if empty_rows.contains(&y) {
            expanded_map.push(empty_row.clone());
            expanded_map.push(empty_row.clone());
        } else {
            let mut expanded_row: Vec<bool> = Vec::new();
            for (x, cell) in row.iter().enumerate() {
                if empty_cols.contains(&x) {
                    expanded_row.push(false);
                    expanded_row.push(false);
                } else {
                    expanded_row.push(*cell);
                }
            }
            expanded_map.push(expanded_row);
        }
    }

    print_map(&expanded_map);

    let mut galaxies: Vec<(usize, usize)> = Vec::new();

    for (y, row) in expanded_map.into_iter().enumerate() {
        for (x, cell) in row.into_iter().enumerate() {
            if cell {
                galaxies.push((x, y));
            }
        }
    }

    galaxies
}

fn distance(a: (usize, usize), b: (usize, usize)) -> usize {
    let dist = (if a.0 > b.0 {
        a.0 - b.0
    } else {
        b.0 - a.0
    }) + (if a.1 > b.1 {
        a.1 - b.1
    } else {
        b.1 - a.1
    });

    info!("Distance {:?} -> {:?}: {}", a, b, dist);
    dist
}

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
    setup_tracing();
    println!("{:?}", get_answer("input"));
}

fn get_answer(file: &str) -> usize {
    let galaxies = get_galaxies(file);

    galaxies.into_iter().combinations(2).map(|pair| distance(pair[0], pair[1])).into_iter().sum()
}

#[test]
fn test() {
    setup_tracing();
    assert_eq!(374, get_answer("test"));
}
