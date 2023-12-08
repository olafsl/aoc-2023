use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufReader, BufRead};
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
    Two
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

#[derive(PartialEq, Eq, PartialOrd, Debug)]    
struct CardSet {
    value: Card,
    amount: usize,
}

impl Ord for CardSet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (&self, other) {
            &self.amount > &other.amount => Ordering::Greater,
        }
        todo!()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]    
struct Hand { 
    hand: Vec<CardSet>,
    bid: usize,
    rank: Option<usize>,
}

impl Hand {
    fn new(card_string: String, bid: usize) -> Self {
        let cards = card_string.chars().map(|x| Card::try_from(x).unwrap()).collect::<Vec<_>>();
        let mut cards_m = cards.clone();
        // let card_sets = cards.clone().map(|card| CardSet { value: card, amount: cards.filter(|x: &&Card| **x == card).count() }).collect::<Vec<_>>();
        // Hand {hand: card_sets, bid: bid, rank: None}
        cards_m.sort();
        cards_m.dedup();
        let card_sets = cards_m.into_iter().map(|card| CardSet { value: card, amount: cards.iter().filter(|y| y == &&card).count()}).collect::<Vec<_>>();
        Hand {hand: card_sets, bid: bid, rank: None}
        // let first = {
        //     let card = cards_m.first().unwrap();
        //     let nr = cards.filter(|x| x == card).count();
        //     match nr {
        //         2.. => Some((card, nr)),
        //         _ => None,
        //     }            
        // };
        // let second = {
        //     let card = cards_m[1..].first().unwrap();
        //     let nr = cards.filter(|x| x == card).count();
        //     match nr {
        //         2.. => Some((card, nr)),
        //         _ => None,
        //     }
        // };

        
    }
}

fn function(file: BufReader<File>) {

    // let mut hands = Vec::new();
    for line in file.lines() {
        let split = line.unwrap();
        let split2 = split.split_once(" ").unwrap();
        let hands = Hand::new(split2.0.to_string(), split2.1.parse::<usize>().unwrap());
        println!("{:?}", hands);
        // let thing = line.unwrap();
        // let (string, bet) = thing.split_once(" ").unwrap();
        // let values: [isize; 5] = string.chars().map(|card| {
        //     match card {
        //         'A' => 1,
        //         'K' => 2,
        //         'Q' => 3,
        //         'J' => 4,
        //         'T' => 5,
        //         _ => (card.to_string().parse::<isize>().unwrap() * -1) + 17
        //     }
        // }).collect::<[isize; 5]>();

        // let bla = Vec::new();
        // values.sort();
        // let score = values.iter().fold(|acc: (Vec<(usize, usize)>), x| {
        //     let new_acc = match acc.last() {
        //         [] if val.0 == x => (),
        //         _ => 
        //     }
        //     acc.last().unwrap().0 == x {
        //         return (acc.0, (x, acc.1.1))
        //     } else {
        //         let new_acc = acc.0.push(acc.1);
        //         return (acc.0, (x, 1))
        //     }
        // }, (Vec::new(), (0, 0)))
        // let hand = values.iter().fold(Vec::new(), |acc, f| {
        //     acc.iter().any(f)
        // })
        // println!("{:?}", values.collect::<Vec<_>>());



    }    


}




fn main() {
    let start = Instant::now();
    let contents = BufReader::new(File::open("./src/input_test").expect("Should have been able to read the file"));
    function(contents);
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration)
}
