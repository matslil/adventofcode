use std::fs::File;
use std::io::{BufReader, BufRead};

const NR_ROCKS: usize = 2022;
const SHAFT_WIDTH: usize = 7;

type RockType = Vec<Vec<bool>>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Rock {
    Minus(RockType),
    Plus(RockType),
    BackL(RockType),
    I(RockType),
    Dot(RockType),
}

type Board = Vec<[bool; 7]>;

type Pos = (usize, usize);

#[derive(Debug, Clone, Copy, Default)]
enum Dir {
    #[default]
    Left,
    Right
}

fn main() {
    const INPUT: &str = "input";
    println!("{}", get_answer(INPUT));
}

fn get_answer(file: &str) -> usize {
    let dir_iter = parse_line(&BufReader::new(File::open(file).unwrap()).lines().map(|x| x.unwrap()).nth(1).unwrap()).into_iter().cycle();
    let rocks: Vec<Rock> = vec![
        Rock::Minus(vec![
            vec![ true, true, true, true ],
        ]),
        Rock::Plus(vec![
            vec![ false, true , false ],
            vec![ true , true , true  ],
            vec![ false, true , false ],
        ]),
        Rock::BackL(vec![
            vec![ false, false, true  ],
            vec![ false, false, true  ],
            vec![ true , true , true  ],
        ]),
        Rock::I(vec![
            vec![ true ],
            vec![ true ],
            vec![ true ],
            vec![ true ],
        ]),
        Rock::Dot(vec![
            vec![ true, true ],
            vec![ true, true ],
        ]),
    ];
    let rocks_iter = rocks.into_iter().cycle();
    let rock = if let Some(try_rock) = rocks_iter.next() {
            try_rock
        } else {
            panic!("Rock iterator exhausted");
        };
    let dir = if let Some(try_dir) = dir_iter.next() {
            try_dir
        } else {
            panic!("Dir iterator exhausted");
        };
    let mut board: Board = Default::default();
    let mut pos  = add_new_rock(&board, &rock);
    let mut do_move_down = false;
    for _ in 0..NR_ROCKS {
        if do_move_down {
            if ! move_down(&board, &rock, &mut pos) {
                merge_rock_into_board(&mut board, &rock, &pos);
                if let Some(try_rock) = rocks_iter.next() {
                    rock = try_rock;
                } else {
                    panic!("Rock iterator exhausted");
                }
                pos = add_new_rock(&board, &rock);
            }
        } else {
            move_sideways(&board, &rock, &mut pos, dir);
            if let Some(try_dir) = dir_iter.next() {
                dir = try_dir;
            } else {
                panic!("Dir iterator exhausted");
            }
        }
    }
    board.len()
}

fn parse_line(line: &str) -> Vec<Dir> {
    line.chars().map(|x| if x == '<' { Dir::Left } else { Dir::Right }).collect::<Vec<_>>()
}

fn add_new_rock(board: &Board, rock: &Rock) -> Pos {
    (2, board.len() + 3 + rock.len())
}

fn merge_rock_into_board(board: &mut Board, rock: &Rock, pos: &Pos) {
    for _ in 0..rock.len() {
        add_blank_row_to_board(board);
    }
    for (rock_y, rock_row) in rock.enumerate() {
        for (rock_x, rock_cell) in rock_row.enumerate() {
            board[rock_y + pos.1][rock_x + pos.0] = rock_cell;
        }
    }
}

fn add_blank_row_to_board(board: &mut Board) {
    board.push([false.repeat(7)]);
}

fn move_down(board: &Board, rock: &Rock, pos: &mut Pos) -> bool {
    let try_pos = (pos.0, pos.1 -1);
    if try_pos.1 == 0 {
        false
    } else if check_collision(board, rock, try_pos) {
        false
    } else {
        pos = try_pos;
        true
    }
}

fn move_sideways(board: &Board, rock: &Rock, pos: &mut Pos, dir: Dir) -> bool {
    let try_pos = (if dir == Dir::Left { pos.0 - 1 } else { pos.0 + 1 }, pos.1);
    if try_pos.0 == 0 || (try_pos.0 + rock[0].len()) >= (SHAFT_WIDTH - 1) {
        false
    } else if check_collision(board, rock, try_pos) {
        false
    } else {
        pos = try_pos;
        true
    }
}

fn check_collision(board: &Board, rock: &Rock, pos: Pos) {
    for (board_y, board_row) in board.enumerate() {
        for (board_x, board_cell) in board_row.enumerate() {
            for (rock_y, rock_row) in rock.enumerate() {
                for (rock_x, rock_cell) in rock_row.enumerate() {
                    if rock_cell {
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
