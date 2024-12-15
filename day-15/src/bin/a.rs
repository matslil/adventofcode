// use flexi_logger;
// use log::{info, warn};

use tracing_subscriber::{filter, prelude::*};
use std::{fs::File, sync::Arc};
use tracing::{info, debug, warn};
use std::io::{BufRead, BufReader};
use rust_tools::direction::StraightDirection;
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum MapItem {
    #[default]
    Empty,
    Wall,
    FoodBox,
    Robot,
}

impl std::fmt::Display for MapItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match self {
            MapItem::Empty => '.',
            MapItem::Wall => '#',
            MapItem::FoodBox => 'O',
            MapItem::Robot => '@',
        })
    }
}

fn do_move(map: &mut Grid2D<MapItem>, robot_pos: &mut (usize, usize), dir: StraightDirection) {
    debug!("Move: {:?}", dir);
    let to_pos = dir.follow(*robot_pos, 1).unwrap();
    if map[to_pos] == MapItem::Wall {
        return;
    }
    if map[to_pos] == MapItem::Empty {
        *robot_pos = to_pos;
        return;
    }
    let mut next_pos = dir.follow(to_pos, 1).unwrap();
    while map[next_pos] != MapItem::Empty {
        if map[next_pos] == MapItem::Wall {
            return;
        }
        next_pos = dir.follow(next_pos, 1).unwrap();
        debug!("next_pos: {:?}", next_pos);
    }
    map[next_pos] = MapItem::FoodBox;
    map[to_pos] = MapItem::Empty;
    *robot_pos = to_pos;
}

fn print_map(map: &Grid2D<MapItem>, robot_pos: &(usize, usize)) {
    let mut map_copy = map.clone();
    map_copy[*robot_pos] = MapItem::Robot;
    debug!("\n{}", map_copy);
}

fn get_answer(file: &str) -> usize {
    let input = BufReader::new(File::open(file).unwrap()).lines()
        .filter_map(Result::ok)
        .fold(vec![Vec::new()], |mut acc, line| {
            if line.is_empty() {
                acc.push(Vec::new());
            } else {
                if let Some(last) = acc.last_mut() {
                    last.push(line);
                }
            }
            acc
        });
    let mut map = Grid2D::new(&mut input[0].iter(), HashMap::from([
            ('.', MapItem::Empty),
            ('#', MapItem::Wall),
            ('O', MapItem::FoodBox),
            ('@', MapItem::Robot),
    ]));

    let moves: Vec<StraightDirection> = input[1].iter().flat_map(|line| line.chars()).map(|entry| match entry {
        '^' => StraightDirection::North,
        '>' => StraightDirection::East,
        'v' => StraightDirection::South,
        '<' => StraightDirection::West,
        _ => panic!("What???"),
    }).collect();

    let mut robot_pos = (0usize, 0usize);
    'outer: for x in 0..map.cols() {
        for y in 0..map.rows() {
            if map[(x,y)] == MapItem::Robot {
                robot_pos = (x,y);
                map[robot_pos] = MapItem::Empty;
                break 'outer;
            }
        }
    }

    for next_move in moves {
        do_move(&mut map, &mut robot_pos, next_move);
        print_map(&map, &robot_pos);
    }

    let mut result = 0usize;

    for x in 0..map.cols() {
        for y in 0..map.rows() {
            if map[(x,y)] == MapItem::FoodBox {
                result += y * 100 + x;
            }
        }
    }

    result
}

#[test]
fn test() {
    setup_tracing();
    assert_eq!(2028, get_answer("test.a"));
}
