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
    line.chars().enumerate().for_each(|(j, c)| {
        match columns.get(j) {
            Some(col) => {
                let mut col = col.clone(); // blah
                col.push(c);
                columns[j] = col
            },
            None => columns.push(vec![c].clone())
        }
    });
    columns
}

fn count_row(line: &str, seen: &mut HashSet<(usize, usize)>, y: usize) {
    // you don't wanna know, trust me (ok fine it's the char where '/' < '0' == true, I was too
    // lazy to parse everything into numbers myself)
    let mut max_seen = '/';

    // *silent screaming* - you can't call .rev().enumerate() on chars() because "the trait
    // `ExactSizeIterator` is not implemented for `Chars<'_>`" ;.;
    let mut reverse_index = Vec::new();

    for (x, c) in line.chars().enumerate() {
        reverse_index.push(x);
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

    for (y, c) in line.iter().enumerate() {
        if max_seen >= *c {
            continue;
        }
        if *c > max_seen {
            max_seen = *c;
        }
        seen.insert((x, y));
    }

    max_seen = '/';
    for (y, c) in line.iter().enumerate().rev() {
        if max_seen >= *c {
            continue;
        }
        if *c > max_seen {
            max_seen = *c;
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
