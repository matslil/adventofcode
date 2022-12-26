use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::VecDeque;
use std::env;
use std::collections::BTreeMap;

type Dirs = BTreeMap<Vec<String>, u64>;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", get_answer(&args[1]));
}

fn get_answer(file: &str) -> u64 {
    let line_iter = BufReader::new(File::open(file).unwrap()).lines().map(|x| x.unwrap());
    let mut cwd = Vec::new();
    let mut dirs = Dirs::new();

    for line in line_iter {
        let words = line.split(' ').collect::<Vec<_>>();
        if words[0] == "$" {
            if words[1] == "cd" {
                println!("{}", words[2]);
                if words[2] == ".." {
                    cwd.pop().unwrap();
                } else {
                    let is_absolute = words[2].starts_with("/");
                    let path = if words[2] != "/" { words[2].split('/').map(|e| e.to_string()).collect::<Vec<_>>() } else { vec!["".to_string()] };
                    if is_absolute {
                        cwd = path;
                    } else {
                        cwd.extend(path);
                    }
                    println!("cwd ({}): {:?}", if is_absolute { "abs" } else { "rel" }, cwd);
                }
            }
        } else {
            if let Ok(n) = words[0].parse::<u64>() {
                let mut path = VecDeque::from(cwd.clone());
                while ! path.is_empty() {
                    let path_vec: Vec<String> = Vec::from(path.clone());
                    dirs.entry(path_vec.clone()).and_modify(|x| *x += n).or_insert(n);
                    println!("{:?}: {:?}", &path_vec, dirs.get(&path_vec));
                    path.pop_back();
                }
            }
        }
    }

    println!("{:?}", dirs);

    dirs.into_iter().fold(0, |acc, entry| if entry.1 <= 100000 { acc + entry.1 } else { acc })
}