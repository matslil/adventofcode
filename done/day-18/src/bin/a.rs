use tracing_subscriber::{filter, prelude::*};
use std::{fs::File, sync::Arc};
use tracing::{info, debug};
use std::io::{BufRead, BufReader};
use itertools::Itertools;
use rust_tools::grid2d::Grid2D;
use std::collections::VecDeque;
use std::iter::zip;

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
    info!("{:?}", get_answer("input", (71, 71), 1024));
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum MapItem {
    #[default]
    Empty,
    Corrupted,
    Step,
}

impl std::fmt::Display for MapItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match self {
            MapItem::Empty => '.',
            MapItem::Corrupted => '#',
            MapItem::Step => 'O',
        })
    }
}

// fn in_any(lists: &Vec<VecDeque<(usize, usize)>>, lists2: &Vec<VecDeque<(usize, usize)>>, pos: &(usize, usize)) -> bool {
//     for list in lists {
//         if list.contains(pos) {
//             return true;
//         }
//     }
//     for list in lists2 {
//         if list.contains(pos) {
//             return true;
//         }
//     }
//     false
// }

fn shortest_path2(map_arg: &Grid2D<MapItem>, goal: (usize, usize)) -> usize {
    let mut map = map_arg.clone();
    let mut queue: Vec<(usize, (usize, usize))> = vec![(0, (0, 0))];

    loop {
        let mut queue_next: Vec<(usize, (usize, usize))> = Vec::new();

        for (dist, pos) in &queue {
            for successor in map.successors_with(*pos, |_, p|
                map[p] != MapItem::Corrupted && map[p] != MapItem::Step && (p.0 == pos.0 || p.1 == pos.1)
            ) {
                if successor == goal {
                    return dist+1;
                }
                map[successor] = MapItem::Step;
                queue_next.push((dist+1, successor));
            }
        }

        if queue == queue_next {
            panic!("No progress!");
        }

        queue = queue_next;
    }
}

// fn shortest_path(map: &Grid2D<MapItem>, goal: (usize, usize)) -> VecDeque<(usize,usize)> {
//     let mut paths: Vec<VecDeque<(usize, usize)>> = vec![VecDeque::from(vec![(0,0)])];
//     let mut remove_paths: Vec<VecDeque<(usize, usize)>> = Vec::new();
//     let mut count = 0usize;
// 
//     loop {
//         info!("Count: {}", count);
//         count += 1;
//         let mut shortest_path: VecDeque<(usize, usize)> = VecDeque::new();
//         let mut shortest: Option<usize> = None;
//         for path in &paths {
//             if let Some(pos) = path.front() {
//                 if *pos == goal {
//                     debug!("Path reached goal, length: {}", path.len());
//                     if shortest.is_none() || shortest.unwrap() > path.len() {
//                         shortest = Some(path.len());
//                         shortest_path = path.clone();
//                     }
//                 }
//             }
//         }
//         if shortest_path.len() > 0 {
//             return shortest_path;
//         }
//         let mut new_paths: Vec<VecDeque<(usize, usize)>> = Vec::new();
//         let mut nexts: Vec<Vec<(usize, usize)>> = Vec::new();
//         for path in &paths {
//             let curr = path.front().unwrap().clone();
//             let next = map.successors_with(curr, |_, pos|
//                 !in_any(&paths, &remove_paths, &pos) && map[pos] != MapItem::Corrupted && (pos.0 == curr.0 || pos.1 == curr.1)
//                 );
//             nexts.push(next);
//         }
//         for (path, next) in zip(&mut paths, nexts) {
//             for idx in 1..next.len() {
//                 let mut new_path = path.clone();
//                 new_path.push_front(next[idx]);
//                 new_paths.push(new_path);
//             }
//             if next.len() > 0 {
//                 path.push_front(next[0]);
//             } else {
//                 remove_paths.push(path.clone());
//             }
//         }
//         for path in &remove_paths {
//             for idx in 0..paths.len() {
//                 if path == &paths[idx] {
//                     paths.remove(idx);
//                     break;
//                 }
//             }
//         }
//         if new_paths.len() > 0 {
//             paths.append(&mut new_paths);
//         }
//     }
// }


fn get_answer(file: &str, size: (usize, usize), nr_rounds: usize) -> usize {
    let input: Vec<(usize, usize)> = BufReader::new(File::open(file).unwrap()).lines()
        .filter_map(Result::ok)
        .map(|line| line
            .split(',')
            .map(|value| value.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap()
        )
        .collect::<Vec<_>>();

    let mut map: Grid2D<MapItem> = Grid2D::default();
    map.set_size(size, MapItem::default());

    for idx in 0..nr_rounds {
        map[input[idx]] = MapItem::Corrupted;
    }

    debug!("\n{}", map);

    shortest_path2(&map, (size.0-1, size.1-1))
//     let path = shortest_path(&map, (size.0-1, size.1-1));
// 
//     for step in &path {
//         map[*step] = MapItem::Step;
//     }
// 
//     debug!("\n{}", map);
// 
//     path.len() - 1
}

#[test]
fn test() {
    setup_tracing();
    assert_eq!(22, get_answer("test.a", (7, 7), 12));
}
