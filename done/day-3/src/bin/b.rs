// 87901961 - too high
// 75847567

use tracing::{self, info};
use tracing_subscriber::{filter, prelude::*};
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::sync::Arc;
use std::fmt;
use std::cmp;

#[derive(PartialEq, Eq, Clone)]
enum EntryType {
    Empty,
    Number,
    Symbol,
    Gear,
}

impl fmt::Display for EntryType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            EntryType::Empty => ".",
            EntryType::Number => "1",
            EntryType::Symbol => "-",
            EntryType::Gear => "*",
        })
    }
}

struct Number {
    value: usize,
    nr_digits: usize,
    x: usize,
    y: usize,
}

impl fmt::Debug for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({}, {}) {}\n", self.value, self.x, self.y, self.nr_digits)
    }
}

impl cmp::Ord for Number {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        if self.y > other.y {
            return cmp::Ordering::Greater;
        }
        if self.y < other.y {
            return cmp::Ordering::Less;
        }

        if self.x > (other.x + other.nr_digits) {
            return cmp::Ordering::Greater;
        }

        if (self.x + self.nr_digits) < other.x {
            return cmp::Ordering::Less;
        }

        cmp::Ordering::Equal
    }
}

impl cmp::PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == cmp::Ordering::Equal
    }
}

impl cmp::Eq for Number {}

impl Number {
    fn is_at(&self, x: usize, y: usize) -> bool {
        x >= self.x && x <= (self.x + self.nr_digits - 1) && y == self.y
    }
}

struct Map {
    values: Vec<EntryType>,
    columns: usize,
    rows: usize,
    numbers: Vec<Number>,
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}x{})\n", self.columns, self.rows)?;
        for (idx, value) in self.values.iter().enumerate() {
            let x = idx % self.columns;
            let y = idx / self.columns;
            if x == 0 {
                write!(f, "{:>4} ", y)?;
            }
            write!(f, "{}", value)?;
            if x == self.columns-1 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

impl Map {
    fn new<T>(file: T) -> Self
        where
            T: BufRead
    {
        let mut values: Vec<EntryType> = Vec::new();
        let mut columns: usize = 0;
        let mut rows: usize = 0;
        let mut numbers: Vec<Number> = Vec::new();

        for (y, line) in file.lines().enumerate() {
            if (y + 1) > rows {
                rows = y + 1;
            }
            let mut number_string = String::new();
            let mut number_x: usize = 0;
            let mut number_y: usize = 0;
            for (x, ch) in line.unwrap().chars().enumerate() {
                if (x + 1) > columns {
                    columns = x + 1;
                }
                if number_string.is_empty() {
                    if ch.is_digit(10) {
                        number_string.push(ch);
                        number_x = x;
                        number_y = y;
                    }
                } else {
                    if ch.is_digit(10) {
                        number_string.push(ch);
                    } else {
                        numbers.push(Number {
                            value: number_string.parse().unwrap(),
                            nr_digits: number_string.len(),
                            x: number_x,
                            y: number_y,
                        });
                        number_string.clear();
                        number_x = 0;
                        number_y = 0;
                    }
                }
                values.push(if ch == '.' {
                    EntryType::Empty
                } else if ch.is_digit(10) {
                    EntryType::Number
                } else if ch == '*' {
                    EntryType::Gear
                } else {
                    EntryType::Symbol
                });
            }
            if ! number_string.is_empty() {
                numbers.push(Number {
                    value: number_string.parse().unwrap(),
                    nr_digits: number_string.len(),
                    x: number_x,
                    y: number_y,
                });
                number_string.clear();
            }
        }
        Self {
            values: values,
            columns: columns,
            rows: rows,
            numbers: numbers,
        }
    }

    fn gear_ratio(&self, x: usize, y:usize) -> Option<usize> {
        assert!(self.columns > x);
        assert!(self.rows > y);
        let mut nrs: Vec<&Number> = Vec::new();

        let start_x = if x > 0 {
            x - 1
        } else {
            x
        };
        let stop_x = if x < (self.columns - 1) {
            x + 1
        } else {
            x
        };
        assert!(start_x <= stop_x);
        let start_y = if y > 0 {
            y - 1
        } else {
            y
        };
        let stop_y = if y < (self.rows - 1) {
            y + 1
        } else {
            y
        };
        assert!(start_y <= stop_y);
        info!("Gear at ({}, {}): x {}..{}, y {}..{}",
        x, y, start_x, stop_x, start_y, stop_y);
        for search_x in start_x ..= stop_x {
            for search_y in start_y ..= stop_y {
                if let Some(nr) = self.number_at(search_x, search_y) {
                    nrs.push(nr);
                    info!("Gear at ({}, {}): Adjacent number {} found at ({}, {})",
                    nrs.len(), x, y, search_x, search_y);
                }
            }
        }

        nrs.sort();
        nrs.dedup();
        if nrs.len() == 2 {
            let ratio = nrs[0].value * nrs[1].value;
            info!("Gear at ({}, {}): Ratio {} * {} = {}",
            x, y, nrs[0].value, nrs[1].value, ratio);
            Some(ratio)
        } else {
            None
        }
    }

    fn number_at(&self, x: usize, y: usize) -> Option<&Number> {
        for number in &self.numbers {
            if number.is_at(x, y) {
                return Some(&number);
            }
        }
        None
    }

    fn idx_to_pos(&self, idx: usize) -> (usize, usize) {
        (idx % self.columns, idx / self.columns)
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
    let map = Map::new(BufReader::new(File::open(file).unwrap()));
    let mut sum: usize = 0;

    info!("{}", map);
    info!("{:?}", map.numbers);
    for (idx, symbol) in map.values.iter().enumerate() {
        if symbol == &EntryType::Gear {
            let pair = map.idx_to_pos(idx);
            if let Some(n) = map.gear_ratio(pair.0, pair.1) {
                sum += n;
            }
        }
    }
    sum
}

#[test]
fn test() {
    setup_tracing();
    assert_eq!(467835, get_answer("test"));
}
