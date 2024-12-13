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
        Self {
            button: Vec::from([a, b]),
            prize: p,
        }
    }
}

// #[instrument(fields(pos, try_nr))]
// fn find_prize(machine: &ClawMachine, pos: (usize, usize), try_nr: usize, tokens_used: usize) -> Option<usize> {
//     if pos == machine.prize {
//         info!("{:?}: Found a prize!", machine);
//         return Some(tokens_used);
//     }
//     if try_nr == 100 {
//         info!("{:?}: Reached 100 tries, stopping", machine);
//         return None;
//     }
//     if pos.0 > machine.prize.0 || pos.1 > machine.prize.1 {
//         return None;
//     }
//     let try1 = find_prize(machine, (pos.0 + machine.button_a.0, pos.1 + machine.button_a.1), try_nr + 1, tokens_used + 3);
//     let try2 = find_prize(machine, (pos.0 + machine.button_b.0, pos.1 + machine.button_b.1), try_nr + 1, tokens_used + 1);
//     if try1.is_none() && try2.is_none() {
//         None
//     } else if try1.is_none() {
//         try2
//     } else if try2.is_none() {
//         try1
//     } else {
//         std::cmp::min(try1, try2)
//     }
// }

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
struct StepState {
    pos: (usize, usize),
    presses: (usize, usize),
}

fn press_a(machine: &ClawMachine, state: &mut StepState) {
    state.pos.0 += machine.button[0].0;
    state.pos.1 += machine.button[0].1;
    state.presses.0 += 1;
    debug!("pressed A: {:?}", state);
}

fn depress_a(machine: &ClawMachine, state: &mut StepState) {
    state.pos.0 -= machine.button[0].0;
    state.pos.1 -= machine.button[0].1;
    state.presses.0 -= 1;
    debug!("depressed A: {:?}", state);
}

fn press_b(machine: &ClawMachine, state: &mut StepState) {
    state.pos.0 += machine.button[1].0;
    state.pos.1 += machine.button[1].1;
    state.presses.1 += 1;
    debug!("pressed B: {:?}", state);
}

#[instrument]
fn find_next_step(machine: &ClawMachine, state: &mut StepState) -> bool {
    if state.pos == (0,0) {
        while state.pos.0 < machine.prize.0 {
            press_a(machine, state);
        }
        debug!("Starting state: {:?}", state);
        if state.pos == machine.prize {
            debug!("-> true");
            return true;
        }
    }
    while state.pos != machine.prize && state.presses.0 > 0 {
        depress_a(machine, state);
        while state.pos.0 > machine.prize.0 && state.presses.0 > 0 {
            depress_a(machine, state);
        }
        while state.pos.0 < machine.prize.0 {
            press_b(machine, state);
        }
        debug!("Next state: {:?}", state);
    }
    if state.pos != machine.prize {
        debug!("-> false");
        false
    } else {
        debug!("-> true");
        true
    }
}

fn cheapest_prize(machine: &ClawMachine) -> Option<usize> {
    let mut cheapest: Option<usize> = None;

    let mut state = StepState::default();
    while find_next_step(machine, &mut state) {
        let cost = state.presses.0 * 3 + state.presses.1;
        if let Some(cheapest_sofar) = cheapest {
            if cost < cheapest_sofar {
                debug!("Cheapest: {}", cost);
                cheapest = Some(cost);
            }
        } else {
            debug!("Cheapest: {}", cost);
            cheapest = Some(cost);
        }
        if state.presses.0 > 0 {
            depress_a(machine, &mut state);
        }
    }
    cheapest
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
struct ClawState {
    pos: (usize, usize),
    tokens: usize,
}

fn one_move(machine: &ClawMachine, from: &ClawState) -> Vec<ClawState> {
    let mut states: Vec<ClawState> = Vec::new();
    for button in 0..=1 {
        if machine.button[button].0 + from.pos.0 <= machine.prize.0
            && machine.button[button].1 + from.pos.1 <= machine.prize.1 {
                states.push(ClawState {
                    pos: (from.pos.0 + machine.button[button].0, from.pos.1 + machine.button[button].1),
                    tokens: if button == 0 { 3 } else { 1 },
                });
            }
    }
    states
}

#[instrument]
fn find_prize(machine: &ClawMachine) -> Option<usize> {
    let mut states: Vec<ClawState> = Vec::new();
    states.push(ClawState::default());

    for _ in 0..100 {
        let mut new_states: Vec<ClawState> = Vec::new();
        for state in states {
            if state.pos == machine.prize {
                debug!("Found price after {} tokens", state.tokens);
                return Some(state.tokens);
            }
            new_states.append(&mut one_move(machine, &state));
        }
        states = new_states;
    }
    debug!("None found");
    None
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

    debug!("Machines: {:?}", machines);

    for machine in machines {
//        if let Some(nr_tokens) = find_prize(&machine) {
        if let Some(nr_tokens) = cheapest_prize(&machine) {
            debug!("{}", nr_tokens);
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
