use std::fs::File;
use std::io::{self, BufRead};
use std::env;
use std::path::Path;
use std::convert::TryInto;

const DIMENSION: usize = 99;

fn main() {
    let mut input = [[0i8; DIMENSION]; DIMENSION];
    let args: Vec<String> = env::args().collect();

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(&args[1]) {
        // Consumes the iterator, returns an (Optional) String
        let mut column = 0;
        for line in lines {
            if let Ok(entry) = line {
                let chars: Vec<i8> = entry.chars().map(|x| x.to_digit(10).unwrap() as i8).collect();
                println!("{:?}", chars);
                let char_array = TryInto::<[i8; DIMENSION]>::try_into(chars);
                input[column] = char_array.unwrap();
                column += 1;
            }
        }
    }

    let mut highest_scenic = 0u32;
    for x in 0..DIMENSION {
        for y in 0..DIMENSION {
            let score = scenic_score(x, y, &input);
            print!("{:<3} ", score);
            if score > highest_scenic {
                highest_scenic = score;
            }
        }
        println!("");
    }

    println!("{}", highest_scenic);
}

fn scenic_score(start_x: usize, start_y: usize, input: &[[i8; DIMENSION]; DIMENSION]) -> u32 {
    let mut scores = [0u32; 4];

    let compare = input[start_x][start_y];
    let mut nr_trees = 0u32;
    if start_x > 0 {
        for x in (0..start_x).rev() {
            nr_trees += 1;
            if input[x][start_y] >= compare {
                break;
            }
        }
    }
    scores[0] = nr_trees;
    nr_trees = 0;
    if start_x < (DIMENSION-1) {
        for x in start_x+1..DIMENSION {
            nr_trees += 1;
            if input[x][start_y] >= compare {
                break;
            }
        }
    }
    scores[1] = nr_trees;
    nr_trees = 0;
    if start_y > 0 {
        for y in (0..start_y).rev() {
            nr_trees += 1;
            if input[start_x][y] >= compare {
                break;
            }
        }
    }
    scores[2] = nr_trees;
    nr_trees = 0;
    if start_y < (DIMENSION-1) {
        for y in start_y+1..DIMENSION {
            nr_trees += 1;
            if input[start_x][y] >= compare {
                break;
            }
        }
    }
    scores[3] = nr_trees;
    return scores[0] * scores[1] * scores[2] * scores[3];
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

