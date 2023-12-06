use tracing::{self, info, instrument};
use tracing_subscriber::{filter, prelude::*};
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::sync::Arc;
use std::fmt;
use std::cmp;
use itertools::Itertools;

#[derive(Clone, Debug)]
struct Range {
    dst_start: usize,
    src_start: usize,
    len: usize,
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
        let parts: Vec<usize> = line.split_whitespace().map(|e| e.parse::<usize>().unwrap()).collect();
        let result = Self {
            dst_start: parts[0],
            src_start: parts[1],
            len: parts[2]
        };
        info!("{}", result);
        result
    }

    fn overlaps_with(&self, other: &Range) -> bool {
        let result = ! ((self.dst_start > (other.dst_start + other.len)) ||
            ((self.dst_start + self.len) < other.dst_start));
        info!("{} overlaps_with({}) -> {}", self, other, result);
        result
    }

    fn merge_with(&self, other: &Range) -> Self {
        let dst_start = cmp::min(self.dst_start, other.dst_start);
        let src_start = cmp::min(self.src_start, other.src_start);
        let dst_end = cmp::max(self.dst_start + self.len, other.dst_start + other.len);
        let len = dst_end - dst_start;

        info!("{} => {}..{} -> {}..{}",
            other,
            src_start, src_start + len,
            dst_start, dst_start + len
        );

        Self {
            dst_start: dst_start,
            src_start: src_start,
            len: len,
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
    fn new(lines: &mut impl Iterator<Item = String>) -> Self {
        let mut ranges: Vec<Range> = Vec::new();
        let name_line = lines.next().unwrap();
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
        result.merge_overlaps();
        result.ranges.sort();
        result
    }

    fn remove(&mut self, range: &Range) {
        let search_for = range.clone();
        self.ranges.remove(self.ranges.iter().position(|e| *e == search_for).unwrap());
    }

    // Search through list of ranges, returns index of first two
    // ranges found to overlap
    fn first_overlap(&self) -> Option<(Range, Range)> {
        for pair in self.ranges.iter().combinations(2) {
            let range1: Range = pair[0].clone();
            let range2: Range = pair[1].clone();
            if range1.overlaps_with(&range2) {
                return Some((range1, range2));
            }
        }
        None
    }

    fn merge_overlaps(&mut self) {
        loop {
            match self.first_overlap() {
                Some((range1, range2)) => {
                    let new_range = range1.merge_with(&range2);
                    self.remove(&range1.clone());
                    self.remove(&range2.clone());
                    self.ranges.push(new_range);
                }
                None => return,
            }
        }
    }

    fn len(&self) -> usize {
        self.ranges.len()
    }
}

fn parse_seeds(lines: &mut impl Iterator<Item=String>) -> Vec<usize> {
    let first_line = lines.next().unwrap();
    let parts: Vec<&str> = first_line.split(":").map(|e| e.trim()).collect();
    let seeds: Vec<usize> = parts[1].split_whitespace().map(|e| e.parse::<usize>().unwrap()).collect();
    seeds
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
    let mut _sum: usize = 0;
    let mut iter = BufReader::new(File::open(file).unwrap()).lines().map(|e| e.unwrap());

    let seeds = parse_seeds(&mut iter);
    info!("{:?}", seeds);
    let _ = iter.next();

    loop {
        let ranges = Ranges::new(&mut iter);
        info!("{}", ranges);
        if ranges.len() == 0 {
            break;
        }
    }
    0
}

#[test]
fn test() {
    setup_tracing();
    assert_eq!(13, get_answer("test"));
}
