// 5688 too low
// 6865 too low
// 8387 too high

use tracing::{self, info};
use tracing_subscriber::{filter, prelude::*};
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::sync::Arc;
use std::collections::VecDeque;

fn setup_tracing() {
    let stdout_log = tracing_subscriber::fmt::layer()
        .pretty();

    // A layer that logs events to a file.
    let file = File::create("debug.log");
    let file = match file  {Ok(file) => file,Err(error) => panic!("Error: {:?}",error),};
    let debug_log = tracing_subscriber::fmt::layer()
        .with_writer(Arc::new(file))
        .with_ansi(false)
        .without_time()
        .with_file(true)
        .with_line_number(true);

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
    println!("{:?}", get_answer("input"));
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum PatternEntry {
    Invalid,
    Valid,
    Unknown,
}

impl std::fmt::Display for PatternEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match self {
            PatternEntry::Invalid => '.',
            PatternEntry::Valid => '#',
            PatternEntry::Unknown => '?',
        })
    }
}

impl std::convert::From<char> for PatternEntry {
    fn from(value: char) -> Self {
        match value {
            '.' => PatternEntry::Invalid,
            '#' => PatternEntry::Valid,
            '?' => PatternEntry::Unknown,
            c   => panic!("{}: Unknown char", c),
        }
    }
}

#[derive(Clone)]
struct Pattern {
    pattern: Vec<PatternEntry>,
}

impl std::fmt::Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for entry in &self.pattern {
            write!(f, "{}", entry)?;
        }
        Ok(())
    }
}

impl Pattern {
    fn fits(&self, nr_digits: usize) -> bool {
        if nr_digits <= self.pattern.len() {
            self.pattern
                .iter()
                .take(nr_digits)
                .fold(true, |acc, &e| acc && e != PatternEntry::Invalid)
        } else {
            false
        }
    }

    fn contains_invalid(&self) -> bool {
        let result = self.pattern.iter().fold(false, |acc, &e| acc || e == PatternEntry::Invalid);
        result
    }

    fn valids_or_unknowns_left(&self) -> bool {
        self.pattern.iter().fold(false, |acc, &e| acc || (e != PatternEntry::Invalid))
    }

    fn valids_left(&self) -> bool {
        self.pattern.iter().fold(false, |acc, &e| acc || (e == PatternEntry::Valid))
    }

    fn len(&self) -> usize {
        self.pattern.len()
    }

    fn slice(&self, from: usize) -> Option<Self> {
        if self.len() > from {
            Some(Pattern::from_iter(self.pattern.iter()
                    .skip(from)
                    .map(|e| *e))
            )
        } else {
            None
        }
    }

    fn slice_from(&self, from: usize) -> Option<Self> {
        if from == 0 || self.len() < (from + 1) {
            return None
        }
        if self.pattern[from] == PatternEntry::Valid {
            return None
        }
        let new_pattern = Pattern::from_iter(self.pattern.iter()
            .skip(from + 1)
            .skip_while(|&e| *e == PatternEntry::Invalid)
            .map(|e| *e));
        if !new_pattern.pattern.is_empty() {
            Some(new_pattern)
        } else {
            None
        }
    }
}

impl std::convert::From<&str> for Pattern {
    fn from(value: &str) -> Self {
        Self { pattern: value
            .chars()
            .map(|c| PatternEntry::from(c))
            .collect::<Vec<PatternEntry>>()
        }
    }
}

impl std::convert::From<&[PatternEntry]> for Pattern {
    fn from(value: &[PatternEntry]) -> Self {
        Self { pattern: value.to_vec() }
    }
}

impl std::iter::FromIterator<PatternEntry> for Pattern {
    fn from_iter<I: IntoIterator<Item=PatternEntry>>(iter: I) -> Self {
        let mut pattern = Vec::new();
        for item in iter {
            pattern.push(item);
        }
        Self { pattern: pattern }
    }
}

fn translate(level: usize, pattern: Pattern, in_values: VecDeque<usize>) -> usize {
    let mut values = in_values.clone();

    info!("{0:1$}translate({2} ({3}), {4:?})", "",
        level*4, pattern, pattern.pattern.len(), values);

    if values.len() == 0 {
        if !pattern.valids_or_unknowns_left() {
            info!("{0:1$}translate() -> 1", "", level*4);
            return 1;
        } else {
            info!("{0:1$}translate() -> 0", "", level*4);
            return 0;
        }
    }

    let value = values.pop_front().unwrap();

    let stop_at: usize = pattern.pattern.len().saturating_sub(values.iter().sum::<usize>() + values.len().saturating_sub(1));
    info!("{0:1$}stop_at: {2}", "", level*4, stop_at);
    if stop_at == 0 {
        info!("{0:1$}translate() -> 0", "", level*4);
        return 0;
    }

    let mut count = 0usize;
    for (nr, trial) in pattern.pattern[..stop_at].windows(value).map(|e| Pattern::from(e)).enumerate() {
        info!("{0:1$}translate() matching {2} against {3}", "", level*4, trial, value);
        if trial.fits(value) {
            info!("{0:1$}Fits!", "", level*4);
            if values.len() == 0 {
                if let Some(slice) = pattern.slice(nr + value) {
                    if slice.valids_left() {
                        info!("{0:1$}Valids left in slice: {2}", "", level*4, slice);
                        continue;
                    }
                }
                info!("{0:1$}Match!", "", level*4);
//                count += 1;
                continue;
            }
            if let Some(slice) = pattern.slice_from(nr + value) {
                count += translate(level + 1, slice, values.clone());
            }
        }
    }

    info!("{0:1$}translate() -> {2}", "", level*4, count);
    if values.len() == 0 {
        1
    } else {
        count
    }
}

fn get_answer(file: &str) -> usize {
    setup_tracing();
    let mut count = 0usize;

    for line in BufReader::new(File::open(file).unwrap())
        .lines()
        .map(|e| e.unwrap()) {
            let parts:Vec<&str> = line.as_str()
                .split_whitespace()
                .collect();
            let values: VecDeque<usize> = parts[1]
                .split(",")
                .map(|v| v
                    .parse::<usize>()
                    .unwrap())
                .collect::<Vec<usize>>().into();
            let pattern = Pattern::from(parts[0]);
            let add = translate(1, pattern.clone(), values.clone());
            info!("==== '{}' {:?} -> {} ====", pattern, values, add);
            count += add;
        }
    count
}

#[test]
fn test() {
    assert_eq!(21, get_answer("test"));
}
