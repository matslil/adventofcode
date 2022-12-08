use std::fs::File;
use std::io::{self, BufRead};
use std::env;
use std::path::{Path, PathBuf};
use std::collections::HashMap;

type CmdOutput = Vec<Vec<String>>;
type Command = (Vec<String>, CmdOutput);

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut commands = Vec::<Command>::new();
    let mut file_sizes = HashMap::<PathBuf, u64>::new();
    let mut dir_sizes = HashMap::<PathBuf, u64>::new();

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(&args[1]) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(entry) = line {
                let words : Vec::<&str> = entry.trim_end().split(' ').collect();
                if words[0] == "$" {
                    commands.push((words[1..].iter().map(|x| x.to_string()).collect(), CmdOutput::new()));
                } else {
                    let last_idx = commands.len() - 1;
                    commands.get_mut(last_idx).unwrap().1.push(words.iter().map(|x| x.to_string()).collect());
                }
            }
        }
    }

    println!("{:?}", commands);

    let mut cwd = PathBuf::new();
    for command in commands {
        match command.0[0].as_str() {
            "cd" => {
                let path = PathBuf::from(command.0[1].as_str());
                if path.is_absolute() {
                   cwd = path;
                } else {
                    cwd = absolute(&cwd.join(path));
                }
            }
            "ls" => {
                for entry in command.1 {
                    if entry[0] == "dir" {
                        continue;
                    }
                    let path = cwd.join(entry[1].as_str());
                    file_sizes.entry(path).or_insert(entry[0].parse().unwrap());
                }
            }
            &_ => panic!("{}: Unknown command", command.0[0]),
        }
    }

    for file in file_sizes {
        let file_path = file.0;
        let file_size = file.1;
        dir_sizes.entry(file_path).and_modify(|size| *size += file_size).or_insert(file_size);
    }

    let mut totals = HashMap::<String, u64>::new();
    calculate_total("/", &dir_sizes, &mut totals);
    println!("{:?}", totals);
    println!("{}", totals_sum(&totals));
}

fn totals_sum(totals : &HashMap<String, u64>) -> u64 {
    let mut total = 0;
    for sum in totals {
        total += sum.1;
    }
    return total;
}

fn calculate_total(dir_prefix : &str, dir_sizes : &HashMap::<PathBuf, u64>, totals : &mut HashMap<String, u64>) -> u64 {
    println!("-- {}", dir_prefix);
    let mut size = 0;
    for entry in dir_sizes {
        if entry.0.to_str().unwrap().starts_with(dir_prefix) {
            let parent = entry.0.parent().unwrap().to_str().unwrap();
            if parent == dir_prefix {
                size += entry.1;
            } else {
                size += calculate_total(parent, dir_sizes, totals);
            }
        }
    }
    if size <= 100000 && ! totals.contains_key(dir_prefix) {
        totals.entry(dir_prefix.to_string()).or_insert(size);
    }
    return size;
}

fn absolute(path : &PathBuf) -> PathBuf {
    let mut path_mut = PathBuf::new();

    for element in path {
        if element == ".." {
            path_mut.pop();
        } else {
            path_mut.push(element);
        }
    }

    return path_mut;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

