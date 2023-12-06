use std::fs::File;
use std::io::{BufReader, BufRead};
use std::time::Instant;

fn function(file: BufReader<File>) {
    for line in file.lines() {
        todo!();
    }    


}




fn main() {
    let start = Instant::now();
    let contents = BufReader::new(File::open("./src/input").expect("Should have been able to read the file"));
    function(contents);
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration)
}
