use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::VecDeque;

fn main() {
    const INPUT: &str = "input";
    println!("{}", get_answer(INPUT));
}

fn get_answer(file: &str) -> isize {
    let lines = BufReader::new(File::open(file).unwrap()).lines().map(|x| x.unwrap()).collect::<Vec<_>>();
    let initial = lines.into_iter().map(|w| w.parse::<isize>().unwrap()).collect::<VecDeque<_>>();
    let mut mixed = initial.clone();
    let mut mixed_idx = VecDeque::new();

    for (idx, _) in mixed.iter().enumerate() {
        mixed_idx.push_back(idx);
    }

    println!("mixed....: {:?}", mixed);
    println!("mixed_idx: {:?}", mixed_idx);

    for (initial_idx, item) in initial.iter().enumerate() {
        println!("---- {}: {} ----", initial_idx, item);
         assert_eq!(initial[initial_idx], mixed[mixed_idx[initial_idx]]);
        let from_idx = mixed_idx[initial_idx as usize];
        let item_from_mixed = mixed.remove(from_idx).unwrap();
         assert_eq!(*item, item_from_mixed);
        let mixed_idx_removed = mixed_idx.remove(from_idx).unwrap();
        println!("mixed....: {:?}: {}", mixed, item);
        println!("mixed_idx: {:?}: {}", mixed_idx, mixed_idx_removed);
        let mut to_idx = (from_idx as isize + item) % mixed.len() as isize;
        if to_idx < 0 {
            to_idx = mixed.len() as isize + to_idx;
        }
        mixed.insert(to_idx as usize, *item);
        mixed_idx.insert(to_idx as usize, mixed_idx_removed);
        println!("mixed....: {:?}", mixed);
        println!("mixed_idx: {:?}", mixed_idx);
    }

    println!("{:?}", mixed);
    mixed[1000 % mixed.len()] + mixed[2000 % mixed.len()] + mixed[3000 % mixed.len()]
}

#[cfg(test)]

mod test {
    use super::*;

#[test]
    fn test_input() {
        const INPUT_FILE: &str = "test";
        const ANSWER: isize = 3;

        assert_eq!(get_answer(INPUT_FILE), ANSWER);
    }
}
