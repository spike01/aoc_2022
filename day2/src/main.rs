use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() -> std::io::Result<()> {
    println!("===part1===");
    part1()?;
    println!("===part2===");
    part2()?;
    Ok(())
}

fn part1() -> std::io::Result<()> {
    let lines = read_lines("./input.txt");
    let mut total = 0;
    for line in lines?.flatten() {
        match line.as_str() {
            "A X" => total += 1 + 3, // println!("Rock Rock"),
            "A Y" => total += 2 + 6, // println!("Rock Paper"),
            "A Z" => total += 3 + 0, // println!("Rock Scissors"),
            "B X" => total += 1 + 0, // println!("Paper Rock"),
            "B Y" => total += 2 + 3, // println!("Paper Paper"),
            "B Z" => total += 3 + 6, // println!("Paper Scissors"),
            "C X" => total += 1 + 6, // println!("Scissors Rock"),
            "C Y" => total += 2 + 0, // println!("Scissors Paper"),
            "C Z" => total += 3 + 3, // println!("Scissors Scissors"),
            _ => println!("Invalid move!"),
        }
    }
    println!("Total score: {}", total);

    Ok(())
}

fn part2() -> std::io::Result<()> {
    let lines = read_lines("./input.txt");

    let mut total = 0;
    for line in lines?.flatten() {
        match line.as_str() {
            "A X" => total += 0 + 3, // println!("Lose with Scissors"),
            "A Y" => total += 3 + 1, // println!("Draw with Rock"),
            "A Z" => total += 6 + 2, // println!("Win with Paper"),
            "B X" => total += 0 + 1, // println!("Lose with Rock"),
            "B Y" => total += 3 + 2, // println!("Draw with Paper"),
            "B Z" => total += 6 + 3, // println!("Win with Scissors"),
            "C X" => total += 0 + 2, // println!("Lose with Paper"),
            "C Y" => total += 3 + 3, // println!("Draw with Scissors"),
            "C Z" => total += 6 + 1, // println!("Win with Rock"),
            _ => println!("Invalid move!"),
        }
    }
    println!("Total score: {}", total);

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
