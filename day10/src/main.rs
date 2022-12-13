use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

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
            }
            None => cycles.push_back(0),
            Some((_, _)) => unreachable!(),
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
    let lines = read_lines("./input.txt")?;

    let mut register: i32 = 1;
    let mut counter: usize = 0;

    let mut output: Vec<char> = Vec::new();
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


    while counter <= 240 {
        let sprite_position = vec![register - 1, register, register + 1];

        match counter {
            _ if sprite_position.contains(&(counter as i32 % 40)) => output.push('#'),
            _ => output.push('.')
        }

        counter += 1;

        if let Some(tick) = cycles.pop_front() {
            register += tick;
        }
    }

    let line_1: String = output[0..39].into_iter().collect();
    let line_2: String = output[40..79].into_iter().collect();
    let line_3: String = output[80..119].into_iter().collect();
    let line_4: String = output[120..159].into_iter().collect();
    let line_5: String = output[160..199].into_iter().collect();
    let line_6: String = output[200..239].into_iter().collect();

    println!("{}", line_1);
    println!("{}", line_2);
    println!("{}", line_3);
    println!("{}", line_4);
    println!("{}", line_5);
    println!("{}", line_6);
    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
