// 215 too low
// 13979 too high
// 1151 too high
// 484
use tracing::{self, info};
use tracing_subscriber::{filter, prelude::*};
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::sync::Arc;
use pathfinding::prelude::Grid;

#[derive(PartialEq, Clone, Copy, Debug)]
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

        let dir_prev = Direction::from_pos(&start, &nodes[nodes.len()-1]);
        let dir_next = Direction::from_pos(&start, &nodes[1]);

        self[start] = [dir_prev, dir_next].into();

        info!("{}", self);

        nodes
    }

    fn color_map(&self, pipe: &Vec<(usize, usize)>) -> Vec<Vec<usize>> {
        let max_x = self.map[0].len() - 1;
        let max_y = self.map.len() - 1;
        let mut color: Vec<Vec<usize>> = Vec::new();

        let mut a_row: Vec<usize> = Vec::new();
        a_row.resize(max_x + 1, 0);
        for _ in 0..=max_y {
            color.push(a_row.clone());
        }

        for (node, next) in pipe.windows(2).map(|e| (e[0], e[1])) {
            info!("{:?} -> {:?}", node, next);
            color[node.1][node.0] = 2;
            match Direction::from_pos(&node, &next) {
                Direction::North => {
                    if node.0 > 0 && self[(node.0-1, node.1)] == Dir::Ground {
                        color[node.1][node.0-1] = 0;
                    }
                    if node.0 < max_x && self[(node.0+1, node.1)] == Dir::Ground {
                        color[node.1][node.0+1] = 1;
                    }
                }
                Direction::East => {
                    if node.1 > 0 && self[(node.0, node.1-1)] == Dir::Ground {
                        color[node.1-1][node.0] = 0;
                    }
                    if node.1 < max_y && self[(node.0, node.1+1)] == Dir::Ground {
                        color[node.1+1][node.0] = 1;
                    }
                }
                Direction::South => {
                    if node.0 > 0 && self[(node.0-1, node.1)] == Dir::Ground {
                        color[node.1][node.0-1] = 1;
                    }
                    if node.0 < max_x && self[(node.0+1, node.1)] == Dir::Ground {
                        color[node.1][node.0+1] = 0;
                    }
                }
                Direction::West => {
                    if node.1 > 0 && self[(node.0, node.1-1)] == Dir::Ground {
                        color[node.1-1][node.0] = 1;
                    }
                    if node.1 < max_y && self[(node.0, node.1+1)] == Dir::Ground {
                        color[node.1+1][node.0] = 0;
                    }
                }
            }
        }

        let mut string: String = String::new();
        for (y, row) in color.iter().enumerate() {
            string.push_str(&format!("\n{:3}-", y));
            for cell in row.iter() {
                string.push(if *cell == 1 { '#' } else if *cell == 2 { '+' } else { '.' });
            }
        }

        info!("{}", string);

        color
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
    println!("{:?}", get_answer("input"));
}

fn successors(color: &Vec<Vec<usize>>, at: &(usize, usize)) -> Vec<(usize, usize)> {
    let max_x = color[0].len() - 1;
    let max_y = color.len() - 1;
    let mut list: Vec<(usize, usize)> = Vec::new();

    if at.0 > 0 {
        list.push((at.0 -1, at.1));
    }
    if at.0 < max_x {
        list.push((at.0 + 1, at.1));
    }
    if at.1 > 0 {
        list.push((at.0, at.1 - 1));
    }
    if at.1 < max_y {
        list.push((at.0, at.1 + 1));
    }

    info!("list: {:?}", list);

    list.into_iter().filter(|pos| color[pos.1][pos.0] == 0usize).collect::<Vec<(usize, usize)>>()
}

fn fill(color: &Vec<Vec<usize>>) -> usize {
    let mut visited: Vec<(usize, usize)> = Vec::new();

    let mut node = (75, 68);
    let mut stack: Vec<(usize, usize)> = Vec::new();
    loop {
        let mut add = successors(color, &node);
        info!("successors: {:?}", add);
        if add.len() == 0 {
            break;
        }
        for prospect in &add {
            if ! visited.contains(prospect) {
                stack.push(*prospect);
                visited.push(*prospect);
            }
        }
        if let Some(n) = stack.pop() {
            node = n;
        } else {
            break;
        }
    }

    info!("Fill: {}", visited.len());
    visited.len()
}

fn get_answer(file: &str) -> usize {
    let mut map = Map::new(file);
    info!("{}", map);
    let visited = map.clean();

    let color = map.color_map(&visited);
    let mut grid = Grid::new(color[0].len(), color.len());

    let nr_cells = color.iter().flatten().fold(0usize, |acc, &next| { if next == 1usize { acc + 1 } else { acc }});

    for (y, row) in color.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell > 0 {
                grid.add_vertex((x, y));
            }
        }
    }

    info!("{:?}", grid);

    info!("nr_cells: {}", nr_cells);
    fill(&color) + nr_cells
}

#[test]
fn test() {
    setup_tracing();
    assert_eq!(4, get_answer("test.3"));
    assert_eq!(4, get_answer("test.4"));
    assert_eq!(8, get_answer("test.5"));
    assert_eq!(10, get_answer("test.6"));
}

// 180, 68 - Good starting point?

