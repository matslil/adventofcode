use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(&args[1]) {
        let mut total_priority : u32 = 0;

        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(entry) = line {
                let entry_trimmed = entry.trim();
                let ch = find_duplicate_char(entry_trimmed);
                print!("'{}' ", ch);
                let priority = item_to_priority(ch);
                total_priority += priority;
                println!("{}", priority);
            }
        }
        println!("{}", total_priority);
    }
}

fn item_to_priority(ch : char) -> u32 {
    let ascii = ch as u32;
    if ascii >= 'a' as u32 && ascii <= 'z' as u32 {
        return ascii - 'a' as u32 + 1;
    }
    return ascii - 'A' as u32 + 27;
}

fn find_duplicate_char(str : &str) -> char {
    let char_array : Vec<char> = str.chars().collect();
    let compartment_size = char_array.len() / 2;
    let compartments = vec![&char_array[..compartment_size], &char_array[compartment_size..]];
    for item in compartments[0] {
        for check_with in compartments[1] {
            if item == check_with {
                return *item;
            }
        }
    }
    return '€';
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

