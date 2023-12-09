use tracing::{self, info};
use tracing_subscriber::{filter, prelude::*};
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::sync::Arc;
use std::collections::HashMap;
use text_io::scan;

fn parse_graph_line(line: &str) -> (String, String, String) {
    let from: String;
    let left: String;
    let right: String;
    scan!(line.bytes() => "{} = ({}, {})", from, left, right);
    (from, left, right)
}

#[derive(Clone, Copy, Debug)]
enum Dir {
    Left,
    Right,
}

fn parse_instruction(line: &str) -> Vec<Dir> {
    let mut dir: Vec<Dir> = Vec::new();

    for ch in line.chars() {
        match ch {
            'L' => dir.push(Dir::Left),
            'R' => dir.push(Dir::Right),
            _   => panic!("{}: Unknown direction", ch),
        }
    }
    dir
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
    let dirs = parse_instruction(&iter.next().unwrap());
    let _ = iter.next().unwrap();

    let mut map: HashMap<String, (String, String)> = HashMap::new();
    for line in iter {
        let result = parse_graph_line(&line);
        map.insert(result.0, (result.1, result.2));
    }

    let mut nr_steps: usize = 0;
    let mut node: &String = &"AAA".to_string();
    for (step, dir) in dirs.into_iter().cycle().enumerate() {
        if node == "ZZZ" {
            nr_steps = step;
            break;
        }
        let (left, right) = &map[node];
        node = match dir {
            Dir::Left => &left,
            Dir::Right => &right,
        }
    }

    nr_steps
}

#[test]
fn test() {
    setup_tracing();
    assert_eq!(2, get_answer("test.1"));
    assert_eq!(6, get_answer("test.2"));
}
