use tracing::{self, info};
use tracing_subscriber::{filter, prelude::*};
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::sync::Arc;
use std::fmt;
use std::cmp;
use std::ops;
use itertools::Itertools;

#[derive(Clone, Debug)]
struct RangeMap {
    dst_start: isize,
    src_start: isize,
    len: isize,
}

impl fmt::Display for RangeMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}..{} -> {}..{}",
            self.src_start, self.src_start + self.len,
            self.dst_start, self.dst_start + self.len
            )
    }
}

impl RangeMap {
    fn new(line: &str) -> Self {
        info!("RangeMap::new({})", line);
        let parts: Vec<isize> = line.split_whitespace().map(|e| e.parse::<isize>().unwrap()).collect();
        let result = Self {
            dst_start: parts[0],
            src_start: parts[1],
            len: parts[2]
        };
        info!("{}", result);
        result
    }

    fn overlaps_with(&self, other: &RangeMap) -> bool {
        let result = ! ((self.dst_start > (other.src_start + other.len)) ||
            ((self.dst_start + self.len) < other.src_start));
        info!("{} overlaps_with({}) -> {}", self, other, result);
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

impl cmp::Ord for RangeMap {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.dst_start.cmp(&other.dst_start)
    }
}

impl cmp::PartialOrd for RangeMap {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::PartialEq for RangeMap {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == cmp::Ordering::Equal
    }
}

impl cmp::Eq for RangeMap {}

struct RangeMaps {
    name: String,
    ranges: Vec<RangeMap>,
}

impl fmt::Display for RangeMaps {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {}\n", self.name)?;
        for range in &self.ranges {
            range.fmt(f)?;
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl RangeMaps {
    fn new(lines: &mut impl Iterator<Item = String>) -> Option<Self> {
        let mut ranges: Vec<RangeMap> = Vec::new();
        let name_line = lines.next()?;
        let name = name_line.split(":").next().unwrap();
        for line in lines.map(|e| e.trim().to_string()) {
            if line.len() == 0 {
                break;
            }
        ranges.push(RangeMap::new(&line));
        }
        let mut result = Self {
            name: name.to_string(),
            ranges: ranges,
        };
        result.ranges.sort();
//        let mut start: isize = 0;
//        let mut gap_fillers: Vec<RangeMap> = Vec::new();
//        for range in &result.ranges {
//            if range.src_start > start {
//                gap_fillers.push(RangeMap {
//                    src_start: start,
//                    dst_start: start,
//                    len: range.src_start - start,
//                });
//                start = range.src_start;
//            }
//        }
//        result.ranges.append(&mut gap_fillers);
//        result.ranges.sort();
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

fn parse_seeds(lines: &mut impl Iterator<Item=String>) -> Vec<ops::Range<isize>> {
    let first_line = lines.next().unwrap();
    let parts: Vec<&str> = first_line.split(":").map(|e| e.trim()).collect();
    let mut result: Vec<ops::Range<isize>> = Vec::new();
    let mut iter = parts[1].split_whitespace().map(|e| e.parse::<isize>().unwrap());
    while let Some(start) = iter.next() {
        let len: isize = iter.next().unwrap();
        result.push(ops::Range{ start: start, end: start + len, });
    }
    result
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

    let mut map: Vec<RangeMaps> = Vec::new();

    loop {
        let ranges = RangeMaps::new(&mut iter);
        if ranges.is_none() {
            break;
        }
        let ranges = ranges.unwrap();
//        info!("{}", ranges);
        map.push(ranges);
    }

    let mut min_result: isize = isize::MAX;

    for seed_range in seeds {
        info!("Seed range: {:?}", seed_range);
        for seed in seed_range {
            let mut result = seed;
            for ranges in &map {
                let new_result = ranges.translate(result);
                //            info!("{} translates {} -> {}", ranges, result, new_result);
                result = new_result;
            }
            if result < min_result {
                min_result = result;
            }
        }
    }
    min_result
}

#[test]
fn test() {
    setup_tracing();
    assert_eq!(46, get_answer("test"));
}
