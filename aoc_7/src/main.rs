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
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

impl TryFrom<char> for Card {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Card::Ace),
            'K' => Ok(Card::King),
            'Q' => Ok(Card::Queen),
            'T' => Ok(Card::Ten),
            '9' => Ok(Card::Nine),
            '8' => Ok(Card::Eight),
            '7' => Ok(Card::Seven),
            '6' => Ok(Card::Six),
            '5' => Ok(Card::Five),
            '4' => Ok(Card::Four),
            '3' => Ok(Card::Three),
            '2' => Ok(Card::Two),
            'J' => Ok(Card::Joker),
            _ => Err("This value can not be converted into a Card."),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
enum Value {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(PartialEq, Eq, Ord, Debug)]
struct Hand {
    hand: Vec<Card>,
    value: Value,
    bet: usize,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let hands_val = zip(self.hand.iter(), other.hand.iter())
            .map(|(x, y)| y.cmp(&x))
            .find(|x| x != &Ordering::Equal)
            .unwrap();

        match other.value.cmp(&self.value) {
            Ordering::Equal => return Some(hands_val),
            val => return Some(val),
        }
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
            .filter(|x| x != &Card::Joker)
            .enumerate()
            .map(|(i, card)| {
                println!("{:?} {:?}", i, card);
                if i == 0 {
                    return cards
                        .iter()
                        .filter(|y| y == &&card || y == &&Card::Joker)
                        .count();
                } else {
                    return cards.iter().filter(|y| y == &&card).count();
                }
            })
            .collect::<Vec<_>>();
        card_sets.sort();
        card_sets.reverse();

        println!("{:?}", card_sets);
        let value = match card_sets[..] {
            [5] => Value::FiveOfAKind,
            [] => Value::FiveOfAKind,
            [4, 1] => Value::FourOfAKind,
            [3, 2] => Value::FullHouse,
            [3, 1, 1] => Value::ThreeOfAKind,
            [2, 2, 1] => Value::TwoPair,
            [2, 1, 1, 1] => Value::OnePair,
            [1, 1, 1, 1, 1] => Value::HighCard,
            _ => panic!(),
        };

        return Hand {
            hand: cards,
            value,
            bet,
        };
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
        // println!("{:?}, {:?}, {:?}, {:?}", acc, i + 1, x.bet, (i + 1) * x.bet);
        println!("{:?}", x.hand);
        return acc + (i + 1) * x.bet;
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
