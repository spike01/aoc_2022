use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() -> std::io::Result<()> {
    let lines = read_lines("./input.txt");

    let mut counter = 0;
    let mut max = 0;

    for line in lines?.flatten() {
        if line == *"".to_string() {
            if counter > max {
                max = counter
            }
            counter = 0;
            continue;
        }
        counter += line.parse::<i32>().unwrap()
    }
    if counter > max {
        max = counter
    }
    println!("Max: {}", max);
    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
