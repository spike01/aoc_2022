use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() -> std::io::Result<()> {
    println!("===part1===");
    let max = part1("./input.txt")?;
    println!("Max: {}", max);

    println!("===part2===");
    let summed = part2("./input.txt")?;
    println!("Summed: {:?}", summed);
    Ok(())
}

fn part1(file: &str) -> std::io::Result<i32> {
    let lines = read_lines(file);

    let mut counter = 0;
    let mut max = 0;

    for line in lines?.flatten() {
        if line.is_empty() {
            if counter > max {
                max = counter
            }
            counter = 0;
            continue;
        }
        counter += line.parse::<i32>().unwrap()
    }
    if counter > max {
        max = counter
    }
    Ok(max)
}

fn part2(file: &str) -> std::io::Result<i32> {
    let lines = read_lines(file);

    let mut counter = 0;
    let mut counts = vec![];

    for line in lines?.flatten() {
        if line.is_empty() {
            counts.push(counter);
            counter = 0;
            continue;
        }
        counter += line.parse::<i32>().unwrap()
    }
    counts.push(counter);

    counts.sort_by(|a, b| b.cmp(a));
    let top_3 = counts.into_iter().take(3).collect::<Vec<i32>>();
    println!("Top 3: {:?}", top_3);
    let summed = top_3.iter().sum::<i32>();

    Ok(summed)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> std::io::Result<()> {
        assert_eq!(24000, part1("./input_smol.txt")?);
        Ok(())
    }

    #[test]
    fn test_part2() -> std::io::Result<()> {
        assert_eq!(45000, part2("./input_smol.txt")?);
        Ok(())
    }
}
