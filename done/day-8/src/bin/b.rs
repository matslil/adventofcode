// use tracing::{self, info};
use tracing_subscriber::{filter, prelude::*};
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::sync::Arc;
use std::collections::HashMap;
use itertools::Itertools;
use num::integer::lcm;

type Node = (char, char, char);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Dir {
    Left,
    Right,
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
    let mut iter = BufReader::new(File::open(file).unwrap()).lines().map(|e| e.unwrap());
    let dir = iter
        .next()
        .unwrap()
        .chars()
        .map(|c| if c == 'R' { Dir::Right } else { Dir::Left })
        .collect_vec();

    let nodes = iter
        .skip(1)
        .map(|line| {
            let (src, left, right): (Node, Node, Node) = line
                .chars()
                .filter(|c| c.is_ascii_alphanumeric())
                .chunks(3)
                .into_iter()
                .map(|node| node.collect_tuple().unwrap())
                .collect_tuple()
                .unwrap();
            (src, (left, right))
        })
        .collect::<HashMap<_, _>>();

    nodes
        .iter()
        .filter(|((_, _, e), _)| *e == 'A')
        .map(|(a, _)| a)
        .map(|start| {
            let mut count: usize = 0;

            dir.iter().cycle().fold_while(nodes[&start], |node, &r| {
                count += 1;
                let key = if r == Dir::Right { node.1 } else { node.0 };

                if key.2 == 'Z' {
                    itertools::FoldWhile::Done(node)
                } else {
                    itertools::FoldWhile::Continue(nodes[&key])
                }
            });
            count
        })
        .reduce(|acc, x| lcm(acc, x))
        .unwrap()
}

#[test]
fn test() {
    setup_tracing();
    assert_eq!(6, get_answer("test.3"));
}
