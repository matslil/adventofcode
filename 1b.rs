use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(&args[1]) {
        // Consumes the iterator, returns an (Optional) String
        let mut total_calories : u32 = 0;
        let mut calories = Vec::new();
        for line in lines {
            if let Ok(entry) = line {
                let entry_trimmed = entry.trim();
                if entry_trimmed == "" {
                    calories.push(total_calories);
                    total_calories = 0;
                } else {
                    total_calories +=  entry_trimmed.parse::<u32>().unwrap();
                }
            }
        }
        calories.sort();
        calories.reverse();
        print!("{}", calories[0] + calories[1] + calories[2]);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
