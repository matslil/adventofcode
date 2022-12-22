use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
use pathfinding::prelude::bfs;
use pathfinding::matrix::Matrix;

type Height = u8;

const MAX_ELEVATION: Height = 'z' as Height - 'a' as Height;

type Pos = (usize, usize);

type Board = Matrix<Height>;

const LEN: usize = 80;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut board = Board::new(0,LEN, 0);
    let mut start_pos_vec = Vec::<Pos>::new();
    let mut end_pos: Pos = (0,0);
    let mut row = 0;
    if let Ok(lines) = read_lines(&args[1]) {
        for line in lines {
            if let Ok(entry) = line {
                let entry_trimmed = entry.trim();
                add_row_to_board(entry_trimmed, &mut board);

                for (idx, ch) in entry_trimmed.chars().enumerate() {
                    match ch {
                        'S' | 'a' => start_pos_vec.push((row, idx)),
                        'E' => end_pos = (row, idx),
                        _   => {},
                    }
                }
            }
            row += 1;
        }
    }

    for row in &board {
        for cell in row {
            print!("<{:>2}>", cell);
        }
        println!("");
    }

    println!("Start: {:?}, end: {:?}", start_pos_vec, end_pos);

    let mut path_lens = Vec::<usize>::new();
    for start_pos in start_pos_vec {
        if let Some(result) = bfs(&start_pos, |pos| successors(&pos, &board), |pos| *pos == end_pos) {
            let path_len = result.len()-1;
            println!("{:?} {}", result, path_len);
            path_lens.push(path_len);
        }
    }

    println!("{}", path_lens.iter().min().unwrap());
}

fn successors(starting_pos: &Pos, board: &Board) -> Vec<Pos> {
    let from_height = *board.get(*starting_pos).unwrap();
    print!("{:?}:{} -- ", *starting_pos, from_height);
    let result = board.neighbours(*starting_pos, false).filter(|pos| { print!("{:?}:{:?} ", pos, *board.get(*pos).unwrap()); (from_height+1) >= *board.get(*pos).unwrap()} ).collect();
    println!("{:?}", result);
    result
}

fn add_row_to_board(line: &str, board: &mut Board) {
    let chars = line.chars();
    let elevations = chars.map(|ch| match ch {
        'S' => 0,
        'E' => MAX_ELEVATION,
        'a'..='z' => ch as Height - 'a' as Height,
        _ => panic!("Unknown character!"),
    }).collect::<Vec<Height>>();
    board.extend(elevations.as_slice()).unwrap();
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
