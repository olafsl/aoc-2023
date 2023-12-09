use std::fs::File;
use std::io::{BufReader, BufRead};
use std::time::Instant;

enum Dir {
    Left,
    Right,
}

struct Choice {
    name: String,
    left: String,
    right: String,
}

fn function(file: BufReader<File>) {
    let mut lines = file.lines();
    let dirs = lines.next().unwrap().unwrap().chars().map(|x| match x {
        'R' => Ok(Dir::Right),
        'L' => Ok(Dir::Left),
        _ => Err("Something"),
    }).cycle();

    let mut choice: Vec<Choice> = Vec::new();
    for line in lines.map(|x| x.unwrap()) {
        let (name, dirs) = line.split_once("=").unwrap();
        let (left, right) = dirs.trim_matches(|x| x =='(' || x == ')').split_once(", ").unwrap();
        choice.push(Choice { name: name.to_string(), left: left.to_string(), right: right.to_string() })
    }
    let mut curr: &str = "AAA";
    while curr != "ZZZ" {

    }


}




fn main() {
    let start = Instant::now();
    let contents = BufReader::new(File::open("./src/input").expect("Should have been able to read the file"));
    function(contents);
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration)
}