use tracing_subscriber::{filter, prelude::*};
use std::{fs::File, sync::Arc};
use tracing::{info, debug};
use std::io::{BufRead, BufReader};
use std::iter::zip;

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

fn next_secret(prev: usize) -> (usize, isize) {
    let mut secret: usize;
    secret = ((prev * 64) ^ prev) % SECRET_MODULO;
    secret = ((secret / 32) ^ secret) % SECRET_MODULO;
    secret = ((secret * 2048) ^ secret) % SECRET_MODULO;
    (secret, (prev % 10) as isize - (secret % 10) as isize)
}

fn equal_changes(lhs: &[(usize, isize)], rhs: &[(usize, isize)]) -> bool {
    let result = zip(lhs.iter(), rhs.iter()).fold(true, | acc, v | acc && v.0.1 == v.1.1);
    if result {
        debug!("Match!");
    }
    result
}

fn get_answer(file: &str) -> usize {
    let input: Vec<usize> = BufReader::new(File::open(file).unwrap()).lines()
        .filter_map(Result::ok)
        .map(|line| line.parse().unwrap())
        .collect();

    let mut result = 0usize;

    let mut secrets: Vec<Vec<(usize, isize)>> = Vec::new();

    for entry in input {
        let mut secret = entry;
        let mut secrets_next: Vec<(usize, isize)> = Vec::new();
        for _ in 0..2000 {
            let next = next_secret(secret);
            secret = next.0;
            secrets_next.push(next);
        }
        secrets.push(secrets_next);
    }

    for cmp_with in secrets[0].windows(4) {
        let mut prices: Vec<Option<usize>> = Vec::new();
        for idx in 1..secrets.len() {
            prices.push(secrets[idx].windows(4).filter(|&v| equal_changes(v, cmp_with)).map(|v| v[3].0 % 10).max());
        }
        let total = prices.iter().map(|v| v.unwrap_or(0)).sum();
        debug!("{:?} => {}", cmp_with, total);
        if total > result {
            result = total;
        }
    }

    result
}

#[test]
fn test() {
    setup_tracing();
    assert_eq!(23, get_answer("test.b"));
}
