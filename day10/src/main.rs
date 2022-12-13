use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::collections::VecDeque;

fn main() -> std::io::Result<()> {
    println!("===part1===");
    println!("{}", part1()?);
    println!("===part2===");
    part2()?;
    Ok(())
}

fn part1() -> std::io::Result<i32> {
    let lines = read_lines("./input.txt")?;

    let mut register = 1;
    let mut counter = 1;
    let mut signal_strength = 0;

    let checkpoints = vec![20, 60, 100, 140, 180, 220];
    let mut cycles: VecDeque<i32> = VecDeque::new();

    for line in lines.flatten() {
        match line.split_once(' ') {
            Some(("addx", x)) => {
                cycles.push_back(0);
                cycles.push_back(x.parse::<i32>().unwrap());
            },
            None => cycles.push_back(0),
            Some((_, _)) => unreachable!()
        }
    }

    cycles.pop_front();

    while counter <= 240 {
        counter += 1;

        if checkpoints.contains(&counter) {
            println!("{counter} * {register} = {}", register * counter as i32);
            signal_strength += register * counter as i32;
        }

        if let Some(tick) = cycles.pop_front() {
            register += tick;
        }
    }

    Ok(signal_strength)
}

fn part2() -> std::io::Result<()> {
    let lines = read_lines("./input_smol.txt")?;

    for line in lines.flatten() {
        println!("{line}");
    }

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
