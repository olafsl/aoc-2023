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

fn function(file_string: String) {
    let chunks = file_string
        .split("\n\n")
        .filter(|x| x != &"")
        .collect::<Vec<_>>();
    let mut answer = 0;
    for chunk in chunks {
        let lines = chunk.lines().collect::<Vec<_>>();
        for i in 1..(lines.len()) {
            let (begin, end) = lines.split_at(i);
            let length = begin.len().min(end.len());
            let new_begin = lines.split_at(length).0;
            println!("{}, {}, {}", begin.len(), end.len(),length);
            let new_end = lines.split_at(lines.len() - length).1;
            println!("EQUAL:{:?}\nWITH  :{:?}", new_begin, new_end);
            if new_begin == new_end {
                answer += i;
                println!("EQUAL:{:?}\nWITH  :{:?}", new_begin, new_end);
                break
            }
        }
        let transposed_chunk = transpose(chunk);

    }

    println!("{}", answer)
}

fn main() {
    let start = Instant::now();
    let mut contents = BufReader::new(
        File::open("./src/input_test").expect("Should have been able to read the file"),
    );
    let mut string_file = String::new();
    contents.read_to_string(&mut string_file).unwrap();
    function(string_file);
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration)
}
