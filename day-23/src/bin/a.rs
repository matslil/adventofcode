use std::fs::File;
use std::io::{BufReader, BufRead};
use pathfinding::matrix::Matrix;
use std::iter;

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
        println!("---- {} ----", _round - 1);
        print_board(&board);
        let mut move_proposals: Vec<((usize, usize),(usize, usize))> = Vec::new();
        for pos in board.items().filter(|item| item.1 == &true).map(|item| item.0).collect::<Vec<_>>() {
            // If no one is next to the elf, do nothing
            if board.neighbours(pos, true).map(|pos| board.get(pos).unwrap()).fold(false, |acc, &occupied| if acc || occupied { true } else { false }) {
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
                            has_neighbours = true;
                            break;
                        }
                    }
                }
                if ! has_neighbours {
                    let delta = match dir {
                        Dir::N(item) | Dir::S(item) | Dir::W(item) | Dir::E(item) => item[0]
                    };
                    move_proposals.push(((pos),((pos.0 as isize + delta.0) as usize, (pos.1 as isize + delta.1) as usize)));
                    break;
                }
            }
        }
        let mut move_proposals_unduplicated = move_proposals.clone();
        for proposal in &move_proposals {
            if move_proposals_unduplicated.contains(proposal) {
                move_proposals_unduplicated.retain(|item| item != proposal);
            }
        }
        for proposal in move_proposals {
            let from = proposal.0;
            let to = proposal.1;
            *board.get_mut(from).unwrap() = false;
            *board.get_mut(to).unwrap() = true;
        }
        start_dir.next().unwrap();
    }

    print_board(&board);
    board.values().fold(0, |acc, occupied| if ! occupied { acc + 1 } else { acc }) - (board.rows * 2) - (board.columns * 2)
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
    fn test_input() {
        const INPUT_FILE: &str = "test";
        const ANSWER: usize = 110;

        assert_eq!(get_answer(INPUT_FILE), ANSWER);
    }
}
