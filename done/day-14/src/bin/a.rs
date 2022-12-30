use std::fs::File;
use std::io::{BufReader, BufRead};
use bresenham::Bresenham;
use pathfinding::matrix::Matrix;
use std::cmp;

type PosType = usize;
type Pos = (PosType, PosType);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Dir {
    Wall,
    Air,
    Sand
}

type Board = Matrix<Dir>;

fn main() {
    println!("{}", get_answer("input"));
}

fn get_answer(file: &str) -> usize {
    let mut slopes = Vec::<(Pos, Pos)>::new();
    for line in BufReader::new(File::open(file).unwrap()).lines().map(|x| x.unwrap()) {
        slopes.append(&mut parse_lines(&line));
    }
    let max_y = slopes.iter().map(|pos| cmp::max(pos.0.0, pos.1.0)).max().unwrap();
    let max_x = slopes.iter().map(|pos| cmp::max(pos.0.1, pos.1.1)).max().unwrap();
    let min_y = slopes.iter().map(|pos| cmp::min(pos.0.0, pos.1.0)).min().unwrap();
    let min_x = slopes.iter().map(|pos| cmp::min(pos.0.1, pos.1.1)).min().unwrap();
    let width = max_x - min_x + 1;
    let y_pan = width / 2;
    let height = max_y - min_y + 1 + y_pan;
    let scale = (min_y, min_x);
    let sand_pos = (0, 500 - scale.1);
    println!("Max x: {}\nMax y: {}\nMin x:{}\nMin y:{}", max_x, max_y, min_x, min_y);

    let mut board = Board::new(height, width, Dir::Air);

    for slope in slopes {
        let (from, to) = rescale(&slope, &scale, y_pan);
        println!("---- New slope ----");
        let dir = Dir::Wall;

        println!("{:?} -> {:?}", from, to);
        for pos_signed in Bresenham::new((from.0 as isize, from.1 as isize), (to.0 as isize, to.1 as isize)) {
            print!("{:?} ", pos_signed);
            let pos = (pos_signed.0 as usize, pos_signed.1 as usize);
            let entry = board.get_mut(pos).unwrap();
            *entry = dir;
        }
        *board.get_mut(to).unwrap() = dir;
        println!("");
    }

    let mut count = 0;
    while let Some(pos) = add_sand(&sand_pos, &board) {
        count += 1;
        *board.get_mut(pos).unwrap() = Dir::Sand;
        print_board_ascii(&board);
    }
    print_board_ascii(&board);
    println!("{}", count);
    count
}

fn add_sand(start_pos: &Pos, board: &Board) -> Option<Pos> {
    let mut pos = start_pos.clone();
    loop {
        let mut found = false;
        for attempt_wrapped in [
            if pos.0 < board.rows { Some((pos.0+1, pos.1)) } else { None },
            if pos.1 > 0 { Some((pos.0+1, pos.1-1)) } else { None },
            if pos.1 < board.columns { Some((pos.0+1, pos.1+1)) } else { None }]
        {
            if let Some(attempt) = attempt_wrapped {
                if let Some(is_clear) = cell_is_free(&attempt, board) {
                    if is_clear {
                        pos = attempt;
                        found = true;
                        break;
                    }
                } else {
                    return None;
                }
            } else {
                return None;
            }
        }
        if ! found {
            return Some(pos);
        }
        println!("Moving sand to {:?}", pos);
    }
}

fn cell_is_free(pos: &Pos, board: &Board) -> Option<bool> {
    if let Some(dir) = board.get(*pos) {
        println!("{:?} is {:?}", pos, dir);
        Some(*dir == Dir::Air)
    } else {
        None
    }
}

fn rescale(pos: &(Pos, Pos), scale: &Pos, y_pan: usize) -> (Pos, Pos) {
    ((pos.0.0 - scale.0 + y_pan, pos.0.1 - scale.1), (pos.1.0 - scale.0 + y_pan, pos.1.1 - scale.1))
}

fn print_board_ascii(board: &Board) {
    println!("");
    for row in board {
        for cell in row {
            print!("{}", match cell {
                Dir::Wall => "#",
                Dir::Air => ".",
                Dir::Sand => "o",
            });
        }
        println!("");
    }
}

fn parse_lines(input: &str) -> Vec<(Pos, Pos)> {
    let mut coord = Vec::new();
    let coord_words: Vec<&str> = input.split(" -> ").collect();
    let mut prev_pos: Option<Pos> = None;
    let mut curr_pos: Pos;
    for coord_word in coord_words {
        let pos_strings: Vec<&str> = coord_word.split(",").collect();
        curr_pos = (pos_strings[1].parse::<PosType>().unwrap(), pos_strings[0].parse::<PosType>().unwrap());
        if prev_pos != None {
            coord.push((prev_pos.unwrap(), curr_pos));
        }
        prev_pos = Some(curr_pos);
    }
    coord
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(get_answer("test"), 24);
    }
}

