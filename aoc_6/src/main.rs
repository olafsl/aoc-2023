use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn _function_a(file: BufReader<File>) {
    let parsed_data = file
        .lines()
        .map(|x| x.unwrap())
        .map(|x| {
            x.split_ascii_whitespace()
                .skip(1)
                .map(|y| y.parse::<isize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let data = parsed_data[0].iter().zip(parsed_data[1].iter());
    for (time, dist) in data {
        let records = (0..*time)
            .filter(|speed| speed * (time - speed) > *dist)
            .count();
        println!("{records}")
    }
}

fn function(file: BufReader<File>) {
    let parsed_data = file
        .lines()
        .map(|x| x.unwrap())
        .map(|x| {
            x.split_ascii_whitespace()
                .skip(1)
                .collect::<Vec<_>>()
                .concat()
                .replace(" ", "")
                .parse::<isize>()
                .unwrap()
        })
        .collect::<Vec<_>>();
    let records = (0..parsed_data[0])
        .filter(|speed| *speed * (parsed_data[0] - *speed) > parsed_data[1])
        .count();
    println!("{records:?}")
}

fn main() {
    let start = Instant::now();
    let contents = BufReader::new(
        File::open("./src/input").expect("Should have been able to read the file"),
    );
    function(contents);
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration)
}
