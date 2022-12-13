use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

const ROW_LENGTH: usize = 40;
const MAX_COUNTER: usize = 240;

fn main() -> std::io::Result<()> {
    println!("===part1===");
    println!("{}", part1()?);
    println!("===part2===");
    part2()?.iter().for_each(|line| println!("{line}"));
    Ok(())
}

fn part1() -> std::io::Result<i32> {
    let lines = read_lines("./input.txt")?;

    let mut register = 1;
    let mut counter = 0;
    let mut signal_strength = 0;

    let checkpoints = vec![20, 60, 100, 140, 180, 220];
    let mut cycles = cycles(lines);

    while counter < MAX_COUNTER {
        counter += 1; // registers are checked at end of cycle

        if checkpoints.contains(&counter) {
            let strength = register * counter as i32;
            signal_strength += strength;

            println!("{counter} * {register} = {strength}");
        }

        if let Some(tick) = cycles.pop_front() {
            register += tick;
        }
    }

    Ok(signal_strength)
}

fn part2() -> std::io::Result<Vec<String>> {
    let lines = read_lines("./input.txt")?;

    let mut register = 1;
    let mut counter = 0;

    let mut output: Vec<char> = Vec::new();
    let mut cycles = cycles(lines);

    while counter < MAX_COUNTER {
        let line_adjusted_counter = (counter % ROW_LENGTH) as i32;

        match line_adjusted_counter {
            _ if register == line_adjusted_counter => output.push('#'),
            _ if register + 1 == line_adjusted_counter => output.push('#'),
            _ if register - 1 == line_adjusted_counter => output.push('#'),
            _ => output.push('.'),
        }

        counter += 1;

        if let Some(tick) = cycles.pop_front() {
            register += tick;
        }
    }

    let printable_output = output
        .chunks(ROW_LENGTH)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect();

    Ok(printable_output)
}

fn cycles(lines: std::io::Lines<BufReader<File>>) -> VecDeque<i32> {
    let mut cycles: VecDeque<i32> = VecDeque::new();

    for line in lines.flatten() {
        match line.split_once(' ') {
            Some(("addx", x)) => {
                cycles.push_back(0);
                cycles.push_back(x.parse::<i32>().unwrap());
            }
            None => cycles.push_back(0),
            Some((_, _)) => unreachable!(),
        }
    }

    cycles
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
