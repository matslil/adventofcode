use tracing_subscriber::{filter, prelude::*};
use std::{fs::File, sync::Arc};
use tracing::{info, debug};
use std::io::{BufRead, BufReader};
use itertools::Itertools;
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

fn main() {
    setup_tracing();
    info!("{}", get_answer("input", (71, 71), 1024));
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum MapItem {
    #[default]
    Empty,
    Corrupted,
    Step,
}

impl std::fmt::Display for MapItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match self {
            MapItem::Empty => '.',
            MapItem::Corrupted => '#',
            MapItem::Step => 'O',
        })
    }
}

fn path_exists(map_arg: &Grid2D<MapItem>, goal: (usize, usize)) -> bool {
    let mut map = map_arg.clone();
    let mut queue: Vec<(usize, usize)> = vec![(0, 0)];

    loop {
        let mut queue_next: Vec<(usize, usize)> = Vec::new();

        for pos in queue {
            for successor in map.successors_with(pos, |_, p|
                map[p] != MapItem::Corrupted && map[p] != MapItem::Step && (p.0 == pos.0 || p.1 == pos.1)
            ) {
                if successor == goal {
                    return true;
                }
                map[successor] = MapItem::Step;
                queue_next.push(successor);
            }
        }

        if queue_next.len() == 0 {
            return false;
        }

        queue = queue_next;
    }
}

fn get_answer(file: &str, size: (usize, usize), nr_rounds: usize) -> String {
    let input: Vec<(usize, usize)> = BufReader::new(File::open(file).unwrap()).lines()
        .filter_map(Result::ok)
        .map(|line| line
            .split(',')
            .map(|value| value.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap()
        )
        .collect::<Vec<_>>();

    let mut map: Grid2D<MapItem> = Grid2D::default();
    map.set_size(size, MapItem::default());

    for idx in 0..nr_rounds {
        map[input[idx]] = MapItem::Corrupted;
    }

    debug!("\n{}", map);

    for idx in nr_rounds..input.len() {
        map[input[idx]] = MapItem::Corrupted;
        if !path_exists(&map, (size.0-1, size.1-1)) {
            return format!("{},{}", input[idx].0, input[idx].1).to_string();
        }
    }
    "None".to_string()
}

#[test]
fn test() {
    setup_tracing();
    assert_eq!("6,1", get_answer("test.a", (7, 7), 12));
}
