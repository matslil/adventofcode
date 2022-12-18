// 3694 - Too high

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

type Elem = i8;
type Pos = (Elem, Elem, Elem);
type Grid = HashMap<Pos, ()>;

fn main() {
    const INPUT: &str = "input";
    println!("{}", get_answer(INPUT));
}

fn get_answer(file: &str) -> usize {
    let mut grid = Grid::new() ;
    for line in BufReader::new(File::open(file).unwrap()).lines().map(|x| x.unwrap()) {
        grid.insert(parse_line(&line), ());
    }
    let nr_sides = grid.keys().fold(0_usize, |sum, &pos| sum + count_exposed_sides(&grid, pos));
    println!("Nr sides: {}", nr_sides);
    nr_sides
}

fn parse_line(line: &str) -> Pos {
    let words = line.split(",").collect::<Vec<_>>();
    (
        words[0].parse::<Elem>().unwrap(),
        words[1].parse::<Elem>().unwrap(),
        words[2].parse::<Elem>().unwrap(),
    )
}

fn count_exposed_sides(grid: &Grid, pos: Pos) -> usize
{
    let mut count: usize = 6;
    if grid.contains_key(&(pos.0 - 1, pos.1, pos.2)) {
        count -= 1;
    }
    if grid.contains_key(&(pos.0 + 1, pos.1, pos.2)) {
        count -= 1;
    }
    if grid.contains_key(&(pos.0, pos.1 - 1, pos.2)) {
        count -= 1;
    }
    if grid.contains_key(&(pos.0, pos.1 + 2, pos.2)) {
        count -= 1;
    }
    if grid.contains_key(&(pos.0, pos.1, pos.2 - 1)) {
        count -= 1;
    }
    if grid.contains_key(&(pos.0, pos.1, pos.2 + 1)) {
        count -= 1;
    }
    count
}

#[cfg(test)]
#[test]
fn test_input() {
    const INPUT_FILE: &str = "test";
    const ANSWER: usize = 64;

    assert_eq!(get_answer(INPUT_FILE), ANSWER);
}
