use std::collections::VecDeque;
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

#[derive(Debug)]
struct Move {
    count: u8,
    from: usize,
    to: usize,
}

fn part1() -> std::io::Result<()> {
    let lines = read_lines("./input.txt");

    let mut in_move_section = false;
    let mut first_line = true;
    let mut stack_count = 0;
    let mut stacks = vec![VecDeque::from(["unused".to_string()])];

    for line in lines?.flatten() {
        if line.is_empty() {
            in_move_section = true;
            continue;
        }
        if !in_move_section {
            let mut parts = line.split("");
            if first_line {
                let line_length = line.len();
                stack_count = (line_length / 4) + 1; // hacky but should work
                for _ in 0..stack_count {
                    stacks.push(VecDeque::new());
                }
                first_line = false;
            }
            if line.trim().starts_with('1') {
                continue;
            }

            if let Some(letter) = parts.nth(2) {
                if letter != " " {
                    stacks[1].push_front(letter.to_string());
                }
            }

            for stack in stacks.iter_mut().take(stack_count + 1).skip(2) {
                if let Some(letter) = parts.nth(3) {
                    if letter != " " {
                        stack.push_front(letter.to_string());
                    }
                }
            }
            continue;
        }

        let mut parts = line.split_whitespace();
        let move_ = Move {
            count: parts.nth(1).unwrap().parse::<u8>().unwrap(),
            from: parts.nth(1).unwrap().parse::<usize>().unwrap(),
            to: parts.nth(1).unwrap().parse::<usize>().unwrap(),
        };
        // push/pop each instruction
        for _ in 0..move_.count {
            let popped = stacks[move_.from].pop_back().unwrap();
            stacks[move_.to].push_back(popped);
        }
    }

    let result: String = stacks
        .into_iter()
        .skip(1)
        .map(|mut stack| stack.pop_back().unwrap())
        .collect();

    println!("{}", result);

    Ok(())
}

fn part2() -> std::io::Result<()> {
    let _lines = read_lines("./input_smol.txt");

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
