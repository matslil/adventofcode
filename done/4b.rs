use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(&args[1]) {
        let mut nr_contains = 0;

        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(entry) = line {
                let entry_trimmed = entry.trim();
                let range_pair = line_to_range_pairs(entry_trimmed);
                let contains =
                    (range_pair[0].contains(range_pair[1].start()) || range_pair[0].contains(range_pair[1].end())) ||
                    (range_pair[1].contains(range_pair[0].start()) || range_pair[1].contains(range_pair[0].end()));
                if contains {
                    nr_contains += 1;
                }
                println!("{:?} {}", range_pair, contains);
            }
        }
        println!("{}", nr_contains);
    }
}

fn line_to_range_pairs(line : &str) -> [std::ops::RangeInclusive<i32>; 2] {
    let str_range : Vec<&str> = line.split(',').collect();
    let range_pairs : [Vec<&str>; 2] = [str_range[0].split('-').collect(), str_range[1].split('-').collect()];
    return [
        std::ops::RangeInclusive::new(
            range_pairs[0][0].parse::<i32>().unwrap(), range_pairs[0][1].parse::<i32>().unwrap()
        ),
        std::ops::RangeInclusive::new(
            range_pairs[1][0].parse::<i32>().unwrap(), range_pairs[1][1].parse::<i32>().unwrap()
        )
    ];
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

