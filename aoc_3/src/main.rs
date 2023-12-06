use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;
use std::time::Instant;

fn function(file: BufReader<File>) {
    let re_tokens = Regex::new(r"(?m)[^\d\.]").unwrap();
    let re_numbers = Regex::new(r"(?m)(\d+)").unwrap();

    let mut tokens: Vec<Vec<usize>> = Vec::new();
    let mut nrs: Vec<Vec<(Range<usize>, usize)>> = Vec::new();

    tokens.push(Vec::new());
    for line in file.lines() {
        let x = line.unwrap();
        let token_locs = re_tokens
            .find_iter(&x)
            .map(|x| x.start())
            .collect::<Vec<_>>(); 
        tokens.push(token_locs);

        let numbers = re_numbers
            .find_iter(&x)
            .map(|x| (x.range(), x.as_str().parse::<usize>().unwrap()))
            .collect::<Vec<_>>();
        nrs.push(numbers);
    }
    tokens.push(Vec::new());

    let grouped = nrs.iter().zip(tokens.windows(3));
    let mut sum = 0;

    for (values, windows) in grouped {
        for value in values {
            if windows.into_iter().flatten().any(|x| {
                value.0.contains(&(x - 1)) || value.0.contains(&(x + 1)) || value.0.contains(x)
            }) {
                sum += value.1
            }
        }
    }

    println!("{:?}", sum);
}

fn main() {
    let start = Instant::now();
    let contents =
        BufReader::new(File::open("./src/input").expect("Should have been able to read the file"));
    function(contents);
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration)
}
