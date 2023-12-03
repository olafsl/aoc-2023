use std::fs::File;
use std::io::{prelude::*, BufReader};
use regex::Regex;

fn check_game(game: &str) -> i32 {

    let re = Regex::new(r"(?P<amount>\d*) (?P<color>red|green|blue)").unwrap();

    let mut rgb = (0, 0, 0);
    for rgb_match in re.captures_iter(game) {
        match (&rgb_match["color"], rgb_match["amount"].parse::<i32>().unwrap()) {
            ("red", d) if d > rgb.0 => rgb.0 = d,
            ("green", d) if d > rgb.1 => rgb.1 = d,
            ("blue", d) if d > rgb.2 => rgb.2 = d,
            _ => (),
        }
    }
    return rgb.0*rgb.1*rgb.2;
}

fn main() {
    let contents = BufReader::new(File::open("./src/input").expect("Should have been able to read the file"));

    let mut games_power = 0;
    for line in contents.lines() {
        let games = line.unwrap();
        let (_, game) = games.split_once(':').unwrap();
        games_power += check_game(game);
    }

    println!("{}", games_power);
}
