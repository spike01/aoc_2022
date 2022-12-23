use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use crate::List::{Cons, Nil};

fn main() -> std::io::Result<()> {
    println!("===part1===");
    part1()?;
    println!("===part2===");
    part2()?;
    Ok(())
}

#[derive(Debug, Clone)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    fn empty() -> List {
        Nil
    }

    fn cons(val: i32, list: List) -> List {
       Cons(val, Box::new(list))
    }

    // not working yet
    fn append(list1: List, list2: List) -> List {
       if Self::is_empty(&list1) {
            return list2;
       }
       // (cons
       //   (car ls1)
       //   (append (cdr ls1) ls2)))))
       Self::cons(
           Self::head(&list1).unwrap(),
           Self::append(Self::tail(&list1).unwrap(), list2)
       )
    }

    fn is_empty(list: &List) -> bool {
       match list {
          Nil => true,
          _ => false
       }
    }

    // car
    fn head(list: &List) -> Option<i32> {
       match list {
         Cons(i, _) => Some(*i),
         Nil => None
       }
    }

    // cdr
    fn tail(list: &List) -> Option<List> {
        match list {
            Cons(_, list) => Some(*list.clone()),
            Nil => None
        }
    }

    fn last(list: List) -> Option<i32> {
        let mut current = list;
        while let Some(cons) = Self::tail(&current) {
            current = cons;
        }
        Self::head(&current)
    }
}

fn part1() -> std::io::Result<()> {
    let lines = read_lines("./input_smol.txt")?;

    let mut list1 = List::empty();
    let mut list2 = List::empty();
    let mut left = true;

    for line in lines.flatten() {

        if line.is_empty() {
            left = true;
            list1 = List::empty();
            list2 = List::empty();
            continue;
        }

        if left {
            list1 = list_from(&line);
            left = false;
            continue;
        }

        list2 = list_from(&line);

        println!("{list1:?}");
        println!("{list2:?}");
    }

    Ok(())
}

#[derive(Debug)]
enum Token {
    Open,
    Close,
    Value(i32)
}

fn list_from(line: &str) -> List {
    let mut tokens = Vec::new();
    let mut current_value = Vec::new();

    for c in line.chars() {
        match c {
            '[' => tokens.push(Token::Open),
            ']' => {
                if !current_value.is_empty() {
                    let s: String = current_value.iter().collect();
                    tokens.push(Token::Value(s.parse::<i32>().unwrap()));
                    current_value.clear();
                }
                tokens.push(Token::Close)
            },
            ' ' => continue,
            ',' => {
                if !current_value.is_empty() {
                    let s: String = current_value.iter().collect();
                    tokens.push(Token::Value(s.parse::<i32>().unwrap()));
                    current_value.clear();
                }
            },
            x => current_value.push(x)
        }
    }

    let mut list = List::empty();

    for token in &tokens {
        match token {
            Token::Open => (),
            Token::Close => (),
            Token::Value(i) => list = List::append(list, List::cons(*i, List::empty())),
        }
    }

    list
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
