use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::VecDeque;

type Pos = (usize, usize);

enum Facing {
    Right = 0_usize,
    Down = 1_usize,
    Left = 2_usize,
    Up = 3_usize,
}

type Neighbors = [Pos; 4];

enum BoardEntry {
    Void,
    Empty(Neighbors),
    Wall,
}

type Board = HashMap<Pos, BoardEntry>;

fn main() {
    const INPUT: &str = "input";
    println!("{}", get_answer(INPUT));
}

fn get_answer(file: &str) -> isize {
    let line_iter = BufReader::new(File::open(file).unwrap()).lines().map(|x| x.unwrap());


}

fn get_board(iter: &mut usize) -> Board {
    let board = Board::new();
    let board_chars = Vec::new();

    for line in line_iter {
        if line.trim() == "" {
            break;
        }
        board_chars = board_chars.push(line.chars().collect());
    }

    let max_x_idx = board_chars[0].len() - 1;
    let max_y_idx = board_chars.len() - 1;

    for (x_idx, row) in &board_chars {
        for (y_idx, ch) in row {
            match ch {
                " " => BoardEntry::Void,
                "." => {
                    let mut entry: BoardEntry([0, 0, 0, 0]);
                    if x_idx == 0 {
                        x_idx = max_x_idx;
                    }
                    if y_idx == 0 {
                        y_idx = max_y_idx;
                    }

                    for facing in Facing {
                        

                    if x == 1 {
                        BoardEntry::Empty([(x + 1, y), (line.len() - x_idx + 1

#[cfg(test)]

mod test {
    use super::*;

#[test]
    fn test_input() {
        const INPUT_FILE: &str = "test";
        const ANSWER: usize = 6032;

        assert_eq!(get_answer(INPUT_FILE), ANSWER);
    }
}
