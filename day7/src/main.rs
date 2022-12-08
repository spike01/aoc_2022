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

fn part1() -> std::io::Result<i32> {
    let lines = read_lines("./input.txt")?;
    let files = directory_sizes(lines);

    let sum_at_most_100_000 = files.iter().filter(|&s| *s <= 100_000).sum();

    Ok(sum_at_most_100_000)
}

fn part2() -> std::io::Result<i32> {
    let lines = read_lines("./input.txt")?;
    let files = directory_sizes(lines);

    let total_space = 70_000_000;
    let space_needed = 30_000_000;
    let available_space = total_space - files[0]; // root

    let deletion_candidate = files
        .iter()
        .filter(|&s| (available_space + *s) >= space_needed)
        .min()
        .expect("Expect an answer to exist");

    Ok(*deletion_candidate)
}

fn directory_sizes(lines: std::io::Lines<BufReader<File>>) -> Vec<i32> {
    let mut size_stack: Vec<i32> = Vec::new();
    let mut file_sizes: Vec<i32> = Vec::new();

    for line in lines.flatten() {
        if line.starts_with('$') {
            let command = &line[2..4];
            match command {
                "cd" => {
                    let dest = &line[5..];
                    if dest == ".." {
                        if let Some(size) = size_stack.pop() {
                            file_sizes.push(size);
                        }
                        continue;
                    }
                    size_stack.push(0);
                }
                "ls" => continue,
                _ => unreachable!(),
            }
            continue;
        }

        if let Some((info, _)) = line.split_once(' ') {
            if info != "dir" {
                for f in size_stack.iter_mut() {
                    *f += info.parse::<i32>().expect("should fit!");
                }
            }
        }
    }
    // append this way around because size_stack[0] is the root file_size
    size_stack.append(&mut file_sizes);

    size_stack
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
