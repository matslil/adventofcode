use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
use std::collections::VecDeque;

type Values = VecDeque<i32>;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut values = Values::new();
    if let Ok(lines) = read_lines(&args[1]) {
        for line in lines {
            if let Ok(entry) = line {
                let words : Vec<&str> = entry.trim().split(' ').collect();
                values.push_back(*values.back().unwrap_or(&1i32));
                if words[0] == "addx" {
                    let value = words[1].parse::<i32>().unwrap();
                    values.push_back(value + *values.back().unwrap());
                }
                println!("{:>3}: {} {:>3} => {:>4}", values.len(), words[0], words.get(1).unwrap_or(&""), values.back().unwrap());
            }
        }
    }
    let mut sum = 0;
    for cycle in (20..=220).step_by(40) {
        let strength = values[cycle-2] * (cycle as i32);
        println!("{:>3} * {:>3} = {:>4}", cycle, values[cycle-2], strength);
        sum += strength;
    }
    println!("{}", sum);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
