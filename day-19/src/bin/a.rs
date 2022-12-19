// 3694 - Too high

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::cmp::Ordering;
use std::ops::{Add, Sub, AddAssign, SubAssign};
use pathfinding::prelude::dijkstra;
use num_traits::identities::Zero;

const TIME_MIN: usize = 24;

#[derive(Debug, Clone, Default, PartialEq, Eq, Copy, Hash)]
struct Resources {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

// A wrapper around Resource
// Compare geode only, since this is the only thing we care about
// Higher geode has lower ordinal, since we want to maximize geode
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct Cost (u32);

// Step through the time, noting current resource level and production
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd)]
struct Node {
    current: Resources,
    production: Resources,
    time: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Hash, PartialOrd)]
struct Blueprint {
    id: u32,
    ore_robot: Resources,
    clay_robot: Resources,
    obsidian_robot: Resources,
    geode_robot: Resources,
}

impl Node {
    fn successors(&self, blueprint: &Blueprint) -> Vec<(Node, Cost)> {
        let mut alternatives = Vec::new();
        let time = self.time + 1;
        let current = Resources {
            ore: self.current.ore + self.production.ore,
            clay: self.current.clay + self.production.clay,
            obsidian: self.current.obsidian + self.production.obsidian,
            geode: self.current.geode + self.production.geode,
        };

        if self.current >= blueprint.ore_robot {
            alternatives.push(( Node {
                current: current - blueprint.ore_robot,
                production: Resources {
                    ore: self.production.ore + 1,
                    clay: self.production.clay,
                    obsidian: self.production.obsidian,
                    geode: self.production.geode,
                },
                time,
            }, Cost (current.geode)
            ));
        }
        if self.current >= blueprint.clay_robot {
            alternatives.push(( Node {
                current: current - blueprint.clay_robot,
                production: Resources {
                    ore: self.production.ore,
                    clay: self.production.clay + 1,
                    obsidian: self.production.obsidian,
                    geode: self.production.geode,
                },
                time,
            }, Cost (current.geode)
            ));
        }
        if self.current >= blueprint.obsidian_robot {
            alternatives.push(( Node {
                current: current - blueprint.obsidian_robot,
                production: Resources {
                    ore: self.production.ore,
                    clay: self.production.clay + 1,
                    obsidian: self.production.obsidian,
                    geode: self.production.geode,
                },
                time,
            }, Cost (current.geode)
            ));
        }
        if self.current >= blueprint.geode_robot {
            alternatives.push(( Node {
                current: current - blueprint.geode_robot,
                production: Resources {
                    ore: self.production.ore,
                    clay: self.production.clay,
                    obsidian: self.production.obsidian,
                    geode: self.production.geode + 1,
                },
                time,
            }, Cost (current.geode)
            ));
        }
        println!("{:?} -> {:?}", self, alternatives);
        alternatives
    }
}

impl Zero for Cost {
    fn zero() -> Self {
        Cost(0)
    }

    fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

impl Ord for Cost {
    fn cmp(&self, lhs: &Self) -> Ordering {
        match self.0.cmp(&lhs.0) {
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => Ordering::Equal,
            Ordering::Greater => Ordering::Less,
        }
    }
}

impl PartialOrd for Cost {
    fn partial_cmp(&self, lhs: &Self) -> Option<Ordering> {
        if lhs.0 == self.0 {
            Some(Ordering::Equal)
        } else if lhs.0 < self.0 {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Less)
        }
    }
}

impl Sub for Cost {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self ( self.0 - rhs.0 )
    }
}

impl Add for Cost {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self ( self.0 + rhs.0 )
    }
}

impl SubAssign for Cost {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self( self.0 - rhs.0 );
    }
}

impl AddAssign for Cost {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self( self.0 + rhs.0 );
    }
}

impl PartialOrd for Resources {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        if self == rhs {
            Some(Ordering::Equal)
        } else if self.ore >= rhs.ore && self.clay >= rhs.clay && self.obsidian >= rhs.obsidian && self.geode >= rhs.geode {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Less)
        }
    }
}

impl Sub for Resources {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            obsidian: self.obsidian - rhs.obsidian,
            geode: self.geode - rhs.geode,
        }
    }
}

impl Add for Resources {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode,
        }
    }
}

impl SubAssign for Resources {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            obsidian: self.obsidian - rhs.obsidian,
            geode: self.geode - rhs.geode,
        }
    }
}

impl AddAssign for Resources {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode,
        }
    }
}

fn main() {
    const INPUT: &str = "input";
    println!("{}", get_answer(INPUT));
}

fn get_answer(file: &str) -> usize {
    let mut blueprints = Vec::new();
    for (idx, line) in BufReader::new(File::open(file).unwrap()).lines().map(|x| x.unwrap()).enumerate() {
        blueprints.push(parse_line(&line, idx as u32 + 1));
    }
    println!("{:?}", blueprints);
    0
}

fn geodes_opened(blueprint: &Blueprint) -> u32 {
    let start_node = Node {
        current: Resources { ore: 0, clay: 0, obsidian: 0, geode: 0 },
        production: Resources { ore: 1, clay: 0, obsidian: 0, geode: 0 },
        time: 1,
    };

    println!("---- Blueprint: {:?} ----", blueprint);
    let (_, geodes) = dijkstra(&start_node, |n| n.successors(blueprint), |n| n.time == 24).unwrap();
    geodes.0
}

fn parse_line(line: &str, id: u32) -> Blueprint {
    let numbers: Vec<u32> = line.split(" ").map(|word| word.parse::<u32>()).filter(|maybe_number| maybe_number.is_ok()).map(|number_result| number_result.unwrap()).collect();

    Blueprint {
        id,
        ore_robot: Resources {ore:  numbers[0], clay: 0, obsidian: 0, geode: 0 },
        clay_robot: Resources { ore: numbers[1], clay: 0, obsidian: 0, geode: 0 },
        obsidian_robot: Resources { ore: numbers[2], clay: numbers[3], obsidian: 0, geode: 0 },
        geode_robot: Resources { ore: numbers[4], clay: 0, obsidian: numbers[5], geode: 0 },
    }
}

#[cfg(test)]

mod test {
    use super::*;

#[test]
    fn test_blueprints() {
        assert_eq!(geodes_opened( &Blueprint {
            id: 1,
            ore_robot: Resources { ore: 4, clay: 0, obsidian: 0, geode: 0, },
            clay_robot: Resources { ore: 2, clay: 0, obsidian: 0, geode: 0 },
            obsidian_robot: Resources { ore: 3, clay: 14, obsidian: 0, geode: 0 },
            geode_robot: Resources { ore: 2, clay: 0, obsidian: 7, geode: 0 },
        }), 9);

        assert_eq!(geodes_opened( &Blueprint {
            id: 1,
            ore_robot: Resources { ore: 2, clay: 0, obsidian: 0, geode: 0, },
            clay_robot: Resources { ore: 3, clay: 0, obsidian: 0, geode: 0 },
            obsidian_robot: Resources { ore: 3, clay: 8, obsidian: 0, geode: 0 },
            geode_robot: Resources { ore: 3, clay: 0, obsidian: 12, geode: 0 },
        }), 12);
    }

#[test]
    fn test_input() {
        const INPUT_FILE: &str = "test";
        const ANSWER: usize = 33;

        assert_eq!(get_answer(INPUT_FILE), ANSWER);
    }
}
