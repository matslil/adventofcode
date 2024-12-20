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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum NewMapItem {
    #[default]
    Empty,
    Wall,
    BoxLeft,
    BoxRight,
    Robot,
}

impl std::fmt::Display for NewMapItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match self {
            NewMapItem::Empty => '.',
            NewMapItem::Wall => '#',
            NewMapItem::BoxLeft => '[',
            NewMapItem::BoxRight => ']',
            NewMapItem::Robot => '@',
        })
    }
}

fn other(from: (NewMapItem, (usize, usize))) -> (NewMapItem, (usize, usize)) {
    if from.0 == NewMapItem::BoxLeft {
        (NewMapItem::BoxRight, (from.1.0+1, from.1.1))
    } else {
        (NewMapItem::BoxLeft, (from.1.0-1, from.1.1))
    }
}

fn do_move(map: &mut Grid2D<NewMapItem>, robot_pos: &mut (usize, usize), dir: StraightDirection) {
    debug!("Move: {:?}", dir);
    let to_pos = dir.follow(*robot_pos, 1).unwrap();
    if map[to_pos] == NewMapItem::Wall {
        return;
    }
    if map[to_pos] == NewMapItem::Empty {
        *robot_pos = to_pos;
        return;
    }
    let mut boxes: Vec<Vec<(NewMapItem, (usize, usize))>> = Vec::new();
    boxes.push(vec![(map[to_pos], to_pos), other((map[to_pos], to_pos))]);

    loop {
        let new_entries: Vec<(NewMapItem, (usize, usize))> = Vec::new();
        match dir {
            StraightDirection::North, StraightDirection::South => {
                    let next_entry = map[dir.follow(entry.1, 1).unwrap()];
                    if newxt_entry == NewMapItem::Wall {
                        return;
                    }
                    if next_entry == NewMapItem::BoxLeft || next_entry == NewMapItem::BoxRight {
                        let first = (next_entry, dir.follow(entry.1, 1));
                        new_entries.push(first);
                        new_entries.push(other(first));
                        all_free = false;
                    }
            },
            StrightDirection::South => {

        }
    }


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

fn print_map(map: &Grid2D<NewMapItem>, robot_pos: &(usize, usize)) {
    let mut map_copy = map.clone();
    map_copy[*robot_pos] = NewMapItem::Robot;
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
    let map_original = Grid2D::new(&mut input[0].iter(), HashMap::from([
            ('.', MapItem::Empty),
            ('#', MapItem::Wall),
            ('O', MapItem::FoodBox),
            ('@', MapItem::Robot),
    ]));

    let mut map: Grid2D<NewMapItem> = Grid2D::default();

    map.set_size((map_original.cols() * 2, map_original.rows()));

    for x in 0..map_original.cols() {
        for y in 0..map_original.rows() {
            let entry = match map_original[(x,y)] {
                MapItem::Empty => (NewMapItem::Empty, NewMapItem::Empty),
                MapItem::Wall => (NewMapItem::Wall, NewMapItem::Wall),
                MapItem::Robot => (NewMapItem::Robot, NewMapItem::Empty),
                MapItem::FoodBox => (NewMapItem::BoxLeft, NewMapItem::BoxRight),
                };
            map[(x*2, y)] = entry.0;
            map[(x*2+1, y)] = entry.1;
        }
    }

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
            if map[(x,y)] == NewMapItem::Robot {
                robot_pos = (x,y);
                map[robot_pos] = NewMapItem::Empty;
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
