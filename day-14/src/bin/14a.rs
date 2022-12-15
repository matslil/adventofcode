use std::fs::File;
use std::io::{BufReader, BufRead};
use std::env;
use bresenham::Bresenham;
use pathfinding::matrix::Matrix;
use image::{RgbImage, Rgb};

type PosType = usize;
type Pos = (PosType, PosType);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Dir {
    Down,
    Left,
    Right,
    Flat,
    Air,
    Sand
}

type Board = Matrix<Dir>;

fn main() {
    let mut slopes: Vec<Vec<Pos>> = Vec::new();
    for line in BufReader::new(File::open(env::args().nth(1).expect("Could not read file")).unwrap()).lines().map(|x| x.unwrap()) {
        slopes.push(parse_lines(&line));
    }
    let max_x = slopes.iter().flatten().map(|pos| pos.0).max().unwrap();
    let max_y = slopes.iter().flatten().map(|pos| pos.1).max().unwrap();
    let min_x = slopes.iter().flatten().map(|pos| pos.0).min().unwrap();
    let min_y = slopes.iter().flatten().map(|pos| pos.1).min().unwrap();
    let scale = (min_x, min_y);
    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;
    println!("Max x: {}\nMax y: {}\nMin x:{}\nMin y:{}", max_x, max_y, min_x, min_y);

    let mut board = Board::new(width, height, Dir::Air);

    for slope in slopes {
        let mut prev_coord : Option<Pos> = None;
        for coord in slope {
            if let Some(prev) = prev_coord {
                let rescaled_prev = rescale(&prev, &scale);
                let rescaled_coord = rescale(&coord, &scale);
                let iprev = (rescaled_prev.0 as isize, rescaled_prev.1 as isize);
                let icoord = (rescaled_coord.0 as isize, rescaled_prev.1 as isize);
                let dir = if prev.0 == coord.0 {
                    // Vertical line
                    Dir::Down
                } else if prev.1 == coord.1 {
                    // Horizontal
                    Dir::Flat
                } else if prev.0 > coord.0 {
                    // Slope to the right
                    Dir::Right
                } else {
                    // Slope to the left
                    Dir::Left
                };

                for pos_signed in Bresenham::new(iprev, icoord) {
                    let pos = (pos_signed.0 as usize, pos_signed.1 as usize);
//                    if let Some(dir) = board.get(pos) {
//                        if *dir != Dir::Air {
//                            panic!("{:?}: {:?}: No air!", pos, board.get(pos));
//                        }
//                    }
                    if let Some(entry) = board.get_mut(pos) {
                        *entry = Dir::Down;
                    }
                }
            } else {
                prev_coord = Some(coord);
            }
        }
    }
    print_board_png(&board, "board.png");
}

fn rescale(pos: &Pos, scale: &Pos) -> Pos {
    (pos.0 - scale.0, pos.1 - scale.1)
}

fn print_board_png(board: &Board, file_name: &str) {
    const WALL: Rgb<u8> = Rgb([128, 128, 128]);
    const SAND: Rgb<u8> = Rgb([128, 128, 0]);
    const AIR : Rgb<u8> = Rgb([0, 0,0 ]);

    let mut img = RgbImage::new(
        board.rows.try_into().unwrap(),
        board.columns.try_into().unwrap());

    for (x, row) in board.iter().enumerate() {
        for (y, cell) in row.iter().enumerate() {
            img.put_pixel(x.try_into().unwrap(), y.try_into().unwrap(), match cell {
                Dir::Down | Dir::Left | Dir::Right | Dir::Flat => WALL,
                Dir::Sand => SAND,
                Dir::Air => AIR,
            });
        }
    }
    img.save(file_name).unwrap();
}

fn print_board_ascii(board: &Board) {
    for row in board {
        for cell in row {
            print!("{}", match cell {
                Dir::Down => "V",
                Dir::Left => "<",
                Dir::Right => ">",
                Dir::Flat => "=",
                Dir::Air => ".",
                Dir::Sand => "o",
            });
        }
    }
}

fn parse_lines(input: &str) -> Vec<Pos> {
    let mut coord = Vec::<Pos>::new();
    let coord_words: Vec<&str> = input.split(" -> ").collect();
    for coord_word in coord_words {
        let pos_strings: Vec<&str> = coord_word.split(",").collect();
        coord.push((pos_strings[0].parse::<PosType>().unwrap(), pos_strings[1].parse::<PosType>().unwrap()));
    }
    coord
}
