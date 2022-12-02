use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

const DRAW: i32 = 3;
const WIN: i32 = 6;
const LOSE: i32 = 0;

const ROCK: i32 = 1;
const PAPER: i32 = 2;
const SCISSORS: i32 = 3;

fn main() -> std::io::Result<()> {
    println!("===part1===");
    println!("Total score: {}", part1()?);
    println!("===part2===");
    println!("Total score: {}", part2()?);
    Ok(())
}

fn part1() -> std::io::Result<i32> {
    let lines = read_lines("./input.txt");

    let mut total = 0;

    for line in lines?.flatten() {
        total += match line.as_str() {
            "A X" => DRAW + ROCK,
            "A Y" => WIN + PAPER,
            "A Z" => LOSE + SCISSORS,
            "B X" => LOSE + ROCK,
            "B Y" => DRAW + PAPER,
            "B Z" => WIN + SCISSORS,
            "C X" => WIN + ROCK,
            "C Y" => LOSE + PAPER,
            "C Z" => DRAW + SCISSORS,
            _ => unreachable!(),
        }
    }
    Ok(total)
}

fn part2() -> std::io::Result<i32> {
    let lines = read_lines("./input.txt");

    let mut total = 0;

    for line in lines?.flatten() {
        total += match line.as_str() {
            "A X" => LOSE + SCISSORS,
            "A Y" => DRAW + ROCK,
            "A Z" => WIN + PAPER,
            "B X" => LOSE + ROCK,
            "B Y" => DRAW + PAPER,
            "B Z" => WIN + SCISSORS,
            "C X" => LOSE + PAPER,
            "C Y" => DRAW + SCISSORS,
            "C Z" => WIN + ROCK,
            _ => unreachable!(),
        }
    }
    Ok(total)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
