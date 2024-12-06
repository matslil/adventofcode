use tracing_subscriber::{filter, prelude::*};
use std::fs::read_to_string;
use std::{fs::File, sync::Arc};
use tracing::{info, debug};
use std::collections::HashMap;
use rust_tools::grid2d::Grid2D;
use itertools::Itertools;

#[derive(PartialEq, Eq, Default, Copy, Clone)]
enum Guard {
    #[default]
    FacingUp,
    FacingRight,
    FacingDown,
    FacingLeft,
}

impl std::fmt::Display for Guard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::FacingUp => '^',
            Self::FacingRight => '>',
            Self::FacingDown => 'v',
            Self::FacingLeft => '<',
        })
    }
}

impl Guard {
    fn turn_right(&self) -> Self {
        match self {
            Self::FacingUp => Self::FacingRight,
            Self::FacingRight => Self::FacingDown,
            Self::FacingDown => Self::FacingLeft,
            Self::FacingLeft => Self::FacingUp,
        }
    }
}

#[derive(PartialEq, Eq, Default, Copy, Clone)]
enum MapItem {
    #[default]
    Empty,
    Obstacle,
    WalkingGuard(Guard),
}

impl std::fmt::Display for MapItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "."),
            Self::Obstacle => write!(f, "#"),
            Self::WalkingGuard(g) => write!(f, "{}", g),
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
    info!("{:?}", get_answer("input"));
}

fn walk(guard_pos: &mut (usize, usize), dir: &mut Guard, steps: &mut Vec<(usize, usize)>, map: &Grid2D<MapItem>) -> bool {
    loop {
        let mut next_guard_pos: (usize, usize) = *guard_pos;
        match dir {
            Guard::FacingUp => if guard_pos.1 == 0 { return false; } else { next_guard_pos.1 -= 1; },
            Guard::FacingRight => if guard_pos.0 == map.cols()-1 { return false; } else { next_guard_pos.0 += 1; },
            Guard::FacingDown => if guard_pos.1 == map.rows()-1 { return false; } else { next_guard_pos.1 += 1; },
            Guard::FacingLeft => if guard_pos.0 == 0 { return false; } else { next_guard_pos.0 -= 1; },
        }
        if map[next_guard_pos] == MapItem::Obstacle {
            return true;
        }
        *guard_pos = next_guard_pos;
        steps.push(*guard_pos);
    }
}

fn is_looping(start_guard_pos: &(usize, usize), start_dir: &Guard, map: &Grid2D<MapItem>) -> bool {
    let mut guard_pos: (usize, usize) = *start_guard_pos;
    let mut dir: Guard = *start_dir;

//    debug!("Start loop check: {:?} {} {}\n{}", guard_pos, dir, map[guard_pos], map);
    let mut steps: Vec<((usize, usize),Guard)> = Vec::new();

    loop {
//        debug!("{}", map[guard_pos]);
        let mut next_guard_pos: (usize, usize) = guard_pos;
        match dir {
            Guard::FacingUp => if guard_pos.1 == 0 { return false; } else { next_guard_pos.1 -= 1; },
            Guard::FacingRight => if guard_pos.0 == map.cols()-1 { return false; } else { next_guard_pos.0 += 1; },
            Guard::FacingDown => if guard_pos.1 == map.rows()-1 { return false; } else { next_guard_pos.1 += 1; },
            Guard::FacingLeft => if guard_pos.0 == 0 { return false; } else { next_guard_pos.0 -= 1; },
        }
        if map[next_guard_pos] == MapItem::Obstacle {
            let mut draw_map = map.clone();
            draw_map[guard_pos] = MapItem::WalkingGuard(dir);
            dir = dir.turn_right();
        } else {
            if steps.contains(&(next_guard_pos, dir)) {
//                debug!("Loop detected!");
                return true;
            }
            guard_pos = next_guard_pos;
            steps.push((guard_pos, dir));
        }
    }
}


fn get_answer(file: &str) -> usize {
    let input = read_to_string(file).unwrap();

    let mut map = Grid2D::new(&mut input.lines(), HashMap::from([
            ('.', MapItem::Empty),
            ('#', MapItem::Obstacle),
            ('^', MapItem::WalkingGuard(Guard::FacingUp)),
            ('>', MapItem::WalkingGuard(Guard::FacingRight)),
            ('v', MapItem::WalkingGuard(Guard::FacingDown)),
            ('<', MapItem::WalkingGuard(Guard::FacingLeft)),
    ]));

    let mut guard_pos = (0usize, 0usize);
    let mut guard = Guard::default();

    for x in 0..map.cols() {
        for y in 0..map.rows() {
            if let MapItem::WalkingGuard(g) = &map[(x, y)] {
                guard_pos = (x, y);
                guard = g.clone();
                map[guard_pos] = MapItem::Empty;
            }
        }
    }

    let start_guard_pos = guard_pos;
    let start_guard = guard;

    debug!("Initial map:\n{}", map);
    debug!("Guard: ({},{}) {}", guard_pos.0, guard_pos.1, guard);

    let mut steps: Vec<(usize, usize)> = Vec::new();

    while walk(&mut guard_pos, &mut guard, &mut steps, &map) {
        guard = guard.turn_right();
    }

    let mut result = 0usize;
    let start_size = steps.len();
    let mut curr_size = 0usize;

    for trial in steps.into_iter().unique() {
        curr_size += 1;
        debug!("Trial: {:?} {}%", trial, (curr_size * 100)/start_size);
        let mut trial_map = map.clone();
        trial_map[trial] = MapItem::Obstacle;
        if is_looping(&start_guard_pos, &start_guard, &trial_map) {
            result += 1;
        }
    }
    result
}

#[test]
fn test() {
    setup_tracing();
    assert_eq!(6, get_answer("test.a"));
}
