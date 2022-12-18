use pathfinding::matrix::Matrix;
use std::fs::File;
use std::io::{BufRead, BufReader};

const NR_ROCKS: usize = 2022;
const SHAFT_WIDTH: usize = 7;

type RockType = Matrix<bool>;

#[derive(Debug, Clone)]
enum Rock {
    Minus(RockType),
    Plus(RockType),
    BackL(RockType),
    I(RockType),
    Dot(RockType),
}

type Board = Matrix<bool>;

type Pos = (usize, usize);

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum Dir {
    #[default]
    Left,
    Right,
}

fn rock_matrix(rock: &Rock) -> &RockType {
    match rock {
        Rock::Minus(entry)
        | Rock::Plus(entry)
        | Rock::BackL(entry)
        | Rock::I(entry)
        | Rock::Dot(entry) => entry,
    }
}

fn main() {
    const INPUT: &str = "input";
    println!("{}", get_answer(INPUT));
}

fn get_answer(file: &str) -> usize {
    let lines = &BufReader::new(File::open(file).unwrap())
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();
    let mut dir_iter = parse_line(&lines[0]).into_iter().cycle();
    // Note: The rocks below are up-side down
    let rocks: Vec<Rock> = vec![
        Rock::Minus(RockType::from_vec(1, 4,vec![
                true, true, true, true
            ]).unwrap()),
        Rock::Plus(
            RockType::from_vec(3, 3, vec![
                false, true , false,
                true , true , true ,
                false, true , false
            ]).unwrap()),
        Rock::BackL(
            RockType::from_vec(3, 3, vec![
                true , true , true ,
                false, false, true ,
                false, false, true ,
            ]).unwrap()),
        Rock::I(RockType::from_vec(4, 1, vec![
                true,
                true,
                true,
                true
            ]).unwrap()),
        Rock::Dot(RockType::from_vec(2, 2, vec![
                true, true,
                true, true
            ]).unwrap()),
    ];
    let mut rocks_iter = rocks.into_iter().cycle();
    let mut rock = if let Some(try_rock) = rocks_iter.next() {
        try_rock
    } else {
        panic!("Rock iterator exhausted");
    };
    let mut dir = if let Some(try_dir) = dir_iter.next() {
        try_dir
    } else {
        panic!("Dir iterator exhausted");
    };
    let mut board = Board::new(0, 7, false);
    let mut pos = add_new_rock(&board, &rock);
    let mut do_move_down = false;
    println!("{:?} {:?}", rock, pos);
    for rock_nr in 0..NR_ROCKS {
        println!("---- Rock {} ----", rock_nr);
        loop {
            if do_move_down {
                if !move_down(&board, &rock, &mut pos) {
                    merge_rock_into_board(&mut board, &rock, &pos);
                    if let Some(try_rock) = rocks_iter.next() {
                        rock = try_rock;
                    } else {
                        panic!("Rock iterator exhausted");
                    }
                    pos = add_new_rock(&board, &rock);
                    println!("{:?} {:?}", rock, pos);
                    do_move_down = false;
                    break;
                } else {
                    println!("V {:?}", pos);
                }
            } else {
                move_sideways(&board, &rock, &mut pos, dir);
                 println!(
                    "{} {:?}",
                    if dir == Dir::Left {
                        '<'
                    } else {
                        '>'
                    },
                    pos
                );
                if let Some(try_dir) = dir_iter.next() {
                    dir = try_dir;
                } else {
                    panic!("Dir iterator exhausted");
                }
            }
            do_move_down = !do_move_down;
        }
    }
    board.rows
}

fn parse_line(line: &str) -> Vec<Dir> {
    line.chars()
        .map(|x| if x == '<' { Dir::Left } else { Dir::Right })
        .collect::<Vec<_>>()
}

fn add_new_rock(board: &Board, rock: &Rock) -> Pos {
    (2, board.rows + 2 + rock_matrix(rock).rows)
}

fn merge_rock_into_board(board: &mut Board, rock: &Rock, pos: &Pos) {
    for _ in board.rows..=pos.1 {
        add_blank_row_to_board(board);
    }
    for (rock_row_idx, rock_row) in rock_matrix(rock).iter().enumerate() {
        for (rock_col_idx, rock_cell) in rock_row.iter().enumerate() {
            let rock_x = rock_col_idx + pos.0;
            let rock_y = pos.1 - (rock_matrix(rock).rows - rock_row_idx - 1);
            if *rock_cell {
                *board.get_mut((rock_y, rock_x)).unwrap() = true;
            }
        }
    }
    //print_board(board);
}

fn print_board(board: &Board) {
    let mut matrix = board.clone();
    matrix.flip_ud();
    for (row_idx, row) in matrix.iter().enumerate() {
        print!("{:>4} ", board.rows - row_idx - 1);
        for cell in row {
            print!("{}", if *cell { '#' } else { '.' });
        }
        println!("");
    }
}

fn add_blank_row_to_board(board: &mut Board) {
    board
        .extend(&[false, false, false, false, false, false, false])
        .unwrap();
}

fn move_down(board: &Board, rock: &Rock, pos: &mut Pos) -> bool {
    if (pos.1 + 1) - rock_matrix(rock).rows == 0 {
        false
    } else {
        let try_pos = (pos.0, pos.1 - 1);
        if check_collision(board, rock, try_pos) {
            false
        } else {
            *pos = try_pos;
            true
        }
    }
}

fn move_sideways(board: &Board, rock: &Rock, pos: &mut Pos, dir: Dir) -> bool {
    let rock_width = rock_matrix(rock).columns;
    if dir == Dir::Left && pos.0 == 0 {
        false
    } else if dir == Dir::Right && (pos.0 + rock_width) >= SHAFT_WIDTH {
        false
    } else {
        let try_pos = (
            if dir == Dir::Left {
                pos.0 - 1
            } else {
                pos.0 + 1
            },
            pos.1,
        );
        if check_collision(board, rock, try_pos) {
            false
        } else {
            *pos = try_pos;
            true
        }
    }
}

fn check_collision(board: &Board, rock: &Rock, pos: Pos) -> bool {
    for (rock_row_idx, rock_row) in rock_matrix(rock).iter().enumerate() {
        for (rock_col_idx, rock_cell) in rock_row.iter().enumerate() {
            if *rock_cell {
                let rock_x = rock_col_idx + pos.0;
                let rock_y = pos.1 - (rock_matrix(rock).rows - rock_row_idx - 1);
                if let Some(board_cell) = board.get((rock_y, rock_x)) {
                    if *board_cell {
                        return true;
                    }
                }
            }
        }
    }
    false
}

#[cfg(test)]
#[test]
fn test_input() {
    const INPUT_FILE: &str = "test";
    const ANSWER: usize = 3068;

    assert_eq!(get_answer(INPUT_FILE), ANSWER);
}