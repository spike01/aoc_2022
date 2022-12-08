use std::collections::HashSet;
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

fn part1() -> std::io::Result<usize> {
    let lines = read_lines("./input.txt")?;

    let mut columns: Vec<Vec<char>> = Vec::new();
    let mut seen: HashSet<(usize, usize)> = HashSet::new();

    for (idx, line) in lines.flatten().enumerate() {
        columns = build_columns(&line, columns);
        count_row(&line, &mut seen, idx);
    }

    for (idx, column) in columns.into_iter().enumerate() {
        count_column(column, &mut seen, idx);
    }

    Ok(seen.len())
}

fn build_columns(line: &str, mut columns: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for (j, c) in line.chars().enumerate() {
        if let Some(col) = columns.get(j) {
            let mut col = col.clone();
            col.push(c);
            columns[j] = col
        } else {
            let col = vec![c];
            columns.push(col.clone());
        }
    }
    columns
}

fn count_row(line: &str, seen: &mut HashSet<(usize, usize)>, y: usize) {
    let mut max_seen = '/'; // you don't wanna know, trust me

    let mut reverse_index = Vec::new(); // *silent screaming*
    for (x, _) in line.chars().clone().enumerate() {
        reverse_index.push(x);
    }

    for (x, c) in line.chars().clone().enumerate() {
        if max_seen >= c {
            continue;
        }
        if c > max_seen {
            max_seen = c;
        }
        seen.insert((x, y));
    }

    max_seen = '/';
    for c in line.chars().rev() {
        let rev_x = reverse_index.pop().unwrap();
        if max_seen >= c {
            continue;
        }
        if c > max_seen {
            max_seen = c;
        }
        seen.insert((rev_x, y));
    }
}

fn count_column(line: Vec<char>, seen: &mut HashSet<(usize, usize)>, x: usize) {
    let mut max_seen = '/';

    for (y, c) in line.clone().into_iter().enumerate() {
        if max_seen >= c {
            continue;
        }
        if c > max_seen {
            max_seen = c;
        }
        seen.insert((x, y));
    }

    max_seen = '/';
    for (y, c) in line.into_iter().enumerate().rev() {
        if max_seen >= c {
            continue;
        }
        if c > max_seen {
            max_seen = c;
        }
        seen.insert((x, y));
    }
}

fn part2() -> std::io::Result<usize> {
    let lines = read_lines("./input_smol.txt")?;

    let mut columns: Vec<Vec<char>> = Vec::new();
    let mut seen: HashSet<(usize, usize)> = HashSet::new();

    //println!("rows");
    for (idx, line) in lines.flatten().enumerate() {
        //println!("{line:#?}");
        columns = build_columns(&line, columns);
        count_row(&line, &mut seen, idx);
    }

    //println!("cols");
    //println!("{columns:#?}");
    for (idx, column) in columns.into_iter().enumerate() {
        //println!("{column:#?}");
        count_column(column, &mut seen, idx);
    }

    //println!("{seen:#?}");

    Ok(seen.len())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
