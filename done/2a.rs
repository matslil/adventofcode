use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

fn main() {
    let select_pts = HashMap::from([
                                     ('X', 1),
                                     ('Y', 2),
                                     ('Z', 3)
    ]);

    let match_pts = HashMap::from([
                                    ("A X", 3),
                                    ("A Y", 6),
                                    ("A Z", 0),
                                    ("B X", 0),
                                    ("B Y", 3),
                                    ("B Z", 6),
                                    ("C X", 6),
                                    ("C Y", 0),
                                    ("C Z", 3)
    ]);

    let args: Vec<String> = env::args().collect();
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(&args[1]) {
        // Consumes the iterator, returns an (Optional) String
        let mut pts = 0;
        for line in lines {
            if let Ok(entry) = line {
                let entry_trimmed = entry.trim();
                let first_char = entry_trimmed.chars().nth(2).unwrap();
                let round_pts = match_pts.get(entry_trimmed).unwrap() + select_pts.get(&first_char).unwrap();
                pts += round_pts;
                println!("{} + {} = {}", match_pts.get(entry_trimmed).unwrap(), select_pts.get(&first_char).unwrap(), round_pts);
            }
        }
        println!("{}", pts);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

