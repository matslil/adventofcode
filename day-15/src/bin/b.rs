use std::fs::File;
use std::io::{BufReader, BufRead};
use scanf::sscanf;
use core::ops::Sub;

type PosType = i64;
type Pos = (PosType, PosType);

fn main() {
    const INPUT: &str = "input";
    const INSPECT: PosType = 4000000;
    println!("{}", get_answer(INPUT, INSPECT));
}

#[derive(Debug, Clone, Copy)]
struct Sensor {
    pub pos: Pos,
    pub distance: PosType,
}

fn get_answer(file: &str, max: PosType) -> usize {
    let mut sensors = Vec::<Sensor>::new();
    for line in BufReader::new(File::open(file).unwrap()).lines().map(|x| x.unwrap()) {
        sensors.push(parse_line(&line));
    }

    for x in 0..=max {
        for y in 0..=max {
            let mut matches = 0;
            for sensor in &sensors {
                if manhattan(sensor.pos, (x, y)) <= sensor.distance {
                    matches += 1;
                    break;
                }
            }
            if matches == 0 {
                println!("Found: {:?}", (x,y));
                return (x * 4000000 + y) as usize;
            }
        }
    }
    panic!("Nothing found");
}

fn abs_difference<T>(x: T, y: T) -> T
where
    T: Sub<Output = T> + Ord,
{
    if x < y {
        y - x
    } else {
        x - y
    }
}

fn manhattan(pos1: Pos, pos2: Pos) -> PosType
{
    abs_difference(pos2.0, pos1.0) + abs_difference(pos2.1, pos1.1)
}

fn parse_line(line: &str) -> Sensor {
    let mut sensor_x: PosType = 0;
    let mut sensor_y: PosType = 0;
    let mut beacon_x: PosType = 0;
    let mut beacon_y: PosType = 0;
    if ! sscanf!(line, "Sensor at x={}, y={}: closest beacon is at x={}, y={}",
                sensor_x, sensor_y, beacon_x, beacon_y).is_ok() {
        panic!("{}: Malformed input", line);
    }
    Sensor {
        pos: (sensor_x, sensor_y),
        distance: manhattan((sensor_x, sensor_y),(beacon_x, beacon_y)),
    }
}

#[cfg(test)]
#[test]
fn test_manhattan() {
    assert_eq!(manhattan((1,1), (1,1)), 0);
    assert_eq!(manhattan((5,4), (3,2)), 4);
    assert_eq!(manhattan((1,1), (0,3)), 3);
}

#[test]
fn test_input() {
    const INPUT_FILE: &str = "test";
    const MAX: PosType = 20;

    assert_eq!(get_answer(INPUT_FILE, MAX), 56000011 as usize);
}
