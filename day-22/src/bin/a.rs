use tracing_subscriber::{filter, prelude::*};
use std::{fs::File, sync::Arc};
use tracing::{info, debug, warn};
use std::io::{BufRead, BufReader};

fn setup_tracing() {
    let stdout_log = tracing_subscriber::fmt::layer()
        .pretty();
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
    info!("{:?}", get_answer("input"));
}

const SECRET_MODULO: usize = 16777216;

fn next_secret(prev: usize) -> usize {
    let mut secret: usize;
    secret = ((prev * 64) ^ prev) % SECRET_MODULO;
    secret = ((secret / 32) ^ secret) % SECRET_MODULO;
    ((secret * 2048) ^ secret) % SECRET_MODULO
}

fn get_answer(file: &str) -> usize {
    let input: Vec<usize> = BufReader::new(File::open(file).unwrap()).lines()
        .filter_map(Result::ok)
        .map(|line| line.parse().unwrap())
        .collect();

    let mut result = 0usize;

    for entry in input {
        let mut secret = entry;
        for _ in 0..2000 {
            secret = next_secret(secret);
        }
        result += secret;
    }

    result
}

#[test]
fn test() {
    setup_tracing();
    assert_eq!(37327623, get_answer("test.a"));
}
