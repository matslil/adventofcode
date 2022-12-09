use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Coordinate (i16, i16);

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut head =  Coordinate(0,0);
    let mut tail =  Coordinate(0,0);
    let mut tail_visited = Vec::<Coordinate>::new();

    tail_visited.push(tail);
    if let Ok(lines) = read_lines(&args[1]) {
        for line in lines {
            if let Ok(entry) = line {
                let words : Vec<&str> = entry.trim().split(' ').collect();
                let direction = words[0];
                let distance = words[1].parse::<u16>().unwrap();
                match direction {
                    "R" => perform_stepping(1, 0, distance, &mut head, &mut tail, &mut tail_visited),
                    "L" => perform_stepping(-1, 0, distance, &mut head, &mut tail, &mut tail_visited),
                    "U" => perform_stepping(0, 1, distance, &mut head, &mut tail, &mut tail_visited),
                    "D" => perform_stepping(0, -1, distance, &mut head, &mut tail, &mut tail_visited),
                    &_  => panic!("Unknown direction")
                }
            }
        }
        println!("");
        tail_visited.sort();
        tail_visited.dedup();
        println!("{:?}", tail_visited);
        println!("{}", tail_visited.len());
    }
}

fn perform_stepping(x_step : i16, y_step : i16, distance : u16, head : &mut Coordinate, tail : &mut Coordinate, tail_visited : &mut Vec<Coordinate>) {
//    println!("Step: {}, {} {}", x_step, y_step, distance);
    for _step in 0..distance {
        head.0 += x_step;
        head.1 += y_step;
        move_tail(head, tail);
        tail_visited.push(*tail);
    }
//    println!("{:?}", tail_visited);
}

fn touches(head : &Coordinate, tail : &Coordinate) -> bool {
    return tail.0 <= (head.0 + 1) && tail.0 >= (head.0 - 1) && tail.1 <= (head.1 + 1) && tail.1 >= (head.1 - 1);
}

fn move_tail(head : &Coordinate, tail : &mut Coordinate) {
    if touches(head, tail) {
        return;
    }
    if tail.0 < head.0 {
        tail.0 += 1;
    }
    if tail.0 > head.0 {
        tail.0 -= 1;
    }
    if tail.1 < head.1 {
        tail.1 += 1;
    }
    if tail.1 > head.1 {
        tail.1 -= 1;
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
