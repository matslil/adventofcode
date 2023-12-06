use tracing::{self, info};
use tracing_subscriber::{filter, prelude::*};
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::sync::Arc;
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
    let mut nr_cards: usize = 0;
    let mut multipliers: HashMap<usize, usize> = HashMap::new();
    for line in BufReader::new(File::open(file).unwrap()).lines().map(|e| e.unwrap()) {
        nr_cards += 1;
        let (card_nr, nr_wins) = parse_line(&line);
        if nr_wins == 0 {
            continue;
        }
        let nr_cards: usize = *multipliers.get(&card_nr).unwrap_or(&1);
        info!("Card {}: Nr cards: {}", card_nr, nr_cards);
        for copy_card in (card_nr + 1) .. (card_nr + nr_wins + 1) {
            let mut card_multiplier: usize = *multipliers.get(&copy_card).unwrap_or(&1);
            card_multiplier += nr_cards;
            info!("Card {}: Added {} copies for card {} for a total of {} cards", card_nr, nr_cards, copy_card, card_multiplier);
            *multipliers.entry(copy_card).or_insert(card_multiplier) = card_multiplier;
        }
    }
    info!("{:?}", multipliers);
    let mut sum: usize = 0;
    for card_nr in 1 .. (nr_cards+1) {
        sum += *multipliers.get(&card_nr).unwrap_or(&1);
    }
    sum
}

fn parse_line(line: &str) -> (usize, usize) {
    info!("{}", line);
    let split_colon: Vec<&str> = line.split(":").map(|e| e.trim()).collect();
    let card_nr: usize = split_colon[0].split_whitespace().skip(1).next().unwrap().parse().unwrap();
    let split_bar: Vec<&str> = split_colon[1].split("|").map(|e| e.trim()).collect();
    let winning: Vec<usize> = split_bar[0].split_whitespace().map(|e| e.parse::<usize>().unwrap()).collect();
    let have: Vec<usize> = split_bar[1].split_whitespace().map(|e| e.parse::<usize>().unwrap()).collect();

    let mut matches: usize = 0;
    for test in &have {
        if winning.contains(test) {
            matches += 1;
        }
    }
    info!("Card {}: Nr winning numbers: {}", card_nr, matches);
    (card_nr, matches)
}

#[test]
fn test() {
    setup_tracing();
    assert_eq!(30, get_answer("test"));
}
