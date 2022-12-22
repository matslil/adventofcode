use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Coordinate (i16, i16);

type Rope = VecDeque::<Coordinate>;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut rope = Rope::new();
    let mut tail_visited = Vec::<Coordinate>::new();

    for _n in 0..10 {
        rope.push_back(Coordinate(0,0));
    }

    tail_visited.push(Coordinate(0,0));
    if let Ok(lines) = read_lines(&args[1]) {
        for line in lines {
            if let Ok(entry) = line {
                let words : Vec<&str> = entry.trim().split(' ').collect();
                let direction = words[0];
                let distance = words[1].parse::<u16>().unwrap();
                match direction {
                    "R" => perform_stepping(1, 0, distance, &mut rope, &mut tail_visited),
                    "L" => perform_stepping(-1, 0, distance, &mut rope, &mut tail_visited),
                    "U" => perform_stepping(0, 1, distance, &mut rope, &mut tail_visited),
                    "D" => perform_stepping(0, -1, distance, &mut rope, &mut tail_visited),
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

fn perform_stepping(x_step : i16, y_step : i16, distance : u16, rope : &mut Rope, tail_visited : &mut Vec<Coordinate>) {
//    println!("Step: {}, {} {}", x_step, y_step, distance);
    for _step in 0..distance {
        let head = rope.front_mut().unwrap();
        head.0 += x_step;
        head.1 += y_step;
        for knot_idx in 1..rope.len() {
            let new_knot = move_tail(&rope[knot_idx-1], &rope[knot_idx]);
            if let Some(knot) = rope.get_mut(knot_idx) {
                *knot = new_knot;
            } else {
                panic!("Knot panic!");
            }
        }
        tail_visited.push(*rope.back().unwrap());
    }
//    println!("{:?}", tail_visited);
}

fn touches(head : &Coordinate, tail : &Coordinate) -> bool {
    return tail.0 <= (head.0 + 1) && tail.0 >= (head.0 - 1) && tail.1 <= (head.1 + 1) && tail.1 >= (head.1 - 1);
}

fn move_tail(head : &Coordinate, tail : &Coordinate) -> Coordinate {
    let mut new_tail = *tail;
    if touches(head, tail) {
        return new_tail;
    }
    if tail.0 < head.0 {
        new_tail.0 += 1;
    }
    if tail.0 > head.0 {
        new_tail.0 -= 1;
    }
    if tail.1 < head.1 {
        new_tail.1 += 1;
    }
    if tail.1 > head.1 {
        new_tail.1 -= 1;
    }
    return new_tail;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
