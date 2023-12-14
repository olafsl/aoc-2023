use std::fs::File;
use std::io::{BufReader, Read};
use std::time::Instant;

fn transpose(file: &str) -> String {
    println!("{:?}", file);
    let lines = file.lines().collect::<Vec<_>>();
    let vec = lines
        .iter()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let rows = vec.len();
    let cols = vec[0].len();
    let transposed = (0..cols)
        .map(|col| (0..rows).map(|row| vec[row][col]).collect::<String>())
        .collect::<Vec<_>>()
        .join("\n");
    return transposed;
}

fn equal_check(chunk: &str) -> Option<usize> {
    let lines = chunk.lines().map(|x| x.to_string()).collect::<Vec<_>>();
    for i in 1..(lines.len()) {
        let (head, tail) = lines.split_at(i);
        let (start, end) = if i * 2 < lines.len() {
            (head, tail.get(..i).unwrap())
        } else {
            (head.get(head.len() - (lines.len() - i)..).unwrap(), tail)
        };
        let (start, mut end) = (Vec::from(start), Vec::from(end));
        end.reverse();

        if start == end {
            return Some(i);
        }
    }
    return None;
}

fn function(file_string: String) {
    let chunks = file_string
        .split("\n\n")
        .filter(|x| x != &"")
        .collect::<Vec<_>>();
    let mut answer = 0;
    for chunk in chunks {
        answer += match equal_check(chunk) {
            Some(x) => x*100,
            None => 0,
        };
        let transposed_chunk = &transpose(chunk);
        answer += match equal_check(transposed_chunk) {
            Some(x) => x,
            None => 0,
        };
    }

    println!("{}", answer)
}

fn main() {
    let start = Instant::now();
    let mut contents = BufReader::new(
        File::open("./src/input").expect("Should have been able to read the file"),
    );
    let mut string_file = String::new();
    contents.read_to_string(&mut string_file).unwrap();
    function(string_file);
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration)
}
