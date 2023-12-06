use tracing::{self, info};
use tracing_subscriber::{filter, prelude::*};
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::sync::Arc;
use std::fmt;

#[derive(PartialEq, Eq, Clone)]
enum EntryType {
    Empty,
    Number,
    Symbol,
}

impl fmt::Display for EntryType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            EntryType::Empty => ".",
            EntryType::Number => "1",
            EntryType::Symbol => "$",
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
                number_x = 0;
                number_y = 0;
            }
        }
        Self {
            values: values,
            columns: columns,
            rows: rows,
            numbers: numbers,
        }
    }

    fn at(&self, x: usize, y: usize) -> EntryType {
        assert!(self.columns > x);
        assert!(self.rows > y);
        self.values[y * self.columns + x].clone()
    }

    fn is_adjacent_to_symbol(&self, nr: &Number) -> bool {
        assert!(self.columns > nr.x);
        assert!(self.rows > nr.y);
        assert!(self.columns > nr.nr_digits);
        let start_x = if nr.x > 0 {
            nr.x - 1
        } else {
            nr.x
        };
        let stop_x = if nr.x < (self.columns - nr.nr_digits) {
            nr.x + nr.nr_digits
        } else {
            self.columns - 1
        };
        assert!(start_x <= stop_x);
        let start_y = if nr.y > 0 {
            nr.y - 1
        } else {
            nr.y
        };
        let stop_y = if nr.y < (self.rows - 1) {
            nr.y + 1
        } else {
            nr.y
        };
        assert!(start_y <= stop_y);
        info!("Number at ({}, {}): x {}..{}, y {}..{}",
        nr.x, nr.y, start_x, stop_x, start_y, stop_y);
        for search_x in start_x ..= stop_x {
            for search_y in start_y ..= stop_y {
                if self.at(search_x, search_y) == EntryType::Symbol {
                    info!("Number at ({}, {}): Adjacent symbol found at ({}, {})",
                    nr.x, nr.y, search_x, search_y);
                    return true;
                }
            }
        }
        false
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
    for number in &map.numbers {
        if map.is_adjacent_to_symbol(number) {
            sum += number.value;
        }
    }
    sum
}

#[test]
fn test() {
    setup_tracing();
    assert_eq!(4361, get_answer("test"));
}
