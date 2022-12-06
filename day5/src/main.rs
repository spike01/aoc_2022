use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() -> std::io::Result<()> {
    println!("===part1===");
    println!("{}", part1()?);
    println!("===part2===");
    println!("{}", part2()?);
    Ok(())
}

fn part1() -> std::io::Result<String> {
    let lines = read_lines("./input.txt");

    let mut create_stacks = true;
    let mut first_line = true;
    let mut stacks = vec![VecDeque::from(["unused".to_string()])]; // allows us to treat the stacks
                                                                   // as 1-indexed
    for line in lines?.flatten() {
        if create_stacks {
            (create_stacks, first_line, stacks) = setup(&line, stacks, first_line);
            continue;
        }

        let (count, from, to) = parse(&line);

        for _ in 0..count {
            let popped = stacks[from].pop_back().unwrap();
            stacks[to].push_back(popped);
        }
    }

    Ok(pop_stacks(stacks))
}


fn part2() -> std::io::Result<String> {
    let lines = read_lines("./input.txt");

    let mut create_stacks = true;
    let mut first_line = true;
    let mut stacks = vec![VecDeque::from(["unused".to_string()])]; // allows us to treat the stacks
                                                                   // as 1-indexed
    for line in lines?.flatten() {
        if create_stacks {
            (create_stacks, first_line, stacks) = setup(&line, stacks, first_line);
            continue;
        }

        let (count, from, to) = parse(&line);

        let mut chunk = Vec::new();
        for _ in 0..count {
            let popped = stacks[from].pop_back().unwrap();
            chunk.push(popped);
        }

        while !chunk.is_empty() {
            let popped = chunk.pop().unwrap();
            stacks[to].push_back(popped);
        }
    }

    Ok(pop_stacks(stacks))
}

fn setup(line: &str, mut stacks: Vec<VecDeque<String>>, first_line: bool) -> (bool, bool, Vec<VecDeque<String>>) {
    if line.is_empty() {
        return (false, false, stacks);
    }

    // Ignore the " 1 2 3 ... n " line
    if line.trim().starts_with('1') {
        return (true, false, stacks);
    }

    let mut parts = line.split("");
    if first_line {
        let stack_count = (line.len() / 4) + 1; // hacky but should work
        for _ in 0..stack_count {
            stacks.push(VecDeque::new());
        }
    }

    //                                  012
    // Handle the first column - nth(2) "[X"
    // Still not sure where the extra empty string comes from...
    if let Some(letter) = parts.nth(2) {
        if letter != " " {
            stacks[1].push_front(letter.to_string());
        }
    }

    // Handle following columns - skip(2) the "unused" and first column
    for stack in stacks.iter_mut().skip(2) {
        //                                                     0123
        // then nth(3) - each following column is of the form "] [X"
        if let Some(letter) = parts.nth(3) {
            if letter != " " {
                stack.push_front(letter.to_string());
            }
        }
    }
    (true, false, stacks)
}

fn parse(line: &str) -> (u8, usize, usize) {
    let mut parts = line.split_whitespace();

    // nth: 0    1       0    1      0  1
    //      move <count> from <from> to <to>
    let count = parts.nth(1).unwrap().parse::<u8>().unwrap();
    let from = parts.nth(1).unwrap().parse::<usize>().unwrap();
    let to = parts.nth(1).unwrap().parse::<usize>().unwrap();
    (count, from, to)
}

fn pop_stacks(stacks: Vec<VecDeque<String>>) -> String {
    stacks
        .into_iter()
        .skip(1) // the "unused" VecDeque
        .map(|mut stack| stack.pop_back().unwrap())
        .collect()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
