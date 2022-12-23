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

    fn cons(self, val: i32) -> List {
       Cons(val, Box::new(self))
    }

    // not working yet
    fn append(&self, list: List) -> List {
       if self.is_empty() {
            return list;
       }
       // (cons (car ls1) (append (cdr ls1) ls2)))))
       Self::append(&self.tail().unwrap(), list).cons(self.head().unwrap())
    }

    fn is_empty(&self) -> bool {
       match self {
          Nil => true,
          _ => false
       }
    }

    // car
    fn head(&self) -> Option<i32> {
       match self {
         Cons(i, _) => Some(*i),
         Nil => None
       }
    }

    // cdr
    fn tail(&self) -> Option<List> {
        match self {
            Cons(_, list) => Some(*list.clone()),
            Nil => None
        }
    }

    fn last(self) -> Option<i32> {
        let mut current = self;
        while let Some(cons) = current.tail() {
            current = cons;
        }
        current.head()
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
            Token::Value(i) => list = list.append(List::empty().cons(*i)),
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
