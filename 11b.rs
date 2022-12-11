use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

#[derive(Debug, Copy, Clone)]
enum Operation {
    Plus,
    Times,
    Square,
}

impl Default for Operation {
    fn default() -> Self { Operation::Times }
}

type Worry = u64;

#[derive(Debug, Default, Clone)]
struct Monkey {
    items: Vec<Worry>,
    operation: Operation,
    operand: Worry,
    dividend: Worry,
    iftrue: usize,
    iffalse: usize,
    inspected: usize,
    curr_items: Vec<Vec<Worry>>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut monkeys = Vec::<Monkey>::new();
    if let Ok(lines) = read_lines(&args[1]) {
        let mut monkey : Monkey = Default::default();
        for line in lines {
            if let Ok(entry) = line {
                let words : Vec<&str> = entry.trim().split(' ').collect();
                match words[0] {
                    "Starting" => monkey.items = get_item_list(&words[2..]),
                    "Operation:" => {
                        let operand = words[5].parse::<Worry>();
                        if operand.is_err() {
                            monkey.operation = Operation::Square
                        } else {
                            if words[4] == "*" {
                                monkey.operation = Operation::Times;
                            } else {
                                monkey.operation = Operation::Plus;
                            }
                            monkey.operand = operand.unwrap();
                        }
                    },
                    "Test:" => monkey.dividend = words[3].parse::<Worry>().unwrap(),
                    "If" if words[1] == "true:" => monkey.iftrue = words[5].parse::<usize>().unwrap(),
                    "If" => monkey.iffalse = words[5].parse::<usize>().unwrap(),
                    "" => {
                        monkeys.push(monkey);
                        monkey = Default::default();
                        continue;
                    }

                    &_ => continue,
                }
            }
        }
        monkeys.push(monkey);
    }
    println!("{:?}", monkeys);

    for monkey_idx in 0..monkeys.len() {
        for item_idx in 0..monkeys[monkey_idx].items.len() {
            let worry = monkeys[monkey_idx].items[item_idx];
            let mut worries = Vec::<Worry>::new();
            for div_idx in 0..monkeys.len() {
                worries.push(worry % monkeys[div_idx].dividend);
            }
            monkeys[monkey_idx].curr_items.push(worries);
        }
    }

    let mut remainders = Vec::<Worry>::new();
    for _rep in 0..monkeys.len() {
        remainders.push(0);
    }

    for _round in 0..10000 {
        for monkey_idx in 0..monkeys.len() {
            let items = monkeys[monkey_idx].curr_items.clone();
            monkeys[monkey_idx].curr_items.clear();
            for loop_item in items {
                let mut item = loop_item;
                monkeys[monkey_idx].inspected += 1;

                update_curr_item(&monkeys, monkeys[monkey_idx].operation, monkeys[monkey_idx].operand, &mut item);
                let test_idx = if item[monkey_idx] % monkeys[monkey_idx].dividend == 0 {
                        monkeys[monkey_idx].iftrue
                    } else {
                        monkeys[monkey_idx].iffalse
                    };
                monkeys[test_idx].curr_items.push(item);
            }
        }
    }
    println!("{:?}", monkeys);
    let mut inspected = monkeys.into_iter().map(|x| x.inspected).collect::<Vec<usize>>();
    inspected.sort();
    inspected.reverse();
    println!("{}", inspected[0] * inspected[1]);
}

fn update_curr_item(monkeys : &Vec<Monkey>, operation : Operation, operand : Worry, worries : &mut Vec<Worry>) {
    for idx in 0..monkeys.len() {
        let old_value = worries[idx];
        match operation {
            Operation::Plus => worries[idx] = (old_value + operand) % monkeys[idx].dividend,
            Operation::Times => worries[idx] = (old_value * operand) % monkeys[idx].dividend,
            Operation::Square => worries[idx] = (old_value * old_value) % monkeys[idx].dividend,
        }
    }
}

fn get_item_list(words: &[&str]) -> Vec<Worry> {
    let mut worries = Vec::<Worry>::new();
    for word in words {
        worries.push(word.trim_end_matches(',').parse::<Worry>().unwrap());
    }
    return worries;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
