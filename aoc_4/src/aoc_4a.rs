use std::fs::File;
use std::io::{prelude::*, BufReader};

fn parse_string(string: &str) -> Vec<i32> {
    return string.trim().split_ascii_whitespace().collect::<Vec<&str>>().iter().map(|&x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
}

fn check_game(input: (&str, &str)) -> i32 {

    let win = parse_string(input.0);
    let own = parse_string(input.1);
    let mut score = 0;
    for nr in win {
        match (own.contains(&nr), score) {
            (false, _) => (),
            (true, 0) => score = 1,
            (true, _) => score *= 2,
        }
    }
    return score
}

fn main() {
    let contents = BufReader::new(File::open("./src/input").expect("Should have been able to read the file"));

    let mut score = 0;
    for line in contents.lines() {
        let games = line.unwrap();
        let (_, game) = games.split_once(':').unwrap();
        score += check_game(game.split_once('|').unwrap());
    }
    println!("{}", score);

}

