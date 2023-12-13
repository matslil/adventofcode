// 215 too low
// 1151 too high
// 13979 too high
// 484 nope
// 487 nope

use tracing::{self, info};
use tracing_subscriber::{filter, prelude::*};
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::sync::Arc;
use pathfinding::prelude::Grid;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {
    fn from_pos(from: &(usize, usize), to: &(usize, usize)) -> Self {
        match (to.0 as isize - from.0 as isize, to.1 as isize - from.1 as isize) {
            (0, -1) => Direction::North,
            (1, 0) => Direction::East,
            (0, 1) => Direction::South,
            (-1, 0) => Direction::West,
            _ => panic!("{:?} -> {:?}: Cannot translate to direction", from, to),
        }
    }

    fn goto(&self, from: &(usize, usize), max_x: usize, max_y: usize) -> Option<(usize, usize)> {
        match self {
            Direction::North => if from.1 > 0 {
                Some((from.0, from.1 - 1))
            } else {
                None
            }
            Direction::East => if from.0 < max_x {
                Some((from.0 + 1, from.1))
            } else {
                None
            }
            Direction::South => if from.1 < max_y {
                Some((from.0, from.1 + 1))
            } else {
                None
            }
            Direction::West => if from.0 > 0 {
                Some((from.0 - 1, from.1))
            } else {
                None
            }
        }
    }

    fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }

    fn right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
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
            Dir::Ground     => write!(f, "{}", '.'),
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
    assert!(visited.len() > 1);
    &visited[visited.len() - 2]
}

fn curr(visited: &Vec<(usize, usize)>) -> &(usize, usize) {
    assert!(visited.len() > 0);
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

    fn width(&self) -> usize {
        self.map[0].len()
    }

    fn height(&self) -> usize {
        self.map.len()
    }

    fn max_x(&self) -> usize {
        self.width() - 1
    }

    fn max_y(&self) -> usize {
        self.height() -1
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
//             info!("{:?} -> {:?} {}", prev(&nodes), curr(&nodes), self[*prev(&nodes)]);
        }

        for (y, row) in self.map.iter_mut().enumerate() {
            for (x, cell) in row.iter_mut().enumerate() {
                if ! nodes.contains(&(x, y)) {
                    *cell = Dir::Ground;
                }
            }
        }

//        info!("Visited: {:?}", nodes);

        let dir_prev = Direction::from_pos(&start, &nodes[nodes.len()-1]);
        let dir_next = Direction::from_pos(&start, &nodes[1]);

        self[start] = [dir_prev, dir_next].into();

        info!("{}", self);

        nodes
    }

    fn start_fill(&self, pipe: &Vec<(usize, usize)>, right: bool) -> Vec<(usize, usize)> {
        let max_x = self.max_x();
        let max_y = self.max_y();
        let mut start_fill: Vec<(usize, usize)> = Vec::new();

        for (node, next) in pipe.windows(2).map(|e| (e[0], e[1])) {
            let dir;
            if right {
                dir = Direction::from_pos(&node, &next).right();
            } else {
                dir = Direction::from_pos(&node, &next).left();
            }
//            info!("{:?} -> {:?}", node, next);
            if let Some(fill_node) = dir.goto(&node, max_x, max_y) {
                if self[fill_node] == Dir::Ground {
                    start_fill.push(fill_node);
                }
            }
        }

//        info!("{:?}", start_fill);

        start_fill.into_iter().filter(|pos| self[*pos] == Dir::Ground).collect::<Vec<(usize, usize)>>()
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
    println!("{:?}", get_answer("input", true));
}

fn successors(map: &Map, at: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut list: Vec<(usize, usize)> = Vec::new();

    if let Some(node) = Direction::North.goto(at, map.max_x(), map.max_y()) {
        list.push(node);
    }
    if let Some(node) = Direction::South.goto(at, map.max_x(), map.max_y()) {
        list.push(node);
    }
    if let Some(node) = Direction::East.goto(at, map.max_x(), map.max_y()) {
        list.push(node);
    }
    if let Some(node) = Direction::West.goto(at, map.max_x(), map.max_y()) {
        list.push(node);
    }

    info!("list: {:?}", list);

    list.into_iter().filter(|pos| map[*pos] == Dir::Ground).collect::<Vec<(usize, usize)>>()
}

fn fill(map: &Map, start_fill: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut visited: Vec<(usize, usize)> = start_fill.clone();

    info!("start_fill: {:?}", start_fill);
    let mut stack: Vec<(usize, usize)> = start_fill;
    while let Some(node) = stack.pop() {
        info!("node: {:?}", node);
        let mut add = successors(map, &node);
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
    let visited = map.clean();

    let mut start_fill = map.start_fill(&visited, right);
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

#[test]
fn test() {
    setup_tracing();
    assert_eq!(4, get_answer("test.3", true));
    assert_eq!(4, get_answer("test.4", true));
    assert_eq!(8, get_answer("test.5", false));
    assert_eq!(10, get_answer("test.6", true));
}

// 180, 68 - Good starting point?

