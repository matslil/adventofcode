use std::fs::File;
use std::io::{BufReader, BufRead};
use pathfinding::matrix::Matrix;
use std::iter;
use std::collections::HashMap;

type Board = Matrix<bool>;

type DirType = [(isize, isize); 3];

#[derive(Debug, PartialEq, Eq)]
enum Dir {
    N(DirType),
    S(DirType),
    W(DirType),
    E(DirType),
}

const DIRS: [Dir; 4] = [Dir::N([(-1, 0), (-1, -1), (-1, 1)]), Dir::S([(1, 0), (1, -1), (1, 1)]), Dir::W([(0, -1), (-1, -1), (1, -1)]), Dir::E([(0, 1), (-1, 1), (1, 1)])];

fn main() {
    const INPUT: &str = "input";
    println!("{}", get_answer(INPUT));
}

fn get_answer(file: &str) -> usize {
    let lines = BufReader::new(File::open(file).unwrap()).lines().map(|x| x.unwrap()).collect::<Vec<_>>();
    let mut board = Board::new_empty(lines[0].len());
    let mut start_dir = DIRS.iter().cycle();

    for line in lines {
        let row = line.chars().map(|x| if x == '.' { false } else { true }).collect::<Vec<_>>();
        board.extend(&row).unwrap();
    }

    for _round in 1..=10 {
        board = board_margins(&board);
        println!("---- {} {:?} ----", _round - 1, start_dir.clone().next());
        print_board(&board);
        let mut move_proposals: HashMap<(usize, usize),Vec<(usize, usize)>> = HashMap::new();
        for pos in board.items().filter(|item| item.1 == &true).map(|item| item.0).collect::<Vec<_>>() {
            // If no one is next to the elf, do nothing
            if ! board.neighbours(pos, true).map(|pos| board.get(pos).unwrap()).fold(false, |acc, &occupied| if acc || occupied { true } else { false }) {
                println!("{:?}: Happy at current position", pos);
                continue;
            }
            let start_at = start_dir.clone();
            // Check each direction, starting at a certain point and restart from beginning when
            // reaching end of list, there are four directions to go
            for dir in start_at.take(4) {
                let mut has_neighbours = false;
                // For this direction, check in front and the diagonals if it's occupied
                let deltas = match dir {
                    Dir::N(item) | Dir::S(item) | Dir::W(item) | Dir::E(item) => item
                };
                for delta in deltas {
                    let check_pos: (usize, usize) = ((pos.0 as isize + delta.0) as usize, (pos.1 as isize + delta.1) as usize);
                    if let Some(occupied) = board.get(check_pos) {
                        if *occupied {
                            println!("{:?}: Blocked by {:?}", pos, check_pos);
                            has_neighbours = true;
                            break;
                        }
                    }
                }
                if ! has_neighbours {
                    let delta = match dir {
                        Dir::N(item) | Dir::S(item) | Dir::W(item) | Dir::E(item) => item[0]
                    };
                    let new_pos = ((pos.0 as isize + delta.0) as usize, (pos.1 as isize + delta.1) as usize);
                    println!("{:?}: proposes move to {:?}", pos, new_pos);
                    move_proposals.entry(new_pos).and_modify(|entry| entry.push(pos)).or_insert(vec![pos]);
                    break;
                }
            }
        }
        println!("Proposals: {:?}", move_proposals);
        for proposal in &move_proposals {
            if proposal.1.len() == 1 {
                let to   = *proposal.0;
                let from = proposal.1[0];
                println!("{:?}: Moving to {:?}", from, to);
                *board.get_mut(from).unwrap() = false;
                *board.get_mut(to).unwrap() = true;
            }
        }
        start_dir.next().unwrap();
    }

    print_board(&board);
    board.values().fold(0, |acc, occupied| if ! occupied { acc + 1 } else { acc }) - calculate_margins(&board)
}

fn print_board(board: &Board) {
    for row in board {
        for cell in row {
            print!("{}", if *cell { '#' } else { '.' });
        }
        println!("");
    }

    println!("");
}

fn board_margins(input_board: &Board) -> Board {
    let mut board = input_board.clone();
    let mut a_match = false;
    for x in 0..board.columns {
        if *board.get((0, x)).unwrap() {
            a_match = true;
            break;
        }
    }
    if a_match {
        board = add_row_top(&board);
    }
    a_match = false;
    for x in 0..board.columns {
        if *board.get((board.rows - 1, x)).unwrap() {
            a_match = true;
            break;
        }
    }
    if a_match {
        board = add_row_bottom(&board);
    }
    a_match = false;
    for y in 0..board.rows {
        if *board.get((y, 0)).unwrap() {
            a_match = true;
            break;
        }
    }
    if a_match {
        board = add_column_left(&board);
    }
    a_match = false;
    for y in 0..board.rows {
        if *board.get((y, board.columns - 1)).unwrap() {
            a_match = true;
            break;
        }
    }
    if a_match {
        board = add_column_right(&board);
    }
    board
}

fn calculate_margins(board: &Board) -> usize {
    let mut empty_top: isize = 0;
    let mut empty_bottom: isize = 0;
    let mut empty_left: isize = 0;
    let mut empty_right: isize = 0;

    let mut last_empty_row_from_top: isize = -2;
    let mut first_empty_row_from_bottom: isize = 0;
    let mut last_empty_column_from_left: isize = -2;
    let mut first_empty_column_from_right: isize = 0;

    for (y, row) in board.iter().enumerate() {
        if row.iter().fold(false, |acc, &entry| if acc || entry { true } else { false }) {
            if last_empty_row_from_top < 0 {
                last_empty_row_from_top = y as isize - 1;
            }
            first_empty_row_from_bottom = y as isize + 1;
        }
    }

    for x in 0..board.columns {
        let mut found = false;
        for y in 0..board.rows {
            if *board.get((y, x)).unwrap() {
                found = true;
                break;
            }
        }
        if found {
            if last_empty_column_from_left < 0 {
                last_empty_column_from_left = x as isize - 1;
            }
            first_empty_column_from_right = x as isize + 1;
        }
    }

    println!("{} {} {} {}", last_empty_row_from_top, first_empty_row_from_bottom, last_empty_column_from_left, first_empty_column_from_right);

    if last_empty_row_from_top >= 0 {
        empty_top = last_empty_row_from_top + 1;
    }
    if first_empty_row_from_bottom < board.rows as isize {
        empty_bottom = board.rows as isize - first_empty_row_from_bottom;
    }
    if last_empty_column_from_left >= 0 {
        empty_left = last_empty_column_from_left + 1;
    }
    if first_empty_column_from_right < board.columns as isize {
        empty_right = board.columns as isize - first_empty_column_from_right;
    }

    println!("Dimensions (rows x columns): {} x {}", board.rows, board.columns);
    println!("Top: {}, bottom: {}, left: {}, right: {}", empty_top, empty_bottom, empty_left, empty_right);

    let empty_rows = (empty_top + empty_bottom) as usize;
    let empty_columns = (empty_left + empty_right) as usize;
    let total = empty_rows * board.columns + empty_columns * board.rows - empty_rows * empty_columns;
    println!("{}", total);
    total
}


fn add_column_right(board: &Board) -> Board {
    let mut new_board = Board::new(board.rows, board.columns + 1, false);

    for (y, row) in board.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            *new_board.get_mut((y, x)).unwrap() = *cell;
        }
    }
    new_board
}

fn add_column_left(board: &Board) -> Board {
    let mut new_board = Board::new(board.rows, board.columns + 1, false);

    for (y, row) in board.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            *new_board.get_mut((y, x + 1)).unwrap() = *cell;
        }
    }
    new_board
}

fn add_row_bottom(board: &Board) -> Board {
    let mut new_board = board.clone();
    let row = board.iter().take(1).collect::<Vec<_>>();
    new_board.extend(row[0]).unwrap();
    new_board
}

fn add_row_top(board: &Board) -> Board {
    let mut new_board = Board::new_empty(board.columns);
    let insert_row = iter::repeat(false).take(board.columns).collect::<Vec<_>>();

    new_board.extend(&insert_row).unwrap();
    for row in board {
        new_board.extend(row).unwrap();
    }
    new_board
}

#[cfg(test)]

mod test {
    use super::*;

#[test]
    fn test_small_input() {
        const INPUT_FILE: &str = "test.small";
        const ANSWER: usize = 25;

        assert_eq!(get_answer(INPUT_FILE), ANSWER);
    }

#[test]
    fn test_input() {
        const INPUT_FILE: &str = "test";
        const ANSWER: usize = 110;

        assert_eq!(get_answer(INPUT_FILE), ANSWER);
    }
}
