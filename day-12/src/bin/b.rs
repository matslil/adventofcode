use tracing_subscriber::{filter, prelude::*};
use std::{fs::File, sync::Arc};
use tracing::{info, debug};
use std::io::{BufRead, BufReader};
use rust_tools::grid2d::Grid2D;
use itertools::Itertools;
use std::collections::VecDeque;
use tracing::instrument;

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

#[instrument(fields(pos))]
fn create_region(map: &Grid2D<char>, pos: (usize, usize)) -> (char, Vec<(usize, usize)>) {
    let ch = map[pos];
    let mut area: Vec<(usize, usize)> = Vec::new();
    let mut stack: VecDeque<(usize, usize)> = VecDeque::new();

    stack.push_back(pos);
    area.push(pos);

    while let Some(next) = stack.pop_front() {
        debug!("Checking: {:?}", next);
        let list = map.successors_with(next, |&entry, p| entry == ch && !area.contains(&p));
        debug!("List: {:?}", list);
        for entry in list.clone() {
            stack.push_back(entry);
        }
        area = area.into_iter().chain(list.into_iter()).unique().collect();
    }

    debug!(" -> {:?}", area);
    (ch, area)
}

fn pos_in_any_region(regions: &Vec<(char, Vec<(usize, usize)>)>, pos: (usize, usize)) -> bool {
    for region in regions {
        if region.1.contains(&pos) {
            return true;
        }
    }
    false
}

#[instrument]
fn fence_length(map: &Grid2D<char>, region: &Vec<(usize, usize)>) -> usize {
    let mut len = 0usize;

    for curr in region {
        len += map.successors_with(*curr, |_, pos| !region.contains(&pos) && (curr.0 == pos.0 || curr.1 == pos.1)).len();
        if curr.0 == 0 || curr.0 == map.cols()-1 {
            len += 1;
        }
        if curr.1 == 0 || curr.1 == map.rows()-1 {
            len += 1;
        }
    }

    debug!("{}", len);
    len
}

fn get_answer(file: &str) -> usize {
    let input: Grid2D<char> = Grid2D::new_translator(&mut BufReader::new(File::open(file).unwrap()).lines()
        .filter_map(Result::ok),
        |&c| c);
    let mut regions: Vec<(char, Vec<(usize, usize)>)> = Vec::new();

    for x in 0..input.cols() {
        for y in 0..input.rows() {
            if ! pos_in_any_region(&regions, (x, y)) {
                regions.push(create_region(&input, (x, y)));
            }
        }
    }

    return regions.into_iter().fold(0usize, |acc, entry| acc + fence_length(&input, &entry.1) * entry.1.len());
}

#[test]
fn test() {
    setup_tracing();
    assert_eq!(80, get_answer("test.a.0"));
    assert_eq!(436, get_answer("test.a.1"));
    assert_eq!(236, get_answer("test.b.0"));
    assert_eq!(368, get_answer("test.b.1"));
}
