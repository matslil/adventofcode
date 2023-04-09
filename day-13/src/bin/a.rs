// 5904 is to high
// 5440 incorect
// 4813 incorrect
// 4044 is to low
use atoi::FromRadix10;

use std::fs::File;
use std::io::{BufReader, BufRead};

type Value = u16;

enum State {
    Left,
    Right,
    Blank,
}

fn main() {
    let answer = get_answer("input");
    if answer >= 5904 {
        println!("{}: Too high", answer);
    } else if answer <= 4044 {
        println!("{}: Too low", answer);
    } else if answer == 5440 {
        println!("{}: Not correct", answer);
    } else if answer == 4813 {
        println!("{}: Not correct", answer);
    } else {
        println!("{}", answer);
    }
}

fn get_answer(file: &str) -> usize {
    let mut state = State::Left;
    let mut idx = 0_usize;
    let mut left: String = String::new();
    let mut right: String;
    let mut ordered_pairs = Vec::<usize>::new();
    for line in BufReader::new(File::open(file).unwrap()).lines().map(|x| x.unwrap()) {
        match state {
            State::Left => {
                left = line;
                state = State::Right;
            }
            State::Right => {
                idx += 1;
                right = line;
                state = State::Blank;
                if pair_is_ordered(&left.as_bytes(), &right.as_bytes()) {
                    println!("Idx: {} -> ordered", idx);
                    ordered_pairs.push(idx);
                } else {
                    println!("Idx: {} -> unordered", idx);
                }
            }
            State::Blank => {
                state = State::Left;
            }
        }

    }
    println!("{:?}", ordered_pairs);
    ordered_pairs.iter().sum::<usize>()
}

fn pair_is_ordered(left_slice: &[u8], right_slice: &[u8]) -> bool {
    let mut left = left_slice.to_vec();
    let mut right = right_slice.to_vec();
    let mut left_depth = 0;
    let mut right_depth = 0;
    let mut left_n = Value::MAX;
    let mut right_n = Value::MAX;

    loop {
        println!("{} < {}", std::str::from_utf8(&left).unwrap(), std::str::from_utf8(&right).unwrap());
        let mut left_used = 0;
        if right[0].is_ascii_digit() {
            let left_n_new;
            (left_n_new, left_used) = Value::from_radix_10(&left.as_slice());
            if left_used > 0 {
                left_n = left_n_new;
                left.drain(..=left_used);
            }
        }
        let mut right_used = 0;
        if left_used > 0 {
            let right_n_new;
            (right_n_new, right_used) = Value::from_radix_10(&right.as_slice());
            if right_used > 0 {
                right_n = right_n_new;
                right.drain(..=right_used);
            }
        }

        if right_used > 0 {
            if left_n < right_n {
                println!("{} < {} => ordered", left_n, right_n);
                return true;
            } else if left_n > right_n {
                println!("{} > {} => unordered", left_n, right_n);
                return false;
            }
        }

        if left.is_empty() {
            println!("Left EOS => ordered");
            return true;
        }

        if right.is_empty() {
            println!("Right EOS => unordered");
            return false;
        }

        if left[0] == '[' as u8 {
            left.drain(0..1);
            left_depth += 1;
            left_n = Value::MAX;
        }
        if right[0] == '[' as u8 {
            right.drain(0..1);
            right_depth += 1;
            right_n = Value::MAX;
        }

        if left[0] == ']' as u8 {
            left.drain(0..1);
            if left.is_empty() {
                println!("Left end of list => ordered");
                return true;
            }
            left_depth -= 1;
//            left_n = Value::MAX;
//            if left_depth < right_depth {
//                println!("Left: left_depth < right_depth => ordered");
//                return true;
//            } else if left_depth > right_depth {
//                println!("Left: right_depth < left_depth => unordered");
//                return false;
//            }
        }

        if right[0] == ']' as u8 {
            right.drain(0..1);
            if right.is_empty() {
                println!("Right end of list => unordered");
                return false;
            }
            right_depth -= 1;
//            right_n = Value::MAX;
//            if left_depth < right_depth {
//                println!("Right: left_depth < right_depth => ordered");
//                return true;
//            } else if left_depth > right_depth {
//                println!("Right: right_depth < left_depth => unordered");
//                return false;
//            }
        }

        if left[0] == ',' as u8 {
            left.drain(0..1);
        }

        if right[0] == ',' as u8 {
            right.drain(0..1);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(get_answer("test"), 13);
    }
}
