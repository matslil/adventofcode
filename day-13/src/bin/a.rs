// 5904 is to high
// 4044 is to low
use json;

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::env;

enum State {
    Left,
    Right,
    Blank,
}

fn main() {
    let mut state = State::Left;
    let mut idx = 0_usize;
    let mut left: String = String::new();
    let mut right: String;
    let mut ordered_pairs = Vec::<usize>::new();
    for line in BufReader::new(File::open(env::args().nth(1).expect("Could not read file")).unwrap()).lines().map(|x| x.unwrap()) {
        match state {
            State::Left => {
                left = line;
                state = State::Right;
            }
            State::Right => {
                idx += 1;
                right = line;
                state = State::Blank;
                if pair_is_ordered(&left, &right) {
                    ordered_pairs.push(idx);
                }
            }
            State::Blank => {
                state = State::Left;
            }
        }

    }
    println!("{:?}", ordered_pairs);
    println!("{}", ordered_pairs.iter().sum::<usize>());
}

fn pair_is_ordered(left: &str, right: &str) -> bool {
    let left_json = json::parse(left).unwrap();
    let right_json = json::parse(right).unwrap();
    let result = unwrap_json(&left_json, &right_json, 0);
    println!("{:?}\n", result);
    result != UnwrapResult::NoMatch
}

#[derive(Debug, PartialEq, Eq)]
enum UnwrapResult {
    Continue,
    Match,
    NoMatch,
}

fn unwrap_json(left: &json::JsonValue, right: &json::JsonValue, indent: usize) -> UnwrapResult {
    println!("{}{} < {}", "+ ".repeat(indent), left, right);
    match left {
        json::JsonValue::Number(left_number) => {
            match right {
                json::JsonValue::Number(right_number) => {
                    let (_, left_int, _) = left_number.as_parts();
                    let (_, right_int, _) = right_number.as_parts();
                    if left_int < right_int {
                        return UnwrapResult::Match;
                    } else if left_int > right_int {
                        return UnwrapResult::NoMatch;
                    }
                    return UnwrapResult::Continue;
                },
                json::JsonValue::Array(right_vector) => {
                    if right_vector.is_empty() {
                        return UnwrapResult::Continue;
                    }
                    let left_entry: Vec<json::JsonValue> = vec![json::JsonValue::Number(left_number.clone())];
                    return unwrap_json(&json::JsonValue::Array(left_entry), right, indent+1);
                },
                _ => panic!("left-right: Failed match"),
            }
        }
        json::JsonValue::Array(left_vector) => {
            if left_vector.is_empty() {
                return UnwrapResult::Continue;
            }
            match right {
                json::JsonValue::Number(right_number) => {
                    let right_entry: Vec<json::JsonValue> = vec![json::JsonValue::Number(right_number.clone())];
                    return unwrap_json(left, &json::JsonValue::Array(right_entry), indent+1);
                },
                json::JsonValue::Array(right_vector) => {
                    let mut left_iter = left_vector.into_iter();
                    let mut right_iter = right_vector.into_iter();
                    loop {
                        let new_left = left_iter.next();
                        let new_right = right_iter.next();
                        if let Some(right_entry) = new_right {
                            if let Some(left_entry) = new_left {
                                match unwrap_json(left_entry, right_entry, indent+1) {
                                    UnwrapResult::NoMatch => return UnwrapResult::NoMatch,
                                    UnwrapResult::Match => return UnwrapResult::Match,
                                    UnwrapResult::Continue => {},
                                }
                            } else {
                                return UnwrapResult::Match;
                            }
                        } else {
                            return UnwrapResult::NoMatch;
                        }
                   }
               },
               _ => panic!("left-right: Failed match"),
            }
        }
        json::JsonValue::Null => {
            return UnwrapResult::Continue;
        }
        _ => panic!("Unexpected json type"),
    }
}
