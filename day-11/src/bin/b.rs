use tracing_subscriber::{filter, prelude::*};
use std::{fs::File, sync::Arc};
use tracing::{info, debug};
use std::io::{BufRead, BufReader};

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
    info!("{:?}", get_answer("input", 75).len());
}

fn blink(input: Vec<usize>) -> Vec<usize>
{
    let mut result = Vec::new();

    for stone in input {
        if stone == 0 {
            result.push(1);
        } else {
            let stone_str = stone.to_string();
            if stone_str.len() % 2 == 0 {
                result.push(stone_str[0..stone_str.len()/2].parse().unwrap());
                result.push(stone_str[stone_str.len()/2..].parse().unwrap());
            } else {
                result.push(stone * 2024);
            }
        }
    }
    result
}

fn print_stones(input: &Vec<usize>)
{
    let str = input.iter().map(|value| value.to_string()).collect::<Vec<String>>().join(" ");
    debug!("{}", str);
}

fn get_answer(file: &str, nr_blinks: u32) -> Vec<usize> {
    let mut input: Vec<usize> = BufReader::new(File::open(file).unwrap()).lines()
        .filter_map(Result::ok)
        .flat_map(|line| line.split_whitespace()
            .into_iter()
            .map(|nr_str| nr_str.parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
        )
        .collect();
    for iter in 0..nr_blinks {
        info!("{}: {}", iter, input.len());
//        print_stones(&input);
        input = blink(input);
    }
    return input;
}

#[test]
fn test() {
    setup_tracing();
    debug!("{:?}", get_answer("test.0", 75).len());
//    assert_eq!(vec![1, 2024, 1, 0, 9, 9, 2021976], get_answer("test.a.1", 1));
//    assert_eq!(vec![2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6, 0, 3, 2], get_answer("test.a.2", 6));
}
