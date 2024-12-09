use tracing_subscriber::{filter, prelude::*};
use std::{fs, sync::Arc};
use tracing::{info, debug};
use std::io::{BufRead, BufReader};
use itertools::Itertools;

fn setup_tracing() {
    let stdout_log = tracing_subscriber::fmt::layer()
        .pretty();

    // A layer that logs events to a file.
    let file = fs::File::create("debug.log");
    let file = match file  {Ok(file) => file,Err(error) => panic!("Error: {:?}",error),};
    let debug_log = tracing_subscriber::fmt::layer()
        .with_writer(Arc::new(file));

    tracing_subscriber::registry()
        .with(
            stdout_log
                // Add an `INFO` filter to the stdout logging layer
                .with_filter(filter::LevelFilter::INFO)
                // Combine the filtered `stdout_log` layer with the
                // `debug_log` layer, producing a new `Layered` layer.
                .and_then(debug_log)
        )
        .init();
}

#[derive(Debug, Clone)]
struct File {
    id: usize,
    size: usize,
    free: usize
}

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for _ in 0..self.size {
            write!(f, "{}", self.id)?;
        }
        for _ in 0..self.free {
            write!(f, ".")?;
        }
        Ok(())
    }
}

fn print_fragmentation(files: &Vec<File>) -> String
{
    let mut result = String::new();
    for file in files {
       result.push_str(&format!("{}", file));
    }
    result
}

fn main() {
    setup_tracing();
    info!("{:?}", get_answer("input"));
}

fn find_first_free(files: &Vec<File>) -> Option<usize> {
    for file in files.iter().enumerate() {
        if file.1.free > 0 {
            return Some(file.0);
        }
    }
    None
}

fn defrag(files: &mut Vec<File>) {
    loop {
        if let Some(first_free) = find_first_free(files) {
            let mut last = files.len() - 1;
            if first_free == last {
                break;
            }
            if files[first_free].id == files[last].id {
                files[first_free].size += 1;
                files[first_free].free -= 1;
            } else {
                let free = files[first_free].free;
                files[first_free].free = 0;
                files.insert(first_free+1, File {
                    id: files[last].id,
                    size: 1,
                    free: free-1
                });
            }
            last = files.len() - 1;
            files[last].size -= 1;
            files[last].free += 1;
            if files[last].size == 0 {
                let free = files[last].free;
                files.remove(last);
                files[last-1].free += free;
            }
        } else {
            break;
        }
//        debug!("{}", print_fragmentation(files));
    }
    let last = files.len() - 1;
    if files[last].id == files[last-1].id {
        files[last-1].size += files[last].size;
        files[last-1].free += files[last].free;
        files.remove(last);
    }
}

fn checksum(files: &Vec<File>) -> usize {
    let mut sum = 0usize;

    let mut position = 0usize;

    for file in files {
        let _span = tracing::span!(tracing::Level::INFO, "checksum", "{:?}", file).entered();
        for pos in position..position+file.size {
            sum += file.id * pos;
            debug!("Size {}: {} * {} = {}", file.size, pos, file.id, sum);
        }
        position += file.size;
    }

    sum
}

fn get_answer(file: &str) -> usize {
    let mut files: Vec<File> = Vec::new();

    for chunk in BufReader::new(fs::File::open(file).unwrap()).lines()
        .filter_map(Result::ok).
        flat_map(|line| line
            .chars()
            .collect::<Vec<char>>()
            .into_iter()
            .map(|ch| ch.to_string().parse::<usize>().unwrap())
        ).chunks(2).into_iter().enumerate() {
            let chunk_vec: Vec<usize> = chunk.1.into_iter().collect();
            debug!("{:?}", chunk_vec);
            files.push(File{ id: chunk.0, size: chunk_vec[0], free: if chunk_vec.len() > 1usize {chunk_vec[1]} else { 0usize } });
        }

    debug!("{}", print_fragmentation(&files));
    defrag(&mut files);
    debug!("{:?}", files);
    return checksum(&files);
}

#[test]
fn test() {
    setup_tracing();
    assert_eq!(1928, get_answer("test.a"));
}
