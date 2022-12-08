use std::fs::File;
use std::io::{self, BufRead};
use std::env;
use std::path::Path;
use std::convert::TryInto;

const DIMENSION: usize = 99;

fn main() {
    let mut visible = [[0u8; DIMENSION]; DIMENSION];
    let mut input = [[0i8; DIMENSION]; DIMENSION];
    let args: Vec<String> = env::args().collect();

    for row in 0..DIMENSION {
        for column in 0..DIMENSION {
            visible[column][row] = 0;
        }
    }

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(&args[1]) {
        // Consumes the iterator, returns an (Optional) String
        let mut column = 0;
        for line in lines {
            if let Ok(entry) = line {
                let chars: Vec<i8> = entry.chars().map(|x| x.to_digit(10).unwrap() as i8).collect();
                let char_array = TryInto::<[i8; DIMENSION]>::try_into(chars);
                input[column] = char_array.unwrap();
                column += 1;
            }
        }
    }

    for x in 0..DIMENSION {
        let mut start: i8 = -1;
        for y in 0..DIMENSION {
            if input[x][y] > start {
                visible[x][y] = 1;
                start = input[x][y];
            }
        }
    }

    for x in 0..DIMENSION {
        let mut start: i8 = -1;
        for y in (0..DIMENSION).rev() {
            if input[x][y] > start {
                visible[x][y] = 1;
                start = input[x][y];
            }
        }
    }

    for x in 0..DIMENSION {
        let mut start: i8 = -1;
        for y in 0..DIMENSION {
            if input[y][x] > start {
                visible[y][x] = 1;
                start = input[y][x];
            }
        }
    }

    for x in 0..DIMENSION {
        let mut start: i8 = -1;
        for y in (0..DIMENSION).rev() {
            if input[y][x] > start {
                visible[y][x] = 1;
                start = input[y][x];
            }
        }
    }

    println!("");

    for (_x, column) in visible.iter().enumerate() {
        println!("{:?}", column);
    }

    let mut sum: u64 = 0;
    for (x, column) in visible.iter().enumerate() {
        for (y, _row) in column.iter().enumerate() {
            sum += visible[x][y] as u64;
        }
    }
    println!("{}", sum);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

