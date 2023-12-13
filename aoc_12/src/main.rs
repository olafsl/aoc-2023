use std::fs::File;
use std::io::{BufReader, BufRead};
use std::time::Instant;

fn function(file: BufReader<File>) {
    let mut multiplication = 1;
    for line in file.lines().map(|x| x.unwrap()) {
        let (data, numbers) = line.split_once(" ").unwrap();
        let groups = numbers.split(",").collect::<Vec<_>>();
        let springs = data.split(".").filter_map(|x| match x {"" => None, _ => Some(x)}).collect::<Vec<_>>();
        println!("{:?}, {:?}", groups, springs);
        let mut counter = 0;
        
        

    }    


}




fn main() {
    let start = Instant::now();
    let contents = BufReader::new(File::open("./src/input_test").expect("Should have been able to read the file"));
    function(contents);
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration)
}

