use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(&args[1]) {
        let mut total_priority : u32 = 0;
        let mut nr_lines = 0;
        let mut group : [String; 3] = Default::default();

        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(entry) = line {
                let entry_trimmed = entry.trim();
                group[nr_lines] = entry_trimmed.to_string();
                nr_lines += 1;
                if nr_lines > 2 {
                    nr_lines = 0;
                    let ch = find_common_item(&group);
                    let priority = item_to_priority(ch);
                    println!("{} ({})", priority, ch);
                    total_priority += priority;
                }
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

fn find_common_item(group : &[String; 3]) -> char {
    for ch0 in group[0].chars().collect::<Vec<char>>() {
        for ch1 in group[1].chars().collect::<Vec<char>>() {
            for ch2 in group[2].chars().collect::<Vec<char>>() {
                if ch0 == ch1 && ch1 == ch2 {
                    return ch0;
                }
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

