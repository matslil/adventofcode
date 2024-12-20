use tracing_subscriber::{filter, prelude::*};
use std::{fs::File, sync::Arc};
use tracing::{info, debug, warn};
use std::io::{BufRead, BufReader};
use rust_tools::{direction::StraightDirection, grid2d::Grid2D, direction::Turn};
use std::collections::HashMap;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MapItem {
    Empty,
    Wall,
    EndTile,
    Start,
}

impl std::fmt::Display for MapItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match self {
            MapItem::Empty => '.',
            MapItem::Wall => '#',
            MapItem::EndTile => 'E',
            MapItem::Start => 'S',
        })
    }
}

fn main() {
    setup_tracing();
    info!("{:?}", get_answer("input"));
}

struct Head {
    pos: (usize, usize),
    points: usize,
    dir: StraightDirection,
    end_tile: bool,
    ignore: bool,
}

impl Head {
    fn move_pos(map: &Grid2D<MapItem>, head: &mut Head, to: (usize, usize)) {
        head.pos = to;
        if map[to] == MapItem::EndTile {
            head.end_tile = true;
        }
        if map[to] == MapItem::Start {
            head.ignore = true;
        }
    }
}

fn next_steps(map: &Grid2D<MapItem>, pos: (usize, usize), dir: StraightDirection) -> (bool, bool) {
    let mut result: (bool, bool) = (false, false);

    if let Some(left) = dir.turn(Turn::Left).follow(pos, 1) {
        if map[left] != MapItem::Wall {
            result.0 = true;
        }
    }

    if let Some(right) = dir.turn(Turn::Right).follow(pos, 1) {
        if map[right] != MapItem::Wall {
            result.1 = true;
        }
    }

    result
}

fn go_forward(map: &Grid2D<MapItem>, pos: (usize, usize), dir: StraightDirection) -> (usize, (usize, usize)) {
    let mut curr = pos;
    let mut cost = 0usize;

    loop {
        if Some(next_pos) = dir.follow(curr, 1) {
        let next = map.successors(curr);
        if next != vec![dir.follow(curr, 1)] || map[next] != MapItem::Empty {
            break;
        }
        cost += 1;
        curr = next;
    }

    (cost, curr)
}

fn get_answer(file: &str) -> usize {
    let mut input = BufReader::new(File::open(file).unwrap()).lines()
        .filter_map(Result::ok);

    let map = Grid2D::new(&mut input, HashMap::from([
            ('#', MapItem::Wall),
            ('.', MapItem::Empty),
            ('E', MapItem::EndTile),
            ('S', MapItem::Start),
    ]));

    let mut min_points = Some(0usize);
    let mut heads: Vec<Head> = Vec::new();

    'outer: for x in 0..map.rows() {
        for y in 0..map.cols() {
            if map[(x,y)] == MapItem::Start {
                let new_head = Head {
                    pos: (x,y),
                    points: 0,
                    dir: StraightDirection::East,
                    end_tile: false,
                    ignore: false,
                };
                heads.push(new_head);
                break 'outer;
            }
        }
    }

    loop {
        let mut added_heads: Vec<Head> = Vec::new();
        let mut all_finished = true;

        for head in heads {
            if let Some(points) = min_points {
                if head.points > points {
                    head.ignore = true;
                }
                continue;
            }

            if head.end_tile && !head.ignore {
                if let Some(points) = min_points {
                    if points < head.points {
                        head.ignore = false;
                    }
                }
                continue;
            }
            if map[head.pos] == MapItem::EndTile {
                head.end_tile = true;
                continue;
            }

            all_finished = false;

            let forward = go_forward(&map, head.pos, head.dir);
            head.points += forward.0;
            head.pos = forward.1;
            if map[head.pos] == MapItem::EndTile {
                head.end_tile = true;
                continue;
            }
            let turns = next_steps(&map, head.pos, head.dir);
            if turns.0 {
                head.points += 1000;
                head.pos = head.pos;
                head.dir = head.dir.turn(Turn::Left);
                if map[head.pos] == MapItem::EndTile {
                    head.end_tile = true;
                    continue;
                }
            }
            if turns.1 {
                added_heads.push( Head {
                    points: head.points + 1000,
                    pos: head.pos,
                    dir: head.dir.turn(Turn::Right),
                    end_tile: false,
                    ignore: false,
                });
            }
        }

        if all_finished {
            break;
        }
        heads.append(&mut added_heads);
    }

    return min_points.unwrap();
}

#[test]
fn test() {
    setup_tracing();
    assert_eq!(4, get_answer("test.a"));
}
