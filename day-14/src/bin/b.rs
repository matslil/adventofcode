use tracing_subscriber::{filter, prelude::*};
use std::{fs::File, sync::Arc};
use tracing::{info, debug};
use std::io::{BufRead, BufReader};
#[macro_use] extern crate scan_fmt;
use rust_tools::grid2d::Grid2D;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
struct Robot {
    pos: (usize, usize),
    vel: (isize, isize),
}

impl Robot {
    fn new(line: &str) -> Self{
        let (pos0, pos1, vel0, vel1)  = scan_fmt!(line, "p={},{} v={},{}", usize, usize, isize, isize).unwrap();
        Self {
            pos: (pos0, pos1),
            vel: (vel0, vel1),
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
    info!("{:?}", get_answer("input", (101, 103)));
}

fn new_pos(pos: usize, vel: isize, max_len: usize, nr_rounds: usize) -> usize
{
    let dist: isize = (vel * nr_rounds as isize) % max_len as isize;
    if pos as isize + dist < 0 {
        (max_len as isize + (pos as isize + dist)) as usize
    } else {
        (pos as isize + dist) as usize % max_len
    }
}

fn is_in(robots: &Vec<Robot>, pos: (usize, usize)) -> bool {
    for match_with in robots {
        if match_with.pos == pos {
            return true;
        }
    }
    false
}

fn is_mirror(robots: &Vec<Robot>, tiles: &(usize, usize)) -> bool {
    for robot in robots {
        if robot.pos.0 < tiles.0/2 {
            if !is_in(robots, (tiles.0 - robot.pos.0 - 1, robot.pos.1)) {
                return false;
            }
        }
    }
    true
}

fn get_answer(file: &str, tiles: (usize, usize)) -> usize {
    let mut robots: Vec<Robot> = BufReader::new(File::open(file).unwrap()).lines()
        .filter_map(Result::ok)
        .fold(Vec::new(), |mut v, entry| { v.push(Robot::new(&entry)); v } );

    debug!("Size: {:?}", tiles);

    for round in 1.. {
//    let mut map: Grid2D<usize> = Grid2D::default();
//    map.set_size::<usize>(tiles);
        for robot in &mut robots {
            robot.pos.0 = new_pos(robot.pos.0, robot.vel.0, tiles.0, round);
            robot.pos.1 = new_pos(robot.pos.1, robot.vel.1, tiles.1, round);
//            map[robot.pos] += 1;
        }
        if is_mirror(&robots, &tiles) {
//        debug!("Map: \n{}", map);
            return round;
        }
        if round % 1000 == 0 {
            info!("Round: {}", round);
        }
    }
    0
}

#[test]
fn test() {
    setup_tracing();
    assert_eq!(12, get_answer("test.a", (11, 7)));
}
