use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::collections::HashSet;

fn main() -> std::io::Result<()> {
    println!("===part1===");
    println!("{}", part1()?);
    println!("===part2===");
    println!("{}", part2()?);
    Ok(())
}

fn part1() -> std::io::Result<i32> {
    let lines = read_lines("./input.txt");

    let mut count = 0;

    for line in lines?.flatten() {
        let mut parts = line.split(',');

        let (l_min, l_max) = extract(parts.next().unwrap());
        let (r_min, r_max) = extract(parts.next().unwrap());

        if is_fully_contained(l_min, l_max, r_min, r_max) {
            count += 1
        }
    }

    Ok(count)
}

fn part2() -> std::io::Result<i32> {
    let lines = read_lines("./input.txt");

    let mut count = 0;

    for line in lines?.flatten() {
        let mut parts = line.split(',');

        let (l_min, l_max) = extract(parts.next().unwrap());
        let (r_min, r_max) = extract(parts.next().unwrap());

        if is_overlap(l_min, l_max, r_min, r_max)  {
            count += 1
        }
    }

    Ok(count)
}

fn extract(part: &str) -> (u8, u8) {
    let mut parts = part.split('-').map(|p| p.parse::<u8>().unwrap());
    (parts.next().unwrap(), parts.next().unwrap())
}

fn is_fully_contained(l_min: u8, l_max: u8, r_min: u8, r_max: u8) -> bool {
    // first range contains second
    (l_min <= r_min && l_max >= r_max)
        ||
    // second range contains first
    (r_min <= l_min && r_max >= l_max)
}

fn is_overlap(l_min: u8, l_max: u8, r_min: u8, r_max: u8) -> bool {
    let left: HashSet<u8> = (l_min..=l_max).collect();
    let right: HashSet<u8> = (r_min..=r_max).collect();
    !left.is_disjoint(&right)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
