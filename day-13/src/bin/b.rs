use tracing_subscriber::{filter, prelude::*};
use std::{fs::File, sync::Arc};
use tracing::{info, debug};
use std::io::{BufRead, BufReader};
#[macro_use] extern crate scan_fmt;
use tracing::instrument;

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
                // Add an `INFO` filter to the stdout logging layer
                .with_filter(filter::LevelFilter::INFO)
                // Combine the filtered `stdout_log` layer with the
                // `debug_log` layer, producing a new `Layered` layer.
                .and_then(debug_log)
        )
        .init();
}

fn main() {
    setup_tracing();
    info!("{:?}", get_answer("input"));
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
struct ClawMachine {
    button: Vec<(usize, usize)>,
    prize: (usize, usize),
}

impl ClawMachine {
    fn new(lines: Vec<String>) -> Self{
        let a = scan_fmt!(&lines[0], "Button A: X+{}, Y+{}", usize, usize).unwrap();
        let b = scan_fmt!(&lines[1], "Button B: X+{}, Y+{}", usize, usize).unwrap();
        let p = scan_fmt!(&lines[2], "Prize: X={}, Y={}", usize, usize).unwrap();
        let add = 10000000000000usize;
        Self {
            button: Vec::from([a, b]),
            prize: (p.0 + add, p.1 + add),
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
struct StepState {
    pos: (usize, usize),
    presses: (usize, usize),
}

fn press_a(machine: &ClawMachine, state: &mut StepState) {
    state.pos.0 += machine.button[0].0;
    state.pos.1 += machine.button[0].1;
    state.presses.0 += 1;
}

fn depress_a(machine: &ClawMachine, state: &mut StepState) {
    state.pos.0 -= machine.button[0].0;
    state.pos.1 -= machine.button[0].1;
    state.presses.0 -= 1;
}

fn press_b(machine: &ClawMachine, state: &mut StepState) {
    state.pos.0 += machine.button[1].0;
    state.pos.1 += machine.button[1].1;
    state.presses.1 += 1;
}

#[instrument]
fn find_next_step(machine: &ClawMachine, state: &mut StepState) -> bool {
    if state.pos == (0,0) {
        state.presses.0 = machine.prize.0 / machine.button[0].0;
        state.pos.0 = machine.button[0].0 * state.presses.0;
        if state.pos.0 < machine.prize.0 {
            state.presses.0 += 1;
            state.pos.0 += machine.button[0].0;
        }
        state.pos.1 = machine.button[0].1 * state.presses.0;
        if state.pos == machine.prize {
            debug!("-> true");
            return true;
        }
    }
    debug!("Starting state: {:?}", state);
    while state.pos != machine.prize && state.presses.0 > 0 {
        depress_a(machine, state);
        while state.pos.0 > machine.prize.0 && state.presses.0 > 0 {
            depress_a(machine, state);
        }
        while state.pos.0 < machine.prize.0 {
            press_b(machine, state);
        }
//        debug!("Next: {:?}", state);
    }
    if state.pos != machine.prize {
        info!("-> false");
        false
    } else {
        info!("-> true");
        true
    }
}

fn cheapest_prize(machine: &ClawMachine) -> Option<usize> {
    let mut state = StepState::default();

    let modified_machine = if machine.button[0].0 * 3 < machine.button[1].0 {
        ClawMachine { button: vec![machine.button[1], machine.button[0]], prize: machine.prize }
    } else {
        machine.clone()
    };

    if find_next_step(&modified_machine, &mut state) {
        if *machine != modified_machine {
            Some(state.presses.1 * 3 + state.presses.0)
        } else {
            Some(state.presses.0 * 3 + state.presses.1)
        }
    } else {
        None
    }
}

fn get_answer(file: &str) -> usize {
    let machines: Vec<ClawMachine> = BufReader::new(File::open(file).unwrap()).lines()
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
        })
        .into_iter()
        .filter(|group| !group.is_empty())
        .fold(Vec::new(), | mut array, entry | { array.push(ClawMachine::new(entry)); array });

    let mut needed_tokens = 0usize;

    for machine in machines {
        if let Some(nr_tokens) = cheapest_prize(&machine) {
            info!("Tokens: {}", nr_tokens);
            needed_tokens += nr_tokens;
        }
    }

    needed_tokens
}

#[test]
fn test() {
    setup_tracing();
    assert_eq!(480, get_answer("test.a"));
}
