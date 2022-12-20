// 3694 - Too high

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::cmp::Ordering;
use std::ops::{Add, Sub, AddAssign, SubAssign};

const TIME_MIN: usize = 24;

#[derive(Debug, Clone, Default, PartialEq, Eq, Copy, Hash)]
struct Resources {
    ore: u16,
    clay: u16,
    obsidian: u16,
    geode: u16,
}

// Step through the time, noting current resource level and production
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd)]
struct Node {
    current: Resources,
    production: Resources,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Hash, PartialOrd)]
struct Blueprint {
    ore_robot: Resources,
    clay_robot: Resources,
    obsidian_robot: Resources,
    geode_robot: Resources,
}

impl Node {
    fn successors(&self, blueprint: &Blueprint) -> Vec<Node> {
        let mut alternatives = Vec::new();
        let current = self.current + self.production;

        alternatives.push( Node {
            current,
            production: self.production,
        }
        );

        if self.current >= blueprint.ore_robot {
            alternatives.push( Node {
                current: current - blueprint.ore_robot,
                production: Resources {
                    ore: self.production.ore + 1,
                    clay: self.production.clay,
                    obsidian: self.production.obsidian,
                    geode: self.production.geode,
                },
            }
            );
        }
        if self.current >= blueprint.clay_robot {
            alternatives.push( Node {
                current: current - blueprint.clay_robot,
                production: Resources {
                    ore: self.production.ore,
                    clay: self.production.clay + 1,
                    obsidian: self.production.obsidian,
                    geode: self.production.geode,
                },
            }
            );
        }
        if self.current >= blueprint.obsidian_robot {
            alternatives.push( Node {
                current: current - blueprint.obsidian_robot,
                production: Resources {
                    ore: self.production.ore,
                    clay: self.production.clay,
                    obsidian: self.production.obsidian + 1,
                    geode: self.production.geode,
                },
            }
            );
        }
        if self.current >= blueprint.geode_robot {
            alternatives.push( Node {
                current: current - blueprint.geode_robot,
                production: Resources {
                    ore: self.production.ore,
                    clay: self.production.clay,
                    obsidian: self.production.obsidian,
                    geode: self.production.geode + 1,
                },
            }
            );
        }
        // println!("{:?} -> {:?}", self, alternatives);
        alternatives
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
    let mut quality_level = 0;
    for (idx, line) in BufReader::new(File::open(file).unwrap()).lines().map(|x| x.unwrap()).enumerate() {
        quality_level += geodes_opened(&parse_line(&line)) as usize * (idx + 1);
    }
    quality_level
}

fn filter_expression(time: usize, max_geode: u16, current: &Resources, blueprint: &Blueprint) -> bool {
    if current.geode == max_geode {
        true
    } else if current.ore < (blueprint.geode_robot.ore*2/3) || current.obsidian < (blueprint.geode_robot.obsidian*2/3) {
//        println!("{}: Filtering: {:?}", time, current);
        false
    } else {
        true
    }
}

fn geodes_opened(blueprint: &Blueprint) -> u16 {
    let start_node = Node {
        current: Resources { ore: 0, clay: 0, obsidian: 0, geode: 0 },
        production: Resources { ore: 1, clay: 0, obsidian: 0, geode: 0 },
    };

    let mut nodes = Vec::<Node>::new();
    let mut max_geode = 0;

    nodes.push(start_node);

    println!("---- Blueprint: {:?} ----", blueprint);
    for time in 0..TIME_MIN {
        let mut new_nodes = Vec::new();
        for node in &nodes {
            new_nodes.append(&mut node.successors(blueprint));
        }
        max_geode = nodes.iter().fold(0, |max, &node| if node.current.geode > max { node.current.geode } else { max });
        if new_nodes.iter().fold(false, |acc, &node| if acc || (node.current.ore >= blueprint.geode_robot.ore && node.current.obsidian >= blueprint.geode_robot.obsidian) { true } else { false }) {
            nodes = new_nodes.into_iter().filter(|node| filter_expression(time, max_geode, &node.current, blueprint)).collect::<Vec<_>>();
        } else {
            nodes = new_nodes;
        }
        println!("{}: max geode: {}, {} alternatives", time + 1, max_geode, nodes.len());
    }
    nodes.into_iter().fold(0, |max, node| if node.current.geode > max { node.current.geode } else { max })
}

fn parse_line(line: &str) -> Blueprint {
    let numbers: Vec<u16> = line.split(" ").map(|word| word.parse::<u16>()).filter(|maybe_number| maybe_number.is_ok()).map(|number_result| number_result.unwrap()).collect();

    Blueprint {
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
    fn test_resource() {
        let resource1 = Resources { ore: 1, clay: 0, obsidian: 0, geode: 0 };
        let resource2 = Resources { ore: 0, clay: 0, obsidian: 0, geode: 0 };

        assert_eq!(resource1, resource1);
        assert_eq!(resource2, resource2);
        assert!(resource1 >= resource2);
        assert!(resource1 >= resource1);
    }

#[test]
    fn test_blueprints() {
        assert_eq!(geodes_opened( &Blueprint {
            ore_robot: Resources { ore: 4, clay: 0, obsidian: 0, geode: 0, },
            clay_robot: Resources { ore: 2, clay: 0, obsidian: 0, geode: 0 },
            obsidian_robot: Resources { ore: 3, clay: 14, obsidian: 0, geode: 0 },
            geode_robot: Resources { ore: 2, clay: 0, obsidian: 7, geode: 0 },
        }), 9);

        assert_eq!(geodes_opened( &Blueprint {
            ore_robot: Resources { ore: 2, clay: 0, obsidian: 0, geode: 0, },
            clay_robot: Resources { ore: 3, clay: 0, obsidian: 0, geode: 0 },
            obsidian_robot: Resources { ore: 3, clay: 8, obsidian: 0, geode: 0 },
            geode_robot: Resources { ore: 3, clay: 0, obsidian: 12, geode: 0 },
        }), 12);
    }

//#[test]
//    fn test_input() {
//        const INPUT_FILE: &str = "test";
//        const ANSWER: u16 = 33;

//        assert_eq!(get_answer(INPUT_FILE), ANSWER);
//    }
}
