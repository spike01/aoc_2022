use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() -> std::io::Result<()> {
    part2()
}

fn part1() -> std::io::Result<()> {
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

fn part2() -> std::io::Result<()> {
    let lines = read_lines("./input.txt");

    let mut counter = 0;
    let mut counts = vec![];

    for line in lines?.flatten() {
        if line == *"".to_string() {
            counts.push(counter);
            counter = 0;
            continue;
        }
        counter += line.parse::<i32>().unwrap()
    }
    counts.push(counter);

    counts.sort_by(|a, b| b.cmp(a));
    let top_3 = counts.into_iter().take(3).collect::<Vec<i32>>();
    println!("Top 3: {:?}", top_3);
    println!("Summed: {:?}", top_3.iter().sum::<i32>());

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
