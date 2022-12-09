use std::collections::{HashMap, HashSet};
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

fn part2() -> std::io::Result<usize> {
    let lines = read_lines("./input.txt")?;

    let mut columns: Vec<Vec<char>> = Vec::new();
    let mut scenic_scores: HashMap<(usize, usize), usize> = HashMap::new();

    for line in lines.flatten() {
        columns = build_columns(&line, columns);
    }

    for (y, col) in columns.iter().enumerate() {
        for (x, _) in col.iter().enumerate() {
            let left = look(Direction::Left, x, y, &columns);
            let right = look(Direction::Right, x, y, &columns);
            let up = look(Direction::Up, x, y, &columns);
            let down = look(Direction::Down, x, y, &columns);

            let score = left * right * up * down;

            *scenic_scores.entry((x, y)).or_insert(0) += score;
        }
    }

    Ok(*scenic_scores.values().max().expect("a solution exists"))
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn look(direction: Direction, x: usize, y: usize, columns: &Vec<Vec<char>>) -> usize {
    let mut score: usize = 0;
    let height = columns[x][y];

    match direction {
        Direction::Left => {
            for i in (0..x).rev() {
                score += 1;
                let next_height = columns[i][y];
                if next_height >= height {
                    break;
                }
            }
            score
        }
        Direction::Right => {
            for col in columns.iter().skip(x + 1) {
                score += 1;
                let next_height = col[y];
                if next_height >= height {
                    break;
                }
            }
            score
        }
        Direction::Up => {
            for i in (0..y).rev() {
                score += 1;
                let next_height = columns[x][i];
                if next_height >= height {
                    break;
                }
            }
            score
        }
        Direction::Down => {
            for i in (y + 1)..columns.len() {
                score += 1;
                let next_height = columns[x][i];
                if next_height >= height {
                    break;
                }
            }
            score
        }
    }
}

fn build_columns(line: &str, mut columns: Vec<Vec<char>>) -> Vec<Vec<char>> {
    line.chars().enumerate().for_each(|(j, c)| {
        match columns.get(j) {
            Some(col) => {
                let mut col = col.clone(); // blah, can't get the borrow checker to comply
                col.push(c);
                columns[j] = col
            }
            None => columns.push(vec![c]),
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
        let x = reverse_index.pop().unwrap();
        if max_seen >= c {
            continue;
        }
        if c > max_seen {
            max_seen = c;
        }
        seen.insert((x, y));
    }
}

fn count_column(col: Vec<char>, seen: &mut HashSet<(usize, usize)>, x: usize) {
    let mut max_seen = '/';

    for (y, c) in col.iter().enumerate() {
        if max_seen >= *c {
            continue;
        }
        if *c > max_seen {
            max_seen = *c;
        }
        seen.insert((x, y));
    }

    max_seen = '/';
    for (y, c) in col.iter().enumerate().rev() {
        if max_seen >= *c {
            continue;
        }
        if *c > max_seen {
            max_seen = *c;
        }
        seen.insert((x, y));
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
