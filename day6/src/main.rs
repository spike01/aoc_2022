use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() -> std::io::Result<()> {
    println!("===part1===");
    println!("{:?}", part1()?);
    println!("===part2===");
    println!("{:?}", part2()?);
    Ok(())
}

fn part1() -> std::io::Result<Vec<usize>> {
    let lines = read_lines("./input.txt");

    let mut positions = Vec::new();
    for line in lines?.flatten() {
        positions.push(find_marker_position(&line, 4).unwrap());
    }

    Ok(positions)
}

fn part2() -> std::io::Result<Vec<usize>> {
    let lines = read_lines("./input.txt");

    let mut positions = Vec::new();
    for line in lines?.flatten() {
        positions.push(find_marker_position(&line, 14).unwrap());
    }

    Ok(positions)
}

fn find_marker_position(line: &str, window_size: usize) -> Option<usize> {
    let mut deque = VecDeque::new();
    for (i, c) in line.chars().enumerate() {
        if deque.len() == window_size {
            deque.pop_front();
        }
        deque.push_back(c);
        let set: HashSet<char> = deque.clone().into_iter().collect();
        if set.len() == window_size {
            return Some(i + 1); // chars zero indexed
        }
    }
    None
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
