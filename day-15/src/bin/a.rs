use std::fs::File;
use std::io::{BufReader, BufRead};
use scanf::sscanf;
use core::ops::Sub;

type PosType = i64;

type Pos = (PosType, PosType);

fn main() {
    const INPUT: &str = "input";
    const INSPECT: PosType = 2000000;
    println!("{}", get_answer(INPUT, INSPECT));
}

#[derive(Debug, Clone, Copy)]
struct Sensor {
    pub pos: Pos,
    pub beacon: Pos,
}

fn get_answer(file: &str, inspect_y: PosType) -> usize {
    let mut sensors = Vec::<Sensor>::new();
    for line in BufReader::new(File::open(file).unwrap()).lines().map(|x| x.unwrap()) {
        sensors.push(parse_line(&line));
    }
    let mut max_x = 0;
    let mut min_x = PosType::MAX;
    for sensor in &sensors {
        let distance = manhattan(sensor.pos, sensor.beacon);
        let new_max_x = sensor.pos.0 + distance;
        let new_min_x = sensor.pos.0 - distance;
        if new_max_x > max_x {
            max_x = new_max_x;
        }
        if new_min_x < min_x {
            min_x = new_min_x;
        }
    }
    let mut covered_x = Vec::<PosType>::new();
    for x in min_x..=max_x {
        for sensor in &sensors {
            let beacon_distance = manhattan(sensor.pos, sensor.beacon);
            let y_distance = manhattan(sensor.pos, (x, inspect_y));
            if y_distance <= beacon_distance && sensor.beacon != (x,inspect_y) {
                covered_x.push(x);
                break;
            }
        }
    }

    println!("{:?}", covered_x);

    covered_x.len()
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
        beacon: (beacon_x, beacon_y)
    }
}

#[cfg(test)]
#[test]
fn test_manhattan() {
    assert_eq!(manhattan((1,1), (1,1)), 0);
    assert_eq!(manhattan((5,4), (3,2)), 4);
    assert_eq!(manhattan((1,1), (0,3)), 3);
    assert_eq!(manhattan((-1, -1), (-2, 1)), 3);
}

#[test]
fn test_input() {
    const INPUT_FILE: &str = "test";
    const INPUT_Y: PosType = 10;

    assert_eq!(get_answer(INPUT_FILE, INPUT_Y), 26 as usize);
}
