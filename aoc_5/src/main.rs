use std::fs::File;
use std::io::{prelude::*, self};
use std::time::Instant;

fn filename_to_string(s: &str) -> io::Result<String> {
    let mut file = File::open(s)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

fn string_to_ranges(string: &str) -> Vec<(isize, isize, isize)> {
    let mut ranges: Vec<(isize, isize, isize)> = Vec::new();
    for transform in string.lines().skip(1) {
        let [destination, source, range] = transform.split_whitespace().map(|x| x.parse::<isize>().unwrap()).collect::<Vec<isize>>()[..] else {panic!()};
        ranges.push((source, source+range, destination-source));
    }
    return ranges;
}


fn main() {
    let start = Instant::now();
    let Ok(file) = filename_to_string("./src/input") else { panic!("Could not open File")};
    let chunks = file.split("\n\n").collect::<Vec<&str>>();
    let seeds = chunks[0].split(" ").skip(1).map(|x| x.parse::<isize>().unwrap()).collect::<Vec<isize>>();
    let transforms = chunks[1..].iter().map(|x| string_to_ranges(x)).collect::<Vec<Vec<(isize, isize, isize)>>>();
    let mut lowest_loc= isize::MAX;

    for seed in seeds {
        let mut val = seed;
        for maps in &transforms {
            for map in maps {
                if map.0 <= val && val < map.1 {
                    val += map.2;
                    break
                }
            }
        }
        if val <= lowest_loc {
            lowest_loc = val;
        }
    }
    println!("{lowest_loc}");
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration)

}
