use tracing_subscriber::{filter, prelude::*};
use std::{fs::File, sync::Arc};
use tracing::{info, debug};
use std::io::{BufRead, BufReader};
use rust_tools::grid2d::Grid2D;
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
                .with_filter(filter::LevelFilter::INFO)
                .and_then(debug_log)
        )
        .init();
}

fn main() {
    setup_tracing();
    info!("{:?}", get_answer("input", 100));
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum MapItem {
    #[default]
    Empty,
    Wall,
    Start,
    End,
}

impl std::fmt::Display for MapItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match self {
            MapItem::Empty => '.',
            MapItem::Wall => '#',
            MapItem::Start => 'S',
            MapItem::End => 'E',
        })
    }
}

fn shortest_path(map_arg: &Grid2D<MapItem>, start: (usize, usize), cheat_at_round_arg: isize) -> usize {
    let mut map = map_arg.clone();
    let mut queue: Vec<(usize, (usize, usize))> = vec![(0, start)];
    let mut cheat_at_round = cheat_at_round_arg;

    loop {
        let mut queue_next: Vec<(usize, (usize, usize))> = Vec::new();

        for (dist, pos) in queue {
            let do_cheat = (-1..=0).contains(&cheat_at_round);
            for successor in map.successors_with(pos, |_, p|
                (do_cheat || map[p] != MapItem::Wall) && map[p] != MapItem::Start && (p.0 == pos.0 || p.1 == pos.1)
            ) {
                if map[successor] == MapItem::End {
                    return dist+1;
                }
                map[successor] = MapItem::Start;
                queue_next.push((dist+1, successor));
            }
        }

        if queue_next.len() == 0 {
            info!("\n{}", map);
            panic!("No progress!");
        }

        queue = queue_next;
        cheat_at_round -= 1;
    }
}

fn nr_cheats(map: &Grid2D<MapItem>, cheat_min_save: usize) -> usize {
    let mut start = (0usize, 0usize);
    for x in 0..map.cols() {
        for y in 0..map.rows() {
            if map[(x,y)] == MapItem::Start {
                start = (x,y);
                break;
            }
        }
    }
    let baseline = shortest_path(map, start, -2);
    let mut counted_cheats = 0usize;

    info!("Baseline: {}", baseline);
    for cheat_at_round in 0..baseline - cheat_min_save {
        let cheat_run = shortest_path(map, start, cheat_at_round as isize);
        if cheat_run <= (baseline - cheat_min_save) {
            info!("Cheat at round {} => {}", cheat_at_round, cheat_run);
            counted_cheats += 1;
        }
    }

    counted_cheats
}

fn get_answer(file: &str, cheat_min_save: usize) -> usize {
    let mut input = BufReader::new(File::open(file).unwrap()).lines()
        .filter_map(Result::ok);

    let map: Grid2D<MapItem> = Grid2D::new(&mut input, HashMap::from([
            ('.', MapItem::Empty),
            ('#', MapItem::Wall),
            ('S', MapItem::Start),
            ('E', MapItem::End)
    ]));

    debug!("\n{}", map);

    nr_cheats(&map, cheat_min_save)
}

#[test]
fn test() {
    setup_tracing();
    assert_eq!(44, get_answer("test.a", 2));
}
