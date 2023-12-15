use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use std::usize;

static mut data: Vec<(&mut Vec<char>, &mut Vec<Possible>)> = Vec::new();

fn memoize<F>(mut input: (&mut Vec<char>, &mut Vec<Possible>), function: F)-> usize
where
    F: Fn(&mut Vec<char>, &mut Vec<Possible>) -> usize,
{
    let result = data
        .iter()
        .clone()
        .find(|(memo_i, _)| memo_i == &input);

    match result {
        Some(x) => return *x.clone().1,
        None => (),
    };
    let mut new_result = function(*input);
    memo.push((&mut input, &mut new_result));
    return new_result;
}

#[derive(Debug, Clone, Copy)]
enum Possible {
    HashDot(usize),
    Hash(usize),
    Dot,
    Nothing,
}

use Possible::*;

fn fill_in(input: (&mut Vec<char>, &mut Vec<Possible>)) -> usize {
    let mut numbers = input.1;
    let mut letters = input.0;
    let tail_num = numbers.pop().unwrap_or_else(|| Nothing);
    let tail_let = letters.pop();
    return match tail_let {
        Some('#') => {
            match tail_num {
                Hash(1) => {numbers.push(Dot); fill_in((letters, numbers))},
                HashDot(x)|Hash(x) => {numbers.push(Hash(x-1)); fill_in((letters, numbers))},
                Nothing | Dot => 0,
            }
        },
        Some('.') => {
            match tail_num {
                HashDot(x) => {numbers.push(HashDot(x)); fill_in((letters, numbers))},
                Dot => fill_in((letters, numbers)),
                Nothing | Hash(_) => 0,
            }
        },
        Some('?') => {
            match tail_num {
                HashDot(x) => {
                    let mut new_values = numbers.to_vec();
                    numbers.push(Hash(x-1));
                    new_values.push(HashDot(x));
                    fill_in((letters, numbers)) * fill_in((letters, &mut new_values))
                },
                Hash(x) => {numbers.push(Hash(x-1)); fill_in((letters, numbers))},
                Dot => fill_in((letters, numbers)),
                Nothing => 0,
            }
        },
        None => {
            match tail_num {
                Dot|Nothing => 1,
                _ => 0,
            }
        }
        _ => panic!()
    };
}

fn function(file: BufReader<File>) {
    let mut multiplication = 1;
    for line in file.lines().map(|x| x.unwrap()) {
        let (letters, groups) = line.split_once(" ").unwrap();
        let numbers = groups.split(",").map(|x| HashDot(x.parse::<usize>().unwrap())).collect::<Vec<_>>();
        let score = memoize((letters, numbers), fill_in);
    }
}

fn main() {
    let start = Instant::now();
    let contents = BufReader::new(
        File::open("./src/input_test").expect("Should have been able to read the file"),
    );
    function(contents);
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration)
}
