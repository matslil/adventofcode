use tracing::{self, info};
use tracing_subscriber::{filter, prelude::*};
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::sync::Arc;
use std::fmt;
use std::cmp;

#[derive(Clone, Debug)]
struct Range {
    dst_start: isize,
    src_start: isize,
    len: isize,
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}..{} -> {}..{}",
            self.src_start, self.src_start + self.len,
            self.dst_start, self.dst_start + self.len
            )
    }
}

impl Range {
    fn new(line: &str) -> Self {
        info!("Range::new({})", line);
        let parts: Vec<isize> = line.split_whitespace().map(|e| e.parse::<isize>().unwrap()).collect();
        let result = Self {
            dst_start: parts[0],
            src_start: parts[1],
            len: parts[2]
        };
        info!("{}", result);
        result
    }

    fn contains(&self, value: isize) -> bool {
        if value >= self.src_start && value < (self.src_start + self.len) {
            true
        } else {
            false
        }
    }
}

impl cmp::Ord for Range {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.dst_start.cmp(&other.dst_start)
    }
}

impl cmp::PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::PartialEq for Range {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == cmp::Ordering::Equal
    }
}

impl cmp::Eq for Range {}

struct Ranges {
    name: String,
    ranges: Vec<Range>,
}

impl fmt::Display for Ranges {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {}\n", self.name)?;
        for range in &self.ranges {
            range.fmt(f)?;
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Ranges {
    fn new(lines: &mut impl Iterator<Item = String>) -> Option<Self> {
        let mut ranges: Vec<Range> = Vec::new();
        let name_line = lines.next()?;
        let name = name_line.split(":").next().unwrap();
        for line in lines.map(|e| e.trim().to_string()) {
            if line.len() == 0 {
                break;
            }
        ranges.push(Range::new(&line));
        }
        let mut result = Self {
            name: name.to_string(),
            ranges: ranges,
        };
        result.ranges.sort();
        Some(result)
    }

    fn translate(&self, input: isize) -> isize {
        for range in &self.ranges {
            if range.contains(input) {
                return input + (range.dst_start - range.src_start)
            }
        }
        input
    }
}

fn parse_seeds(lines: &mut impl Iterator<Item=String>) -> Vec<isize> {
    let first_line = lines.next().unwrap();
    let parts: Vec<&str> = first_line.split(":").map(|e| e.trim()).collect();
    let seeds: Vec<isize> = parts[1].split_whitespace().map(|e| e.parse::<isize>().unwrap()).collect();
    seeds
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

fn get_answer(file: &str) -> isize {
    let mut _sum: isize = 0;
    let mut iter = BufReader::new(File::open(file).unwrap()).lines().map(|e| e.unwrap());

    let seeds = parse_seeds(&mut iter);
    info!("{:?}", seeds);
    let _ = iter.next();

    let mut map: Vec<Ranges> = Vec::new();

    loop {
        let ranges = Ranges::new(&mut iter);
        if ranges.is_none() {
            break;
        }
        let ranges = ranges.unwrap();
//        info!("{}", ranges);
        map.push(ranges);
    }

    let mut results: Vec<(isize, isize)> = Vec::new();

    for seed in seeds {
        let mut result = seed;
        for ranges in &map {
            let new_result = ranges.translate(result);
//            info!("{} translates {} -> {}", ranges, result, new_result);
            result = new_result;
        }
        let end_result = result;
        info!("Seed {} -> Location {}", seed, end_result);
        results.push((seed, end_result));
    }

    let mut min_result: isize = isize::MAX;
    for result in results {
        if result.1 < min_result {
            min_result = result.1;
        }
    }
    min_result
}

#[test]
fn test() {
    setup_tracing();
    assert_eq!(35, get_answer("test"));
}
