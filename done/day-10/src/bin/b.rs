use tracing_subscriber::{filter, prelude::*};
use std::{fs::File, sync::Arc};
use tracing::{info, debug};
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use rust_tools::grid2d::Grid2D;

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
                .with_filter(filter::LevelFilter::INFO)
                .and_then(debug_log)
        )
        .init();
}

fn get_trail_score(map: &Grid2D<u32>, start: (usize, usize)) -> Vec<(usize, usize)> {
    let level = map[start];
    let _span = tracing::span!(tracing::Level::INFO, "", "{}: {:?}", level, start).entered();
    if level == 9 {
        debug!("Found");
        return Vec::from([start]);
    }
    let next = map.successors_with(start, |&next_level, next_pos| next_level == level + 1 && (next_pos.0 == start.0 || next_pos.1 == start.1));

    next.into_iter().fold(Vec::new(), |mut acc, value| {
        acc.append(&mut get_trail_score(map, value));
        acc
    })
}

fn main() {
    setup_tracing();
    info!("{:?}", get_answer("input"));
}

fn get_answer(file: &str) -> usize {
    let map = Grid2D::new(&mut BufReader::new(File::open(file).unwrap()).lines()
        .filter_map(Result::ok),
        HashMap::from([
            ('0', 0),
            ('1', 1),
            ('2', 2),
            ('3', 3),
            ('4', 4),
            ('5', 5),
            ('6', 6),
            ('7', 7),
            ('8', 8),
            ('9', 9)]));

    let mut result = 0usize;

    for x in 0..map.cols() {
        for y in 0..map.rows() {
            if map[(x, y)] == 0 {
                result += get_trail_score(&map, (x, y)).into_iter().count();
            }
        }
    }

    return result;
}

#[test]
fn test() {
    setup_tracing();
    assert_eq!(81, get_answer("test.a"));
}
