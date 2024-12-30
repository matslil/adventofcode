use tracing_subscriber::{filter, prelude::*};
use std::{fs::File, sync::Arc};
use tracing::{info, debug};
use std::io::{BufRead, BufReader};
use rust_tools::grid2d::Grid2D;
use std::collections::HashMap;
use std::collections::VecDeque;

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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct State {
    dist: usize,
    cheats_left: usize,
    pos: (usize, usize),
}

fn shortest_path(map_arg: &Grid2D<MapItem>, start: (usize, usize), cheats_left: usize, max_time: usize) -> usize {
    let mut map = map_arg.clone();
    let mut queue: VecDeque<State> = VecDeque::new();
    let mut nr_cheats = 0usize;

    queue.push_back(State{
        dist: 0,
        cheats_left,
        pos: start,
    });

    while let Some(state) = queue.pop_front() {
        debug!("Looking at: {} {:?}, queue length: {}", map[state.pos], state, queue.len());
        if state.cheats_left == 0 {
            debug!("\n{}", map);
            if let Some(dist_left) = shortest_path_len(&map, state.pos) {
                let dist = state.dist + dist_left;
                if dist <= max_time {
                    nr_cheats += 1;
                }
            }
            continue;
        }

        for next in map.successors_with(state.pos, |_, next_pos|
            (state.cheats_left > 0 || map[next_pos] != MapItem::Wall) && map[next_pos] != MapItem::Start && (state.pos.0 == next_pos.0 || state.pos.1 == next_pos.1)
        ) {
            let new_state = State {
                dist: state.dist + 1,
                cheats_left: if map[next] == MapItem::Wall || state.cheats_left == 1 { state.cheats_left - 1 } else { state.cheats_left },
                pos: next,
            };
            debug!("Queued: {:?}", queue.back());
            if map[next] != MapItem::End && state.cheats_left != 1 {
                map[next] = MapItem::Start;
            }
            queue.push_back(new_state);
        }
    }

    debug!("Final map: \n{}", map);
    nr_cheats
}

fn shortest_path_len(map_arg: &Grid2D<MapItem>, start: (usize, usize)) -> Option<usize> {
    let mut map = map_arg.clone();
    let mut queue: VecDeque<State> = VecDeque::new();

    queue.push_back(State{
        dist: 0,
        cheats_left: 0,
        pos: start,
    });

    while let Some(state) = queue.pop_front() {
        if map[state.pos] == MapItem::End {
            debug!("shortest_path_len = {}", state.dist);
            return Some(state.dist);
        }

        for next in map.successors_with(state.pos, |_, next_pos|
            map[next_pos] != MapItem::Wall && map[next_pos] != MapItem::Start && (state.pos.0 == next_pos.0 || state.pos.1 == next_pos.1)
        ) {
            queue.push_back(State {
                dist: state.dist + 1,
                cheats_left: 0,
                pos: next,
            });
            if map[next] != MapItem::End {
                map[next] = MapItem::Start;
            }
        }
    }

    None
}

// fn shortest_path(map_arg: &Grid2D<MapItem>, start: (usize, usize), cheat_at_round_arg: isize) -> (usize, Option<Vec<(usize, usize)>>) {
//     let mut map = map_arg.clone();
//     let mut queue: Vec<(usize, (usize, usize))> = vec![(0, start)];
//     let mut cheat_at_round = cheat_at_round_arg;
//     let mut cheat1: Option<(usize, usize)> = None;
//     let mut cheat: Option<Vec<(usize, usize)>> = None;
// 
//     loop {
//         let mut queue_next: Vec<(usize, (usize, usize))> = Vec::new();
// 
//         for (dist, pos) in queue {
//             let do_cheat: bool;
//             if cheat_at_round == 0 {
//                 cheat1 = pos;
//                 do_cheat = true;
//             } else if cheat_at_round == -1 {
//                 if map[cheat1] == MapItem::Wall || map[pos] == MapItem::Wall {
//                     cheat = Some([cheat1, pos]);
//                     do_cheat = true;
//                 } else {
//                     do_cheat = false;
//                 }
//             } else {
//                 do_cheat = false;
//             }
//             if map[pos] == MapItem::End {
//                 if cheat.is_none() {
//                     debug!("{}: No cheat made", cheat_at_round_arg);
//                 } else {
//                     debug!("{}: Did cheat", cheat_at_round_arg);
//                 }
//                 return (dist, cheat);
//             }
//             for successor in map.successors_with(pos, |_, p|
//                 (do_cheat || map[p] != MapItem::Wall) && map[p] != MapItem::Start && (p.0 == pos.0 || p.1 == pos.1)
//             ) {
//                 if map[successor] != MapItem::End {
//                     map[successor] = MapItem::Start;
//                 }
//                 queue_next.push((dist+1, successor));
//             }
//         }
// 
//         if queue_next.len() == 0 {
//             info!("\n{}", map);
//             panic!("No progress!");
//         }
// 
//         queue = queue_next;
//         cheat_at_round -= 1;
//     }
// }

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
    let baseline = shortest_path_len(map, start.clone()).unwrap();
    shortest_path(map, start, 2, baseline - cheat_min_save)
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
