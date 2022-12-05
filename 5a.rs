use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
use std::collections::BTreeMap;
use std::collections::VecDeque;

type Stack = BTreeMap<usize, VecDeque<char>>;

fn main() {
    let args: Vec<String> = env::args().collect();
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(&args[1]) {
        let mut is_move = false;
        let mut stack = Stack::new();

        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(entry) = line {
                let entry_trimmed = entry.trim_end();
                if is_move {
                    execute(entry_trimmed, &mut stack);
                    println!("{:?}", stack);
                } else {
                    if entry_trimmed.len() == 0 {
                        is_move = true;
                        println!("--- Moving ---");
                        continue;
                    }
                    let char_array : Vec<char> = entry_trimmed.chars().collect();
                    if char_array[1] != '1' {
                        parse_crate_stacks(&char_array, &mut stack);
                        println!("{:?}", stack);
                    }
                }
            }
        }
        print_stack_top(&stack);
    }
}

fn print_stack_top(stack : &Stack) {
    for entry in stack {
        print!("{}", entry.1.front().unwrap());
    }
    println!("");
}

fn parse_crate_stacks(char_array : &Vec<char>, stack : &mut Stack) {
    let nr_stacks = char_array.len() / 4 + 1;
    for stack_nr in 0..nr_stacks {
        let crate_name = char_array[stack_nr * 4 + 1];
        if ('A'..='Z').contains(&crate_name) {
            stack.entry(stack_nr + 1).or_default().push_back(crate_name);
        }
    }
}

fn execute(line : &str, stack : &mut Stack) {
    let words : Vec<&str> = line.split(' ').collect();
    let nr_crates = words[1].parse::<usize>().unwrap();
    let from_stack = words[3].parse::<usize>().unwrap();
    let to_stack = words[5].parse::<usize>().unwrap();
    for _repetition in 0..nr_crates {
        let crate_name = stack.get_mut(&from_stack).unwrap().pop_front().unwrap();
        stack.entry(to_stack).or_default().push_front(crate_name);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

