// 1757008096 too high
// 1757008038 too high

use tracing::{self, info};
use tracing_subscriber::{filter, prelude::*};
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::sync::Arc;

#[derive(PartialEq, Eq, Debug)]
enum Dir {
    EastWest,   // |
    NorthSouth, // -
    NorthEast,  // L
    NorthWest,  // J
    SouthWest,  // 7
    SouthEast,  // F
    Ground,     // .
    Start,      // S
}

impl std::convert::From<char> for Dir {
    fn from(value: char) -> Self {
        match value {
            '|' => Dir::NorthSouth,
            '-' => Dir::EastWest,
            'L' => Dir::NorthEast,
            'J' => Dir::NorthWest,
            '7' => Dir::SouthWest,
            'F' => Dir::SouthEast,
            '.' => Dir::Ground,
            'S' => Dir::Start,
            _   => panic!("{}: Unknown character", value),
        }
    }
}

impl std::fmt::Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Dir::EastWest   => write!(f, "{}", '─'),
            Dir::NorthSouth => write!(f, "{}", '│'),
            Dir::NorthEast  => write!(f, "{}", '╰'),
            Dir::NorthWest  => write!(f, "{}", '╯'),
            Dir::SouthWest  => write!(f, "{}", '╮'),
            Dir::SouthEast  => write!(f, "{}", '╭'),
            Dir::Ground     => write!(f, "{}", ' '),
            Dir::Start      => write!(f, "{}", '▒'),
        }
    }
}

impl Dir {
    fn next(&self, prev: &(usize, usize), curr: &(usize, usize)) -> (usize, usize) {
        let diff: (isize, isize) =
            (prev.0 as isize - curr.0 as isize,
             prev.1 as isize - curr.1 as isize);

        let apply: (isize, isize) = match self {
            Dir::EastWest   => if diff == (-1, 0) {
                (2, 0)
            } else {
                (-2, 0)
            }
            Dir::NorthSouth => if diff == (0, -1) {
                (0, 2)
            } else {
                (0, -2)
            }
            Dir::NorthEast  => if diff == (0, -1) {
                (1, 1)
            } else {
                (-1, -1)
            }
            Dir::NorthWest  => if diff == (0, -1) {
                (-1, 1)
            } else {
                (1, -1)
            }
            Dir::SouthWest  => if diff == (-1, 0) {
                (1, 1)
            } else {
                (-1, -1)
            }
            Dir::SouthEast  => if diff == (1, 0) {
                (-1, 1)
            } else {
                (1, -1)
            }
            _               => panic!("{:?}: Direction not supported!", self),
        };

        info!("next({:?}, {:?}), diff: {:?}, apply: {:?}", prev, curr, diff, apply);

        ((prev.0 as isize + apply.0) as usize, (prev.1 as isize + apply.1) as usize)
    }
}

struct Map {
    map: Vec<Vec<Dir>>,
}

impl Map {
    fn new(file: &str) -> Self {
        Self { map: BufReader::new(File::open(file).unwrap())
            .lines()
                .map(|e| e
                    .unwrap()
                    .chars()
                    .map(|e| Into::<Dir>::into(e))
                    .collect::<Vec<Dir>>()
                )
                .collect::<Vec<Vec<Dir>>>()
        }
    }

    fn path_len(&self) -> usize {
        let mut start: (usize, usize) = (0, 0);

        for (y, row) in self.map.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell == Dir::Start {
                    start = (x, y);
                    break;
                }
            }
        }

        info!("start: {:?} {}", start, self[start]);

        // Manual look at the map tells that we can start at east
        let mut nr_steps: usize = 1;
        let mut prev: (usize, usize) = start;
        let mut node: (usize, usize) = (start.0 + 1, start.1);

        while node != start {
            nr_steps += 1;
            let new_node = self[node].next(&prev, &node);
            prev = node;
            node = new_node;
            info!("{:?} -> {:?} {}", prev, node, self[prev]);
        }

        info!("finish: {:?} {}", node, self[node]);

        nr_steps / 2 + if nr_steps % 2 != 0 { 1 } else { 0 }
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in &self.map {
            write!(f, "\n")?;
            for cell in row {
                write!(f, "{}", cell)?;
            }
        }
        Ok(())
    }
}

impl std::ops::Index<(usize, usize)> for Map {
    type Output = Dir;
    fn index(&self, index: (usize, usize)) -> &Dir {
        &self.map[index.1][index.0]
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

fn main() {
    setup_tracing();
    println!("{:?}", get_answer("input"));
}

fn get_answer(file: &str) -> usize {
    let map = Map::new(file);
    info!("{}", map);
    map.path_len()
}

#[test]
fn test() {
    setup_tracing();
    assert_eq!(4, get_answer("test.1"));
    assert_eq!(8, get_answer("test.2"));
}
