use regex::Regex;
use std::{fs::File, io::BufRead, io::BufReader};

fn capture(word: &str, re_digits: Regex) -> String {
    let re_numbers = Regex::new(r"([0-9])").unwrap();
    let digit = re_digits.find(&word).map(|x| x.as_str()).unwrap_or("");

    let number_digit = match digit {
        "one" | "eno" => "1",
        "two" | "owt" => "2",
        "three" | "eerht" => "3",
        "four" | "ruof" => "4",
        "five" | "evif" => "5",
        "six" | "xis" => "6",
        "seven" | "neves" => "7",
        "eight" | "thgie" => "8",
        "nine" | "enin" => "9",
        _ => "",
    };

    let result = re_digits.replace(word, number_digit);
    let Some(match_begin) = re_numbers.captures(&result) else {
        panic!()
    };
    return match_begin[0].to_string();
}

fn main() {
    let contents =
        BufReader::new(File::open("./input").expect("Should have been able to read the file"));

    let mut sum = 0;
    for line in contents.lines() {
        for word in line.unwrap().split_whitespace() {
            let re_digits = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine)").unwrap();
            let re_digits_rev =
                Regex::new(r"(eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)").unwrap();
            let match_first = capture(&word, re_digits);
            let match_last = capture(&word.chars().rev().collect::<String>(), re_digits_rev);
            let concat = format!("{}{}", &match_first, &match_last);
            sum += concat.parse::<i32>().unwrap();
        }
    }
    println!("{}", sum)
}
