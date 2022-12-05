use std::collections::HashSet;
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

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn part1() -> std::io::Result<()> {
    let lines = read_lines("./input.txt");
    let mut total = 0;
    for line in lines?.flatten() {
        let (left, right) = line.split_at(line.len() / 2);
        let left_set: HashSet<char> = left.chars().collect();
        let right_set: HashSet<char> = right.chars().collect();
        let intersection = left_set.intersection(&right_set).next().unwrap();
        total += priority(*intersection);
    }
    println!("{}", total);

    Ok(())
}

fn part2() -> std::io::Result<()> {
    let lines = read_lines("./input.txt");
    let mut total = 0;
    let mut counter = 0;

    let mut first = HashSet::new();
    let mut second = HashSet::new();
    let mut third = HashSet::new();

    for line in lines?.flatten() {
        match counter {
            0 => {
                for c in line.chars() {
                    first.insert(c);
                }
                counter += 1;
            }
            1 => {
                for c in line.chars() {
                    second.insert(c);
                }
                counter += 1;
            }
            2 => {
                for c in line.chars() {
                    third.insert(c);
                }
                let mut temp = HashSet::new();
                for c in first.intersection(&second) {
                    temp.insert(*c);
                }
                let mut second_intersection = temp.intersection(&third);
                total += priority(*second_intersection.next().unwrap());
                counter = 0;
                first.clear();
                second.clear();
                third.clear();
            }
            _ => unreachable!(),
        }
    }

    println!("{}", total);

    Ok(())
}

fn priority(c: char) -> usize {
    ALPHABET.chars().position(|x| x == c).unwrap() + 1
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
