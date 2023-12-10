use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn next<'a>(series: &'a Vec<isize>) -> isize {
    let next_series = series
        .windows(2)
        .map(|slice| slice.last().unwrap() - slice.first().unwrap())
        .collect::<Vec<_>>();
    return match next_series.clone().into_iter().all(|x| x == 0) {
        true => *series.first().unwrap(),
        false => series.first().unwrap() - next(&next_series.clone())
    };
}

fn function(file: BufReader<File>) {
    let sum = file.lines().map(|x| x.unwrap()).fold(0, |acc, x| {
        acc + next(
            &x.split_ascii_whitespace()
                .map(|x| x.parse::<isize>().unwrap())
                .collect::<Vec<_>>(),
        )
    });
    println!("{sum}");
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
