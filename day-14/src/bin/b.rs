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
    let max_y = slopes.iter().map(|pos| cmp::max(pos.0.0, pos.1.0)).max().unwrap() + 1;
    let max_x = slopes.iter().map(|pos| cmp::max(pos.0.1, pos.1.1)).max().unwrap();
    let min_y = 0;
    let min_x = slopes.iter().map(|pos| cmp::min(pos.0.1, pos.1.1)).min().unwrap() - 200;
    let y_pan = 0;
    let height = max_y - min_y + 1 + y_pan;
    let width = height;
    let scale = (min_y, min_x);
    let sand_pos = (0, 500 - scale.1);
    println!("Max x: {}\nMax y: {}\nMin x:{}\nMin y:{}", max_x, max_y, min_x, min_y);
    println!("Scale: {:?}", scale);

    let mut board = Board::new(height, width + 400, Dir::Air);

    println!("Board height x width: {} x {}", board.rows, board.columns);
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
    loop {
        let pos = add_sand(&sand_pos, &board);
        if pos == sand_pos {
            break;
        }
        count += 1;
        *board.get_mut(pos).unwrap() = Dir::Sand;
//        if (count % 100) == 0 {
//            println!("{}", count);
//            print_board_ascii(&board);
//        }
    }
    print_board_ascii(&board);
    count + 1
}

fn add_sand(start_pos: &Pos, board: &Board) -> Pos {
    let mut pos = start_pos.clone();
    loop {
        let mut found = false;
        for attempt in [(pos.0+1, pos.1), (pos.0+1, pos.1-1), (pos.0+1, pos.1+1)] {
            if cell_is_free(&attempt, board) {
                pos = attempt;
                found = true;
                break;
            }
        }
        if ! found {
            return pos;
        }
//        println!("Moving sand to {:?}", pos);
    }
}

fn cell_is_free(pos: &Pos, board: &Board) -> bool {
    if pos.0 == board.rows {
        return false;
    }
    *board.get(*pos).unwrap() == Dir::Air
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
        assert_eq!(get_answer("test"), 93);
    }
}

