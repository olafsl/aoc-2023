use std::fs::File;
use std::io::{BufReader, BufRead};
use std::time::Instant;

enum _Direction {
    Vertical,
    Horizontal,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

fn function(file: BufReader<File>) {
    file.lines().map(|x| x.unwrap()).reduce(|x, a| (x + &a));


}




fn main() {
    let start = Instant::now();
    let contents = BufReader::new(File::open("./src/input").expect("Should have been able to read the file"));
    function(contents);
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration)
}
