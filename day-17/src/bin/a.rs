use tracing_subscriber::{filter, prelude::*};
use std::{fs::File, sync::Arc};
use tracing::{info, debug, warn};
use std::io::{BufRead, BufReader};
#[macro_use] extern crate scan_fmt;
use itertools::Itertools;

fn setup_tracing() {
    let stdout_log = tracing_subscriber::fmt::layer()
        .pretty();

    // A layer that logs events to a file.
    let file = File::create("debug.log");
    let file = match file  {Ok(file) => file,Err(error) => panic!("Error: {:?}",error),};
    let debug_log = tracing_subscriber::fmt::layer()
        .with_writer(Arc::new(file));

    tracing_subscriber::registry()
        .with(
            stdout_log
                .with_filter(filter::LevelFilter::INFO)
                .and_then(debug_log)
        )
        .init();
}

fn main() {
    setup_tracing();
    info!("{:?}", get_answer("input"));
}

const OP_ADV: u8 = 0;
const OP_BXL: u8 = 1;
const OP_BST: u8 = 2;
const OP_JNZ: u8 = 3;
const OP_BXC: u8 = 4;
const OP_OUT: u8 = 5;
const OP_BDV: u8 = 6;
const OP_CDV: u8 = 7;

struct State {
    code: Vec<u8>,
    ip: usize,
    a: usize,
    b: usize,
    c: usize,
    out: Vec<usize>,
}

fn combo_operand(state: &State, operand: u8) -> usize {
    if operand <= 3 {
        operand as usize
    } else if operand == 4 {
        state.a
    } else if operand == 5 {
        state.b
    } else if operand == 6 {
        state.c
    } else {
        panic!("Illegal operand");
    }
}

fn execute_next(state: &mut State) -> bool {
    if state.ip >= state.code.len()-1 {
        return false;
    }

    let opcode = state.code[state.ip];
    let operand = state.code[state.ip+1];

    match opcode {
        OP_ADV => {
            state.a = state.a / (2usize.pow(combo_operand(state, operand) as u32));
            state.ip += 2;
        },
        OP_BXL => {
            state.b = state.b ^ operand as usize;
            state.ip += 2;
        },
        OP_BST => {
            state.b = combo_operand(state, operand) % 8;
            state.ip += 2;
        },
        OP_JNZ => {
            if state.a == 0 {
                state.ip += 2;
            } else {
                state.ip = operand as usize;
            }
        },
        OP_BXC => {
            state.b = state.b ^ state.c;
            state.ip += 2;
        },
        OP_OUT => {
            state.out.push(combo_operand(state, operand) % 8);
            state.ip += 2;
        },
        OP_BDV => {
            state.b = state.a / (2usize.pow(combo_operand(state, operand) as u32));
            state.ip += 2;
        },
        OP_CDV => {
            state.c = state.a / (2usize.pow(combo_operand(state, operand) as u32));
            state.ip += 2;
        },
        _ => panic!("Illegal opcode"),
    }
    true
}

fn get_answer(file: &str) -> String {
    let input = BufReader::new(File::open(file).unwrap()).lines()
        .filter_map(Result::ok)
        .fold(vec![Vec::new()], |mut acc, line| {
            if line.is_empty() {
                acc.push(Vec::new());
            } else {
                if let Some(last) = acc.last_mut() {
                    last.push(line);
                }
            }
            acc
        });

    let regs: Vec<usize> = input[0].iter()
        .take_while(|line| !line.is_empty()).map(|line| {
        let parse = scan_fmt!(&line, "Register {}: {}", char, usize).unwrap();
        parse.1
    })
    . collect();

    let code_line: Vec<&str> = input[1][0].split_whitespace().collect();

    let code: Vec<u8> = code_line[1].split(',')
            .map(|entry| entry.parse::<u8>().unwrap())
            .collect::<Vec<_>>();

    let mut state = State {
        code,
        ip: 0,
        a: regs[0],
        b: regs[1],
        c: regs[2],
        out: Vec::new(),
    };

    while execute_next(&mut state) {};

    state.out.into_iter().join(",")
}

#[test]
fn test() {
    setup_tracing();
    assert_eq!("4,6,3,5,6,3,5,2,1,0", get_answer("test.a"));
}
