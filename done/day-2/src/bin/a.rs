use tracing::{self, info};
use tracing_subscriber::{filter, prelude::*};
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::sync::Arc;
use std::collections::HashMap;

#[derive(Debug)]
struct Colors {
    colors: HashMap<&'static str, usize>,
}

impl Default for Colors {
    fn default() -> Self {
        Self { colors: HashMap::from([
            ("red", 0),
            ("green", 0),
            ("blue", 0),
        ])}
    }
}

impl Colors {
    fn new(red: usize, green: usize, blue: usize) -> Self {
        Self { colors: HashMap::from([
            ("red", red),
            ("green", green),
            ("blue", blue),
        ])}
    }

    fn set(&mut self, color: &str, amount: usize) {
        if !self.colors.contains_key(color) {
            panic!("{}: Color not valid", color);
        }
        *self.colors.get_mut(color).unwrap() = amount;
    }

    fn within_limits(&self, limits: &Colors) -> bool {
        for limit_color in limits.colors.keys() {
            if self.colors[limit_color] > limits.colors[limit_color] {
                return false;
            }
        }
        true
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
    println!("{:?}", get_answer("input", &Colors::new(12, 13, 14)));
}

fn get_answer(file: &str, limits: &Colors) -> usize {
    let mut games: Vec<usize> = Vec::new();
    for line in BufReader::new(File::open(file).unwrap()).lines().map(|x| x.unwrap()) {
        if let Some(game_nr) = parse_line(&line, limits) {
            games.push(game_nr);
        }
    }
    games.into_iter().sum()
}

fn parse_line(line: &String, limits: &Colors) -> Option<usize> {
    info!("{}", line);
    let game: Vec<&str> = line.split(":").collect();
    let game_parts: Vec<&str> = game[0].split(" ").collect();
    let game_nr: usize = game_parts[1].parse::<usize>().unwrap();
    for set in game[1].split(";") {
        let mut colors: Colors = Colors::default();
        for cubes in set.split(",").map(|e| e.trim()) {
            info!("cubes: {}", cubes);
            let parts: Vec<&str> = cubes.split(" ").collect();
            let count: usize = parts[0].parse::<usize>().unwrap();
            let color: &str = parts[1];
            colors.set(color, count);
        }
        if !colors.within_limits(limits) {
            return None;
        }
    }
    info!("Adding game {}", game_nr);
    Some(game_nr)
}

#[test]
fn test() {
    setup_tracing();
    assert_eq!(8, get_answer("test.a", &Colors::new(12, 13, 14)));
}
