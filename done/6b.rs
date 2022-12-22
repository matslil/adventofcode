use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
use std::collections::VecDeque;
use std::hash::Hash;
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(&args[1]) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(entry) = line {
                let entry_trimmed = entry.trim_end();
                println!("{}", find_start_of_packet(entry_trimmed));
            }
        }
    }
}

fn find_start_of_packet(line : &str) -> u32 {
    let line_chars : Vec::<char> = line.chars().collect();
    let mut ring = VecDeque::<char>::new();
    let mut nr_chars = 0;

    for curr_char in line_chars {
        if nr_chars >= 14 {
            ring.pop_front();
        }
        ring.push_back(curr_char);
        nr_chars += 1;
        println!("{:?} {}", ring, curr_char);
        if nr_chars >= 14 && has_unique_elements(&ring) {
            return nr_chars;
        }
    }
    return 0;
}

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

