use std::fs::File;
use std::io::{BufReader, BufRead};
use std::time::Instant;
use regex::{Regex, RegexBuilder};


fn function(file: String, row_length: usize, col_length: usize) {
    let multiplication_factor = 1000000;
    let universe = file.clone();
    let regex_string = "(?mU:".to_owned() + &format!("(?:(\\.)[\\n\\.#A]{{{row_length}}})").repeat(col_length-1).to_owned() + "(\\.))";
    let col_re = Regex::new(&regex_string).unwrap();

    let mut expanded_universe = universe.to_string();

    loop {
        let cols_clone = expanded_universe.clone();
        let captures = col_re.captures(&cols_clone);

        let new_matches = match captures {
            None => break,
            Some(x) => x,
        };

        let hits_iterator = new_matches.iter().skip(1).map(|x| x.unwrap().range());
        hits_iterator.for_each(|range| expanded_universe.replace_range(range, "A"));
    }
    
    expanded_universe = expanded_universe.replace('A', &".".repeat(multiplication_factor));

    let mut expanded_rows = Vec::new();

    let mut galaxies = Vec::new();
    for (i, line) in expanded_universe.lines().enumerate() {
        let line_re = RegexBuilder::new(&format!("(?m)^(\\.*)$")).build().unwrap();
        let row_expanded = line_re.find(line);
        match row_expanded {
            None => (),
            Some(_) => expanded_rows.push(i)
        }

        println!("{:?}", expanded_rows);
        let expanded_rows_before = expanded_rows.iter().filter(|x| x <= &&i).count();
        let line_galaxies = line.match_indices("#").map(|x| (i + expanded_rows_before * (multiplication_factor -1), x.0)).collect::<Vec<_>>();
        match line_galaxies.len() {
            0 => continue,
            _ => (),
        };
        for thing in line_galaxies {
            galaxies.push((thing.0 as isize, thing.1 as isize));
        }
    }
    let mut sum = 0;
    let galaxies_clone = galaxies.clone();
    for galaxy_i in galaxies.clone().iter() {
        for galaxy_j in &galaxies_clone {
            sum += isize::abs_diff(galaxy_i.0, galaxy_j.0) + isize::abs_diff(galaxy_i.1, galaxy_j.1)
        }

    }
    sum /= 2;
    println!("{:?}", sum)
}



fn main() {
    let start = Instant::now();
    let contents = BufReader::new(File::open("./src/input").expect("Should have been able to read the file"));
    let file_collection = contents.lines().map(|x| x.unwrap()).collect::<Vec<_>>();
    let line_length = file_collection.first().unwrap().len();
    let file_length = file_collection.len();
    let string = file_collection.join("\n");
    function(string, line_length, file_length);
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration)
}
