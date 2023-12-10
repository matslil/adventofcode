// 1757008096 too high
// 1757008038 too high

use tracing::{self, info};
use tracing_subscriber::{filter, prelude::*};
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::sync::Arc;

#[derive(PartialEq, Clone, Copy, Debug)]
enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {
    fn from_pos(from: (usize, usize), to: (usize, usize)) -> Self {
        match (to.0 as isize - from.0 as isize, to.1 as isize - from.1 as isize) {
            (0, -1) => Direction::North,
            (1, 0) => Direction::East,
            (0, 1) => Direction::South,
            (-1, 0) => Direction::West,
            _ => panic!("{:?} -> {:?}: Cannot translate to direction", from, to),
        }
    }
}

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

impl std::convert::From<[Direction;2]> for Dir {
    fn from(value: [Direction;2]) -> Self {
        if value.contains(&Direction::North) {
            if value.contains(&Direction::East) {
                Dir::NorthEast
            } else if value.contains(&Direction::South) {
                Dir::NorthSouth
            } else {
                Dir::NorthWest
            }
        } else if value.contains(&Direction::South) {
            if value.contains(&Direction::East) {
                Dir::SouthEast
            } else {
                Dir::SouthWest
            }
        } else {
            Dir::EastWest
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

    // Returns from <-> to directions
    fn directions(&self) -> Vec<Direction> {
        match self {
            Dir::EastWest   => vec![Direction::East, Direction::West],
            Dir::NorthSouth => vec![Direction::North, Direction::South],
            Dir::NorthEast  => vec![Direction::North, Direction::East],
            Dir::NorthWest  => vec![Direction::North, Direction::West],
            Dir::SouthWest  => vec![Direction::South, Direction::West],
            Dir::SouthEast  => vec![Direction::South, Direction::East],
            Dir::Ground     => Vec::new(),
            Dir::Start      => Vec::new(),
        }
    }
}

struct Map {
    map: Vec<Vec<Dir>>,
}

fn prev(visited: &Vec<(usize, usize)>) -> &(usize, usize) {
    &visited[visited.len() - 2]
}

fn curr(visited: &Vec<(usize, usize)>) -> &(usize, usize) {
    &visited[visited.len() -1]
}


impl Map {
    fn new(file: &str) -> Self {
        Self { map:  BufReader::new(File::open(file).unwrap())
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

    fn clean(&mut self) -> Vec<(usize, usize)> {
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
        let mut nodes: Vec<(usize, usize)> = Vec::new();
        nodes.push(start);
        nodes.push((start.0 + 1, start.1));

        loop {
            let next = self[*curr(&nodes)].next(
                prev(&nodes), curr(&nodes));
            if next == start {
                break;
            }
            nodes.push(next);
            info!("{:?} -> {:?} {}", prev(&nodes), curr(&nodes), self[*prev(&nodes)]);
        }

        for (y, row) in self.map.iter_mut().enumerate() {
            for (x, cell) in row.iter_mut().enumerate() {
                if ! nodes.contains(&(x, y)) {
                    *cell = Dir::Ground;
                }
            }
        }

        info!("Visited: {:?}", nodes);

        let dir_prev = Direction::from_pos(start, nodes[nodes.len()-1]);
        let dir_next = Direction::from_pos(start, nodes[1]);

        self[start] = [dir_prev, dir_next].into();

        info!("{}", self);

        nodes
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

impl std::ops::IndexMut<(usize, usize)> for Map {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Dir {
        &mut self.map[index.1][index.0]
    }
}

enum Passable {
    Ground,
    Wall,
    PassageVertical,
    PassageHorizontal,
    PassageBoth,
}

fn is_passable(map: &Map, from: (usize, usize), to: (usize, usize)) -> bool {
    let reverse_dir = std::collections::HashMap::from([
        (Direction::North, Direction::South),
        (Direction::South, Direction::North),
        (Direction::East,  Direction::West),
        (Direction::West,  Direction::Eqst)
    ]);
    let dir = Direction::from_pos(from, to);
    if map[from].directions().contains(dir) {
        false
    } else if map[to].directions().contains(reverse_dir[dir]) {
        false
    } else {
        true
    }
}

fn is_passable_edge(map: &Map, from: (usize, usize), edge_at: Direction) -> bool {
    ! map.directions().contains(edge_at)
}

struct NewMap {
    map: Vec<Vec<Passable>>,
}

impl NewMap {
    fn new(old_map: &Map, visited: &Vec<(usize, usize)>) -> Self {
        let mut map: Vec<Vec<Passable>> = Vec::new();
        let max_x = old_map[0].len() - 1;
        let max_y = old_map.len() - 1;

        // Default to ground
        for _ in 0..=max_y {
            let row: Vec<Passable> = Vec::new();
            row.resize(max_x + 1, Passable::Ground);
            map.push(row);
        }

        for item in visited {
            // Make them all walls or passage, depending on whether it is
            // possible to get between walls

            let dir = old_map[item];

            if (item.0 == 0 && item.1 == 0) ||
                (item.0 == max_x && item.1 == 0) ||
                    (item.0 == max_x && item.1 == max_y) ||
                    (item.0 == 0 && item.1 == max_y) {
                        if Passable::PassableBoth
            } else if item.0 == 0
            map.get_mut(item) = match *old_map[item] {
                Dir::NorthEast => if ,
                Dir::NorthSouth => ,
                Dir::NorthWest => ,
                Dir::SouthEast => ,
                Dir::SouthWest => ,
                Dir::EastWest => ,
                dir => panic!("{}: Unexpected direction", dir),
            }
        }
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
    let mut map = Map::new(file);
    info!("{}", map);
    map.clean();

    let mut simple_map: Vec<Vec<bool>> = Vec::new();

    for row in map.map.iter() {
        let mut simple_row: Vec<bool> = Vec::new();
        for cell in row.iter() {
            simple_row.push(*cell != Dir::Ground);
        }
        simple_map.push(simple_row);
    }

    let mut map_str: String = String::new();

    for row in simple_map.iter() {
        for cell in row.iter() {
            map_str.push(if *cell { 'X' } else { ' ' });
        }
        map_str.push('\n');
    }

    info!("Simplified map:\n{}", map_str);

    0
}

#[test]
fn test() {
    setup_tracing();
    assert_eq!(4, get_answer("test.3"));
    assert_eq!(4, get_answer("test.4"));
    assert_eq!(8, get_answer("test.5"));
    assert_eq!(10, get_answer("test.6"));
}
