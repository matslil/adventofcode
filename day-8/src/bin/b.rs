use tracing_subscriber::{filter, prelude::*};
use std::{fs::File, sync::Arc};
use tracing::{info, debug};
use std::io::{BufRead, BufReader};
use rust_tools::grid2d::Grid2D;
use itertools::Itertools;

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

#[derive(PartialEq, Eq, Clone, Default)]
enum Antenna {
    #[default]
    Empty,
    Occupied(char),
}

impl std::fmt::Display for Antenna {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match self {
            Antenna::Empty => &'.',
            Antenna::Occupied(ch) => ch,
        })
    }
}

fn valid_node(size: (usize, usize), node: (isize, isize)) -> bool {
    if node.0 >= 0 && node.0 < size.0 as isize
        && node.1 >= 0 && node.1 < size.1 as isize {
            debug!("valid_node({:?} -> true", node);
            true
    } else {
        debug!("valid_node({:?} -> false", node);
        false
    }
}

fn find_antinodes(size:( usize,  usize), pair: ((usize, usize), (usize, usize))) -> Vec<(isize, isize)> {
    let dist = (pair.0.0 as isize - pair.1.0 as isize, pair.0.1 as isize - pair.1.1 as isize);
    let mut antinodes: Vec<(isize, isize)> = Vec::new();

    let _span = tracing::span!(tracing::Level::INFO, "find_antinodes", "{:?}", pair).entered();

    let mut current = (pair.0.0 as isize, pair.0.1 as isize);
    antinodes.push(current);
    loop {
        current = (current.0 - dist.0, current.1 - dist.1);
        let _span0 = tracing::span!(tracing::Level::INFO, "sub", "{:?}", current).entered();
        if !valid_node(size, current) {
            break;
        }
        antinodes.push(current);
    }

    let mut current = (pair.0.0 as isize, pair.0.1 as isize);
    loop {
        let _span0 = tracing::span!(tracing::Level::INFO, "add", "{:?}", current).entered();
        current = (current.0 + dist.0, current.1 + dist.1);
        if !valid_node(size, current) {
            break;
        }
        antinodes.push(current);
    }

    debug!("=> {:?}", antinodes);
    antinodes
}

fn find_pairs(antennas: &Grid2D<Antenna>) -> Vec<((usize, usize),(usize, usize))> {
    let mut pairs: Vec<((usize, usize),(usize, usize))> = Vec::new();

    for x in 0..antennas.cols() {
        for y in 0..antennas.rows() {
            let _trace0 = tracing::span!(tracing::Level::INFO, "pair.0", "({}, {})", x, y).entered();
            if let Antenna::Occupied(antenna) = antennas[(x, y)] {
                for x1 in 0..antennas.cols() {
                    for y1 in 0..antennas.rows() {
                        let _trace0 = tracing::span!(tracing::Level::INFO, "pair.1", "{}: ({}, {})", antenna, x1, y1).entered();
                        if (x, y) != (x1, y1) {
                            if let Antenna::Occupied(other_antenna) = antennas[(x1, y1)] {
                                if antenna == other_antenna {
                                    let pair = if (y < y1) || (y == y1 && x < x1) {
                                        ((x, y), (x1, y1))
                                    } else {
                                        ((x1, y1), (x, y))
                                    };
                                    if !pairs.contains(&pair) {
                                        debug!("Added {:?}", pair);
                                        pairs.push(pair);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    pairs
}

fn get_answer(file: &str) -> usize {
    let antennas  = Grid2D::new_translator(&mut BufReader::new(File::open(file).unwrap()).lines()
        .filter_map(Result::ok), |ch| if ch == &'.' { Antenna::Empty } else { Antenna::Occupied(*ch) });

    let _span = tracing::span!(tracing::Level::INFO, "map", "{} x {}", antennas.cols(), antennas.rows()).entered();
    debug!("\n   0   1   2   3   4   5   6   7   8   9  10  11\n{:#}", antennas);
    let size = (antennas.cols(), antennas.rows());

    let pairs = find_pairs(&antennas);

    debug!("Pairs: {:?}", pairs);

    let antinodes: Vec<(isize, isize)> = pairs.into_iter().flat_map(|pair| find_antinodes(size, pair)).unique().collect();
    antinodes.len()
}

#[test]
fn test() {
    setup_tracing();
    assert_eq!(9, get_answer("test.b"));
    assert_eq!(34, get_answer("test.a"));
}
