use num::integer::Integer;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

#[derive(Debug)]
enum Dir {
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Choice {
    name: String,
    left: String,
    right: String,
}

impl Choice {
    fn dir(&self, dir: &Dir) -> &str {
        match dir {
            Dir::Left => return &self.left,
            Dir::Right => return &self.right,
        }
    }
}

fn function(file: BufReader<File>) {
    let mut lines = file.lines();
    let first_line = lines.next().unwrap().unwrap();
    let dirs = first_line
        .chars()
        .map(|x| match x {
            'R' => Ok(Dir::Right),
            'L' => Ok(Dir::Left),
            _ => Err("Something"),
        })
        .cycle();
    lines.next();

    let mut choice: Vec<Choice> = Vec::new();
    for line in lines.map(|x| x.unwrap()) {
        let (name, dirs) = line.split_once("=").unwrap();
        let (left, right) = dirs
            .trim()
            .trim_matches(|x| x == '(' || x == ')')
            .split_once(", ")
            .unwrap();
        choice.push(Choice {
            name: name.trim().to_string(),
            left: left.to_string(),
            right: right.to_string(),
        })
    }

    let starts = choice
        .iter()
        .filter_map(|choice| match choice.name.chars().last() {
            Some('A') => Some(choice),
            _ => None,
        })
        .collect::<Vec<_>>();

    let mut result = 1;
    for start in starts {
        let mut curr_dirs = dirs.clone();
        let mut curr = start.name.clone();
        let mut steps: usize = 0;
        while &curr.chars().last().unwrap() != &'Z' {
            let bla = choice.iter().find(|y| y.name == curr.to_string()).unwrap();
            curr = bla.dir(&curr_dirs.next().unwrap().unwrap()).to_string();
            steps += 1;
        }
        result = result.lcm(&steps);
    }

    println!("{:?}", result);
}

fn main() {
    let start = Instant::now();
    let contents =
        BufReader::new(File::open("./src/input").expect("Should have been able to read the file"));
    function(contents);
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration)
}
