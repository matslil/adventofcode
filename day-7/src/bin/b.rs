//248553310 too low
//251106438 too low

use tracing::{self, info};
use tracing_subscriber::{filter, prelude::*};
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::sync::Arc;
use std::fmt;
use std::cmp;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
enum Card {
    CardJ,
    Card2,
    Card3,
    Card4,
    Card5,
    Card6,
    Card7,
    Card8,
    Card9,
    CardT,
    CardQ,
    CardK,
    CardA,
}

impl Card {
    fn new(card: char) -> Self {
        match card {
            '2' => Card::Card2,
            '3' => Card::Card3,
            '4' => Card::Card4,
            '5' => Card::Card5,
            '6' => Card::Card6,
            '7' => Card::Card7,
            '8' => Card::Card8,
            '9' => Card::Card9,
            'T' => Card::CardT,
            'J' => Card::CardJ,
            'Q' => Card::CardQ,
            'K' => Card::CardK,
            'A' => Card::CardA,
            _   => panic!("{}: Unknown card", card),
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Card::Card2 => '2',
            Card::Card3 => '3',
            Card::Card4 => '4',
            Card::Card5 => '5',
            Card::Card6 => '6',
            Card::Card7 => '7',
            Card::Card8 => '8',
            Card::Card9 => '9',
            Card::CardT => 'T',
            Card::CardJ => 'J',
            Card::CardQ => 'Q',
            Card::CardK => 'K',
            Card::CardA => 'A',
        })
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl fmt::Display for HandType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            HandType::HighCard     => "High card",
            HandType::OnePair      => "One pair",
            HandType::TwoPairs     => "Two pairs",
            HandType::ThreeOfAKind => "Three of a kind",
            HandType::FullHouse    => "Full house",
            HandType::FourOfAKind  => "Four of a kind",
            HandType::FiveOfAKind  => "Five of a kind",
        })
    }
}

struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
}

impl Hand {
    fn new(line: &str) -> Self {
        let mut ch = line.chars();
        let mut unsorted_cards: Vec<Card> = Vec::new();

        for _ in 0..5 {
            unsorted_cards.push(Card::new(ch.next().unwrap()));
        }
        let mut cards = unsorted_cards.clone();
        cards.sort();

        let mut duplicates: Vec<(Card, usize)> = Vec::new();
        let mut iter = cards.iter();
        let mut prev_card = iter.next().unwrap();
        let mut nr_duplicates: usize = 0;
        let mut nr_jokers: usize = 0;
        for card in iter {
            if prev_card.clone() == Card::CardJ {
                nr_jokers += 1;
            }
            if prev_card == card {
                nr_duplicates += 1;
            } else if nr_duplicates > 0 {
                duplicates.push((prev_card.clone(), nr_duplicates));
                nr_duplicates = 0;
            }
            prev_card = card;
        }
        if nr_duplicates > 0 {
            duplicates.push((prev_card.clone(), nr_duplicates));
        }
        if prev_card.clone() == Card::CardJ {
            nr_jokers += 1;
        }

        // Sorts by number of duplicates in descending order
        duplicates.sort_by(|a, b| b.1.cmp(&a.1));

        let mut hand_type = if duplicates.len() == 1 && duplicates[0].1 == 4 {
            HandType::FiveOfAKind
        } else if duplicates.len() == 1 && duplicates[0].1 == 3 {
            HandType::FourOfAKind
        } else if duplicates.len() == 2 && duplicates[0].1 != duplicates[1].1 {
            HandType::FullHouse
        } else if duplicates.len() == 1 && duplicates[0].1 == 2 {
            HandType::ThreeOfAKind
        } else if duplicates.len() == 2 && duplicates[0].1 == 1 && duplicates[1].1 == 1 {
            HandType::TwoPairs
        } else if duplicates.len() == 1 && duplicates[0].1 == 1 {
            HandType::OnePair
        } else if duplicates.len() == 0 {
            HandType::HighCard
        } else {
            panic!("{:?}: Unknown hand type", duplicates);
        };

        duplicates.retain(|e| e.0 != Card::CardJ);

        info!("Duplicates: {:?}, nr jokers: {}", duplicates, nr_jokers);

        // See if joker can improve this
        let alternative_hand_type = if nr_jokers == 4 {
            HandType::FiveOfAKind
        } else if (nr_jokers == 3) && (duplicates.len() > 0) && (duplicates[0].1 > 0) {
            HandType::FiveOfAKind
        } else if (nr_jokers == 2) && (duplicates.len() > 0) && (duplicates[0].1 > 1) {
            HandType::FiveOfAKind
        } else if (nr_jokers == 1) && (duplicates.len() > 0) && (duplicates[0].1 > 2) {
            HandType::FiveOfAKind
        } else if nr_jokers == 3 {
            HandType::FourOfAKind
        } else if nr_jokers == 2 && duplicates.len() > 0 && duplicates[0].1 > 0 {
            HandType::FourOfAKind
        } else if nr_jokers == 1 && duplicates.len() > 0 && duplicates[0].1 > 1 {
            HandType::FourOfAKind
        } else if nr_jokers == 3 && duplicates.len() > 0 {
            HandType::FullHouse
        } else if nr_jokers == 2 && duplicates.len() > 0 && duplicates[0].1 > 1 {
            HandType::FullHouse
        } else if nr_jokers == 2 {
            HandType::ThreeOfAKind
        } else if nr_jokers == 1 && duplicates.len() > 0 {
            HandType::ThreeOfAKind
        } else if nr_jokers > 0 && duplicates.len() > 0 {
            HandType::TwoPairs
        } else if nr_jokers == 1 {
            HandType::OnePair
        } else {
            HandType::HighCard
        };

        if alternative_hand_type > hand_type {
            hand_type = alternative_hand_type.clone();
        }

        info!("{:?} {:?} => {} ({})", cards, duplicates, hand_type, alternative_hand_type);

        Self { cards: unsorted_cards, hand_type: hand_type }
    }
}

impl cmp::Ord for Hand {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        let type_ordering = self.hand_type.cmp(&other.hand_type);
        if type_ordering == cmp::Ordering::Equal {
            let mut self_iter = self.cards.iter();
            let mut other_iter = other.cards.iter();
            while let Some(self_card) = self_iter.next() {
                let other_card = other_iter.next().unwrap();
                let card_ordering = self_card.cmp(other_card);
                if card_ordering != cmp::Ordering::Equal {
                    return card_ordering;
                }
            }
            return cmp::Ordering::Equal;
        }
        type_ordering
    }
}

impl cmp::PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == cmp::Ordering::Equal
    }
}

impl cmp::Eq for Hand {}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for card in &self.cards {
            write!(f, "{}", card)?;
        }
        write!(f, " ({})", self.hand_type)
    }
}

struct Game {
    plays: Vec<(Hand, usize)>,
    total_winning: usize,
}

impl Game {
    fn new(lines: &mut impl Iterator<Item=String>) -> Self {
        let mut plays: Vec<(Hand, usize)> = Vec::new();
        let mut total_winning: usize = 0;
        for line in lines {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let hand = Hand::new(parts[0]);
            let bid = parts[1].parse::<usize>().unwrap();
            plays.push((hand, bid));
        }
        plays.sort();
        for (idx, hand_bid) in plays.iter().enumerate() {
//            info!("{} * {} = {}", (idx + 1), hand_bid.1, (idx + 1) * hand_bid.1);
            total_winning += (idx + 1) * hand_bid.1;
        }
        Self { plays: plays, total_winning: total_winning }
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (idx, hand_bid) in self.plays.iter().enumerate() {
            writeln!(f, "{}: {} {:4}", idx + 1, hand_bid.0, hand_bid.1)?;
        }
        writeln!(f, "Total winning: {}", self.total_winning)
    }
}

fn setup_tracing() {
    let stdout_log = tracing_subscriber::fmt::layer()
        .pretty();

    // A layer that logs events to a file.
    let file = File::create("debug.log");
    let file = match file  {Ok(file) => file,Err(error) => panic!("Error: {:?}",error),};
    let debug_log = tracing_subscriber::fmt::layer()
        .with_writer(Arc::new(file))
        .with_ansi(false);

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

fn main() {
    setup_tracing();
    println!("{:?}", get_answer("input"));
}

fn get_answer(file: &str) -> usize {
     let mut iter = BufReader::new(File::open(file).unwrap()).lines().map(|e| e.unwrap());

     let game = Game::new(&mut iter);
     info!("{}", game);
     game.total_winning
}

#[test]
fn test() {
    setup_tracing();
    assert_eq!(5905, get_answer("test"));
    assert_eq!(1369, get_answer("test.extra"));
}
