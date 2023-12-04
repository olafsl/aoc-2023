use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::cmp;

fn parse_string(string: &str) -> Vec<&str> {
    return string.trim().split_ascii_whitespace().collect::<Vec<&str>>();
}

fn check_game(input: (&str, &str)) -> usize {

    let win = parse_string(input.0);
    let own = parse_string(input.1);
    let mut score = 0;
    for nr in win {
        match own.contains(&nr) {
            true => score += 1,
            false => (),
        }
    }
    return score
}

fn main() {
    let contents = BufReader::new(File::open("./src/input").expect("Should have been able to read the file"));

    let max_nr = 199;
    let mut nr_scratchcards = vec![1; max_nr];

    for (index, line) in contents.lines().enumerate() {
        let curr_score = nr_scratchcards[index];
        let games = line.unwrap();
        let (_, game) = games.split_once(':').unwrap();
        let score = check_game(game.split_once('|').unwrap());
        if score != 0 {
            for i in (cmp::min(index+1, max_nr))..(cmp::min(index+score+1,max_nr)) {
                nr_scratchcards[i] += curr_score
            }
        }
    }
    println!("{:?}", nr_scratchcards.iter().sum::<usize>());

}

