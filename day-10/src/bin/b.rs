// 215 too low
// 1151 too high
// 13979 too high
// 484 nope
// 487 nope
// 488 nope

use tracing::{self, info};
use tracing_subscriber::{filter, prelude::*};
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::sync::Arc;
use pathfinding::prelude::Grid;
use rust_tools::direction::{Turn, StraightDirection, Pipe};
use rust_tools::grid2d::Grid2D;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MapEntry {
    Pipe(Pipe),
    Ground,
    Start,
}

impl std::fmt::Display for MapEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MapEntry::Pipe(p) => write!(f, "{}", p),
            MapEntry::Ground => write!(f, "."),
            MapEntry::Start => write!(f, "S"),
        }
    }
}

struct Map {
    map: Grid2D<MapEntry>,
}

impl Map {
    fn new(file: &str) -> Self {
        Self { map:  Grid2D::new(
            &mut BufReader::new(&mut File::open(file).unwrap()).lines().map(|e| e.unwrap()),
            HashMap::from([
                ('|', MapEntry::Pipe(Pipe::NorthSouth)),
                ('-', MapEntry::Pipe(Pipe::EastWest)),
                ('7', MapEntry::Pipe(Pipe::SouthWest)),
                ('F', MapEntry::Pipe(Pipe::SouthEast)),
                ('L', MapEntry::Pipe(Pipe::NorthEast)),
                ('J', MapEntry::Pipe(Pipe::NorthWest)),
                ('.', MapEntry::Ground),
                ('S', MapEntry::Start),
            ])
        )}
    }

    fn width(&self) -> usize {
        self.map.cols()
    }

    fn height(&self) -> usize {
        self.map.rows()
    }

    fn max_x(&self) -> usize {
        self.width() - 1
    }

    fn max_y(&self) -> usize {
        self.height() -1
    }

    fn pipe_loop(&self, start: (usize, usize)) -> Option<Vec<(usize, usize)>> {
        let mut chain: Vec<(usize, usize)> = Vec::new();

        if let MapEntry::Pipe(_) = self[start] {
            let mut current = start;

            loop {
                if let MapEntry::Pipe(from_entry) = self.map[current] {
                    chain.push(current);
                    let next = self.map.successors_with(current, |&to_entry, to| {
                        if chain.len() > 1 && chain[chain.len()-2] == to {
                            false
                        } else if let MapEntry::Pipe(to_pipe) = to_entry {
                            if let Some(dir) = StraightDirection::from_pos(current, to) {
                                let result = from_entry.is_connected_to(dir, to_pipe);
                                result
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    });
                    if next.len() == 0 {
                        return Some(chain);
                    } else if next.len() != 1 {
                        return None
                    }
                    current = next[0];
                } else {
                    return None;
                }
            }
        } else {
            return None
        }
    }

    fn clean(&mut self) -> (Turn, Vec<(usize, usize)>) {
        let mut start: (usize, usize) = (0, 0);

        for (y, row) in self.map.row_iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if **cell == MapEntry::Start {
                    start = (x, y);
                    break;
                }
            }
        }

        info!("start: {:?} {}", start, self[start]);

        // Try all four directions, until a complete loop has been detected
        let mut pipes: Vec<(usize, usize)> = Vec::new();
        pipes.push(start);
        for node in self.map.successors(start) {
            info!("node: {:?}", node);
            if let Some(mut a_loop) = self.pipe_loop(node) {
                if a_loop.len() > 1 {
                    pipes.append(&mut a_loop);
                }
                break;
            }
        }

        assert!(pipes.len() > 1);

        let turns = if pipes.windows(2)
            .map(|e| StraightDirection::from_pos(e[0], e[1]).unwrap())
            .collect::<Vec<_>>()
            .windows(2).map(|e| Turn::from([e[0], e[1]]))
            .fold(0isize, |acc, e|
                acc + match e {
                    Turn::Right => 1,
                    Turn::Left => -1,
                    Turn::Straight => 0,
                }
            ) > 0 { Turn::Right } else { Turn::Left };

        info!("{:?}", turns);

        let dir_prev = StraightDirection::from_pos(start, pipes[pipes.len()-1]).unwrap();
        let dir_next = StraightDirection::from_pos(start, pipes[1]).unwrap();

        self[start] = MapEntry::Pipe(Pipe::try_from([dir_prev, dir_next]).unwrap());

        for row in 0..self.map.rows() {
            for col in 0..self.map.cols() {
                let pos = (col, row);
                if !pipes.contains(&pos) {
                    self.map[pos] = MapEntry::Ground;
                }
            }
        }

        info!("{}", self);

        (turns, pipes)
    }

    fn start_fill(&self, pipe: &Vec<(usize, usize)>, turn: Turn) -> Vec<(usize, usize)> {
        let mut start_fill: Vec<(usize, usize)> = Vec::new();
        let mut dir;

        for (node, next) in pipe.windows(2).map(|e| (e[0], e[1])) {
            dir = StraightDirection::from_pos(node, next).unwrap();
            let fill_dir;
            if turn == Turn::Right {
                fill_dir = dir.turn(Turn::Right);
            } else {
                fill_dir = dir.turn(Turn::Left);
            }
//            info!("{:?} -> {:?}", node, next);
            if let Some(fill_node) = fill_dir.follow(node, 1) {
                if self.map.is_in_range(fill_node) && self[fill_node] == MapEntry::Ground {
                    start_fill.push(fill_node);
                    if let Some(another_fill) = dir.follow(fill_node, 1) {
                        if self.map.is_in_range(another_fill) && self[another_fill] == MapEntry::Ground {
                            start_fill.push(another_fill);
                        }
                    }
                }
            }
        }

//        info!("{:?}", start_fill);

        start_fill.into_iter().filter(|pos| self[*pos] == MapEntry::Ground).collect::<Vec<(usize, usize)>>()
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in self.map.row_iter() {
            write!(f, "\n")?;
            for cell in row {
                write!(f, "{}", cell)?;
            }
        }
        Ok(())
    }
}

impl std::ops::Index<(usize, usize)> for Map {
    type Output = MapEntry;
    fn index(&self, index: (usize, usize)) -> &MapEntry {
        &self.map[index]
    }
}

impl std::ops::IndexMut<(usize, usize)> for Map {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut MapEntry {
        &mut self.map[index]
    }
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

fn successors(map: &Map, at: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut list: Vec<(usize, usize)> = Vec::new();

    if let Some(node) = StraightDirection::North.follow(*at, 1) {
        if map.map.is_in_range(node) {
            list.push(node);
        }
    }
    if let Some(node) = StraightDirection::South.follow(*at, 1) {
        if map.map.is_in_range(node) {
            list.push(node);
        }
    }
    if let Some(node) = StraightDirection::East.follow(*at, 1) {
        if map.map.is_in_range(node) {
            list.push(node);
        }
    }
    if let Some(node) = StraightDirection::West.follow(*at, 1) {
        if map.map.is_in_range(node) {
            list.push(node);
        }
    }

    info!("list: {:?}", list);

    list.into_iter().filter(|pos| map[*pos] == MapEntry::Ground).collect::<Vec<(usize, usize)>>()
}

fn fill(map: &Map, start_fill: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut visited: Vec<(usize, usize)> = start_fill.clone();

    info!("start_fill: {:?}", start_fill);
    let mut stack: Vec<(usize, usize)> = start_fill;
    while let Some(node) = stack.pop() {
        info!("node: {:?}", node);
        let add = successors(map, &node);
        info!("successors: {:?}", add);
        for prospect in &add {
            if ! visited.contains(prospect) {
                stack.push(*prospect);
                visited.push(*prospect);
            }
        }
    }

    info!("Stack: {:?}", stack);
    info!("Fill: {}", visited.len());
    visited
}

fn get_answer(file: &str, right: bool) -> usize {
    let mut map = Map::new(file);
    info!("{}", map);
    let (turn, visited) = map.clean();

    let mut start_fill = map.start_fill(&visited, turn);
    start_fill.sort();
    start_fill.dedup();
    let mut grid = Grid::new(map.width(), map.height());

    for node in &start_fill {
        grid.add_vertex((node.0, node.1));
    }

    info!("{:?}", grid);

    let all_nodes = fill(&map, start_fill);

    let mut grid = Grid::new(map.width(), map.height());

    for node in &all_nodes {
        grid.add_vertex((node.0, node.1));
    }

    info!("{:?}", grid);

    all_nodes.len()
}

fn main() {
    setup_tracing();
    println!("{:?}", get_answer("input", false));
}

#[test]
fn test() {
    setup_tracing();
    assert_eq!(4, get_answer("test.3", true));
    assert_eq!(4, get_answer("test.4", true));
    assert_eq!(8, get_answer("test.5", false));
    assert_eq!(10, get_answer("test.6", true));
}

