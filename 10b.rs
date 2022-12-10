use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
use std::collections::VecDeque;

type Values = VecDeque<i32>;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut values = Values::new();
    values.push_back(1i32);
    if let Ok(lines) = read_lines(&args[1]) {
        for line in lines {
            if let Ok(entry) = line {
                let words : Vec<&str> = entry.trim().split(' ').collect();
                values.push_back(*values.back().unwrap_or(&1i32));
                if words[0] == "addx" {
                    let value = words[1].parse::<i32>().unwrap();
                    values.push_back(value + *values.back().unwrap());
                }
            }
        }
    }

    for cycle in 0..values.len() {
        let column = cycle as i32 % 40;
        if column == 0 {
            println!("");
        }
        if values[cycle] >= (column - 1) && values[cycle] <= (column + 1) {
            print!("#");
        } else {
            print!(" ");
        }
    }
    println!("");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
