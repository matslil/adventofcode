use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

type Elem = i64;

#[derive(Debug, PartialEq, Clone)]
enum Op {
    Num(Elem),
    Var,
    Equ(((String, bool), (String, bool))),
    Add(((String, bool), (String, bool))),
    Sub(((String, bool), (String, bool))),
    Mul(((String, bool), (String, bool))),
    Div(((String, bool), (String, bool))),
}

type Statements = HashMap<String, Op>;

fn main() {
    const INPUT: &str = "input";
    get_answer(INPUT);
}

fn get_answer(file: &str) {
    let mut statements = Statements::new();
    for line in BufReader::new(File::open(file).unwrap()).lines().map(|x| x.unwrap()) {
        let (name, op) = parse_line(&line);
        statements.insert(name, op);
    }
    let mut stack = Vec::new();
    let mut op_name = "root".to_string();
    // @todo: Find two leaves, perform parent operation on them, and recurse
    loop {
        let mut result: Option<Elem> = None;
        println!("Checking: {} {:?}", op_name, statements.get(&op_name).unwrap());
        match statements.get(&op_name).unwrap() {
            Op::Equ(((lhs, ldone), (rhs, rdone))) => {
                if ! ldone {
                    match statements.get(lhs).unwrap() {
                        Op::Add(_) | Op::Sub(_) | Op::Mul(_) | Op::Div(_) => {
                            stack.push(op_name);
                            op_name = lhs.clone();
                            continue;
                        }
                        _ => {}
                    }
                }
                if ! rdone {
                    match statements.get(rhs).unwrap() {
                        Op::Add(_) | Op::Sub(_) | Op::Mul(_) | Op::Div(_) => {
                            stack.push(op_name);
                            op_name = rhs.clone();
                            continue;
                        }
                        _ => {}
                    }
                }
                break;
            }
            Op::Num(_) => {
                panic!("{}: Did not expect Num", op_name);
            }
            Op::Var => {}
            Op::Add(((lhs, ldone), (rhs, rdone))) => {
                if !ldone && let Op::Num(lhs_value) = statements.get(lhs).unwrap() {
                    if !rdone && let Op::Num(rhs_value) = statements.get(rhs).unwrap() {
                        result = Some(lhs_value + rhs_value);
                    } else if *statements.get(rhs).unwrap() != Op::Var {
                        stack.push(op_name);
                        op_name = rhs.clone();
                        continue;
                    }
                } else if statements.get(lhs).unwrap() != &Op::Var {
                    stack.push(op_name);
                    op_name = lhs.clone();
                    continue;
                }
            }
            Op::Sub((lhs, rhs)) => {
                if let Op::Num(lhs_value) = statements.get(lhs).unwrap() {
                    if let Op::Num(rhs_value) = statements.get(rhs).unwrap() {
                        result = Some(lhs_value - rhs_value);
                    } else if statements.get(rhs).unwrap() != &Op::Var {
                        stack.push(op_name);
                        op_name = rhs.clone();
                        continue;
                    }
                } else if statements.get(lhs).unwrap() != &Op::Var {
                    stack.push(op_name);
                    op_name = lhs.clone();
                    continue;
                }
            }
            Op::Mul((lhs, rhs)) => {
                if let Op::Num(lhs_value) = statements.get(lhs).unwrap() {
                    if let Op::Num(rhs_value) = statements.get(rhs).unwrap() {
                        result = Some(lhs_value * rhs_value);
                    } else if statements.get(rhs).unwrap() != &Op::Var {
                        stack.push(op_name);
                        op_name = rhs.clone();
                        continue;
                    }
                } else if statements.get(lhs).unwrap() != &Op::Var {
                    stack.push(op_name);
                    op_name = lhs.clone();
                    continue;
                }
            }
            Op::Div((lhs, rhs)) => {
                if let Op::Num(lhs_value) = statements.get(lhs).unwrap() {
                    if let Op::Num(rhs_value) = statements.get(rhs).unwrap() {
                        result = Some(lhs_value / rhs_value);
                    } else if statements.get(rhs).unwrap() != &Op::Var {
                        stack.push(op_name);
                        op_name = rhs.clone();
                        continue;
                    }
                } else if statements.get(lhs).unwrap() != &Op::Var {
                    stack.push(op_name);
                    op_name = lhs.clone();
                    continue;
                }
            }
        }
        if let Some(new_op_name) = stack.pop() {
            println!("{} <- {:?}, going to {}",op_name, result, new_op_name);
            if let Some(result_unwrapped) = result {
                *(statements.get_mut(&op_name).unwrap()) = Op::Num(result_unwrapped);
            }
            op_name = new_op_name;
        } else {
            break;
        }
    }
    println!("{:?}", statements);
}

fn nr(line: &str) -> Elem {
    line.parse::<Elem>().unwrap()
}

fn parse_line(line: &str) -> (String, Op) {
    let words = line.split(" ").collect::<Vec<_>>();
    let name = words[0].strip_suffix(":").unwrap();
    (name.to_string(),
    if name == "humn" {
        Op::Var
    } else if name == "root" {
        Op::Equ((words[1].to_string(), words[3].to_string()))
    } else {
        match words.get(2) {
            None => Op::Num(nr(words[1])),
            Some(&"+") =>  Op::Add((words[1].to_string(), words[3].to_string())),
            Some(&"-") =>  Op::Sub((words[1].to_string(), words[3].to_string())),
            Some(&"*") =>  Op::Mul((words[1].to_string(), words[3].to_string())),
            Some(&"/") =>  Op::Div((words[1].to_string(), words[3].to_string())),
            Some(&_)  =>  panic!("{:?}: Unknown operation", words.get(2)),
        }
    })
}

#[cfg(test)]
#[test]
fn test_input() {
    const INPUT_FILE: &str = "test";
    const ANSWER: Elem = 152;

    get_answer(INPUT_FILE);
}
