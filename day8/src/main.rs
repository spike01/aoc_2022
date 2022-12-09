use std::collections::{HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::iter::Rev;
use std::ops::Range;
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
    let mut scenic_scores: Vec<usize> = Vec::new();

    for line in lines.flatten() {
        columns = build_columns(&line, columns);
    }

    for (y, c) in columns.iter().enumerate() {
        for (x, _) in c.iter().enumerate() {
            let max = columns.len();
            let height = columns[x][y];

            let p = Point { x, y, height, max };

            let left = p.look(Direction::Left, &columns);
            let right = p.look(Direction::Right, &columns);
            let up = p.look(Direction::Up, &columns);
            let down = p.look(Direction::Down, &columns);

            let score = left * right * up * down;

            scenic_scores.push(score);
        }
    }

    Ok(*scenic_scores.iter().max().expect("a solution should exist"))
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, PartialEq)]
struct Point {
    x: usize,
    y: usize,
    height: char,
    max: usize,
}

impl Point {
    fn left(&self) -> Rev<Range<usize>> {
        (0..self.x).rev()
    }

    fn right(&self) -> Range<usize> {
        (self.x + 1)..self.max
    }

    fn up(&self) -> Rev<Range<usize>> {
        (0..self.y).rev()
    }

    fn down(&self) -> Range<usize> {
        (self.y + 1)..self.max
    }

    fn look(&self, direction: Direction, columns: &[Vec<char>]) -> usize {
        let mut score: usize = 0;

        match direction {
            Direction::Left => {
                self.left()
                    .take_while(|i| { score += 1; self.height > columns[*i][self.y] })
                    .count();
                score
            }
            Direction::Right => {
                self.right()
                    .take_while(|i| { score += 1; self.height > columns[*i][self.y] })
                    .count();
                score
            }
            Direction::Up => {
                self.up()
                    .take_while(|i| { score += 1; self.height > columns[self.x][*i] })
                    .count();
                score
            }
            Direction::Down => {
                self.down()
                    .take_while(|i| { score += 1; self.height > columns[self.x][*i] })
                    .count();
                score
            }
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
    let mut max_height = '/';

    // *silent screaming* - you can't call .rev().enumerate() on chars() because "the trait
    // `ExactSizeIterator` is not implemented for `Chars<'_>`" ;.;
    let mut reverse_index = Vec::new();

    for (x, current_height) in line.chars().enumerate() {
        reverse_index.push(x);
        if max_height >= current_height {
            continue;
        }
        if current_height > max_height {
            max_height = current_height;
        }
        seen.insert((x, y));
    }

    max_height = '/';
    for current_height in line.chars().rev() {
        let x = reverse_index.pop().unwrap();
        if max_height >= current_height {
            continue;
        }
        if current_height > max_height {
            max_height = current_height;
        }
        seen.insert((x, y));
    }
}

fn count_column(col: Vec<char>, seen: &mut HashSet<(usize, usize)>, x: usize) {
    let mut max_height = '/';

    for (y, current_height) in col.iter().enumerate() {
        if max_height >= *current_height {
            continue;
        }
        if *current_height > max_height {
            max_height = *current_height;
        }
        seen.insert((x, y));
    }

    max_height = '/';
    for (y, current_height) in col.iter().enumerate().rev() {
        if max_height >= *current_height {
            continue;
        }
        if *current_height > max_height {
            max_height = *current_height;
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
