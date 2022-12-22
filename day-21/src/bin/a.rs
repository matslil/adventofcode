// 3694 - Too high

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

type Elem = i64;

enum Op {
    Num(Elem),
    Add((String, String)),
    Sub((String, String)),
    Mul((String, String)),
    Div((String, String)),
}

type Statements = HashMap<String, Op>;

fn main() {
    const INPUT: &str = "input";
    println!("{}", get_answer(INPUT));
}

fn get_answer(file: &str) -> Elem {
    let mut statements = Statements::new();
    for line in BufReader::new(File::open(file).unwrap()).lines().map(|x| x.unwrap()) {
        let (name, op) = parse_line(&line);
        statements.insert(name, op);
    }
    let mut stack = Vec::new();
    let mut op_name = "root".to_string();
    // @todo: Find two leaves, perform parent operation on them, and recurse
    loop {
        let result: Elem;
        match statements.get(&op_name).unwrap() {
            Op::Num(_) => {
                panic!("{}: Did not expect Num", op_name);
            }
            Op::Add((lhs, rhs)) => {
                if let Op::Num(lhs_value) = statements.get(lhs).unwrap() {
                    if let Op::Num(rhs_value) = statements.get(rhs).unwrap() {
                        result = lhs_value + rhs_value;
                    } else {
                        stack.push(op_name);
                        op_name = rhs.clone();
                        continue;
                    }
                } else {
                    stack.push(op_name);
                    op_name = lhs.clone();
                    continue;
                }
            }
            Op::Sub((lhs, rhs)) => {
                if let Op::Num(lhs_value) = statements.get(lhs).unwrap() {
                    if let Op::Num(rhs_value) = statements.get(rhs).unwrap() {
                        result = lhs_value - rhs_value;
                    } else {
                        stack.push(op_name);
                        op_name = rhs.clone();
                        continue;
                    }
                } else {
                    stack.push(op_name);
                    op_name = lhs.clone();
                    continue;
                }
            }
            Op::Mul((lhs, rhs)) => {
                if let Op::Num(lhs_value) = statements.get(lhs).unwrap() {
                    if let Op::Num(rhs_value) = statements.get(rhs).unwrap() {
                        result = lhs_value * rhs_value;
                    } else {
                        stack.push(op_name);
                        op_name = rhs.clone();
                        continue;
                    }
                } else {
                    stack.push(op_name);
                    op_name = lhs.clone();
                    continue;
                }
            }
            Op::Div((lhs, rhs)) => {
                if let Op::Num(lhs_value) = statements.get(lhs).unwrap() {
                    if let Op::Num(rhs_value) = statements.get(rhs).unwrap() {
                        result = lhs_value / rhs_value;
                    } else {
                        stack.push(op_name);
                        op_name = rhs.clone();
                        continue;
                    }
                } else {
                    stack.push(op_name);
                    op_name = lhs.clone();
                    continue;
                }
            }
        }
        if let Some(new_op_name) = stack.pop() {
            println!("{} <- {}",op_name,  result);
            *(statements.get_mut(&op_name).unwrap()) = Op::Num(result);
            op_name = new_op_name;
        } else {
            return result;
        }
    }
}

fn nr(line: &str) -> Elem {
    line.parse::<Elem>().unwrap()
}

fn parse_line(line: &str) -> (String, Op) {
    let words = line.split(" ").collect::<Vec<_>>();
    let name = words[0].strip_suffix(":").unwrap();
    (name.to_string(), match words.get(2) {
        None => Op::Num(nr(words[1])),
        Some(&"+") =>  Op::Add((words[1].to_string(), words[3].to_string())),
        Some(&"-") =>  Op::Sub((words[1].to_string(), words[3].to_string())),
        Some(&"*") =>  Op::Mul((words[1].to_string(), words[3].to_string())),
        Some(&"/") =>  Op::Div((words[1].to_string(), words[3].to_string())),
        Some(&_)  =>  panic!("{:?}: Unknown operation", words.get(2)),
    })
}

#[cfg(test)]
#[test]
fn test_input() {
    const INPUT_FILE: &str = "test";
    const ANSWER: Elem = 152;

    assert_eq!(get_answer(INPUT_FILE), ANSWER);
}
