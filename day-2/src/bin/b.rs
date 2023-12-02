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
    fn set(&mut self, color: &str, amount: usize) {
        if !self.colors.contains_key(color) {
            panic!("{}: Color not valid", color);
        }
        *self.colors.get_mut(color).unwrap() = amount;
    }

    fn min(&mut self, mins: &Colors) {
        for min_color in mins.colors.keys() {
            if self.colors[min_color] <= mins.colors[min_color] {
                *self.colors.get_mut(min_color).unwrap() = mins.colors[min_color];
            }
        }
    }

    fn power(&self) -> usize {
        self.colors["red"] * self.colors["green"] * self.colors["blue"]
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
    println!("{:?}", get_answer("input"));
}

fn get_answer(file: &str) -> usize {
    let mut games: Vec<usize> = Vec::new();
    for line in BufReader::new(File::open(file).unwrap()).lines().map(|x| x.unwrap()) {
        games.push(parse_line(&line));
    }
    games.into_iter().sum()
}

fn parse_line(line: &String) -> usize {
    info!("{}", line);
    let game: Vec<&str> = line.split(":").collect();
    let mut min_colors: Colors = Colors::default();
    for set in game[1].split(";") {
        let mut colors: Colors = Colors::default();
        for cubes in set.split(",").map(|e| e.trim()) {
            let parts: Vec<&str> = cubes.split(" ").collect();
            let count: usize = parts[0].parse::<usize>().unwrap();
            let color: &str = parts[1];
            colors.set(color, count);
        }
        min_colors.min(&colors);
    }
    info!("Min set {:?}", min_colors);
    min_colors.power()
}

#[test]
fn test() {
    setup_tracing();
    assert_eq!(2286, get_answer("test.a"));
}
