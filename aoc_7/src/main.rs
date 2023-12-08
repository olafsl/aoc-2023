use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;
use std::time::Instant;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl TryFrom<char> for Card {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Card::Ace),
            'K' => Ok(Card::King),
            'Q' => Ok(Card::Queen),
            'J' => Ok(Card::Jack),
            'T' => Ok(Card::Ten),
            '9' => Ok(Card::Nine),
            '8' => Ok(Card::Eight),
            '7' => Ok(Card::Seven),
            '6' => Ok(Card::Six),
            '5' => Ok(Card::Five),
            '4' => Ok(Card::Four),
            '3' => Ok(Card::Three),
            '2' => Ok(Card::Two),
            _ => Err("This value can not be converted into a Card."),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Ord, Clone, Copy)]
struct CardSet {
    value: Card,
    amount: usize,
}

impl PartialOrd for CardSet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match other.amount.partial_cmp(&self.amount) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.value.partial_cmp(&other.value)
    }
}

#[derive(PartialEq, Eq, Ord, Debug)]
struct Hand {
    hand: Vec<CardSet>,
    bet: usize,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let hands_amt = zip(
            self.hand.iter().map(|x| x.amount),
            other.hand.iter().map(|x| x.amount),
        )
        .map(|(x, y)| x.partial_cmp(&y))
        .find(|x| x.unwrap() != Ordering::Equal);
        let hands_val = zip(
            self.hand.iter().map(|x| x.value),
            other.hand.iter().map(|x| x.value),
        )
        .map(|(x, y)| y.partial_cmp(&x))
        .find(|x| x.unwrap() != Ordering::Equal);
        return match (hands_amt, hands_val) {
            (None, a) => a.unwrap(),
            (b, _) => b.unwrap(),
        };
    }
}

impl Hand {
    fn new(card_string: String, bet: usize) -> Self {
        let cards = card_string
            .chars()
            .map(|x| Card::try_from(x).unwrap())
            .collect::<Vec<_>>();
        let mut cards_m = cards.clone();
        cards_m.sort();
        cards_m.dedup();
        let mut card_sets = cards_m
            .into_iter()
            .map(|card| CardSet {
                value: card,
                amount: cards.iter().filter(|y| y == &&card).count(),
            })
            .collect::<Vec<_>>();
        card_sets.sort();
        Hand {
            hand: card_sets,
            bet,
        }
    }
}

fn function(file: BufReader<File>) {
    let mut hands = file
        .lines()
        .map(|x| match x.unwrap().split_once(" ") {
            Some((y, z)) => return Hand::new(y.to_string(), z.parse::<usize>().unwrap()),
            _ => panic!(),
        })
        .collect::<Vec<_>>();

    hands.sort();

    let value = hands.iter().enumerate().fold(0, |acc, (i, x)| {
        println!("{:?}, {:?}, {:?}, {:?}", acc, i + 1, x.bet, (i + 1) * x.bet);
        // println!("{:?}", x.hand);
        return acc + (i + 1) * x.bet
    });

    println!("{:?}", value);
}

fn main() {
    let start = Instant::now();
    let contents =
        BufReader::new(File::open("./src/input").expect("Should have been able to read the file"));
    function(contents);
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration)
}
