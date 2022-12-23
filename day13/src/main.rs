use crate::List::{Cons, Nil};
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

fn part1() -> std::io::Result<usize> {
    let lines = read_lines("./input_smol.txt")?;

    let mut list1 = List::empty();
    let mut list2 = List::empty();

    let mut is_left = true;
    let mut count = 0;
    let mut idx = 0;

    for line in lines.flatten() {
        if line.is_empty() {
            is_left = true;
            list1 = List::empty();
            list2 = List::empty();
            continue;
        }

        if is_left {
            list1 = list_from(&line);
            is_left = false;
            continue;
        }

        list2 = list_from(&line);

        println!("{list1:?}");
        println!("{list2:?}");

        idx += 1;

        if List::are_ordered(&list1, &list2) {
            count += idx;
        }
    }

    Ok(count)
}

fn part2() -> std::io::Result<()> {
    let lines = read_lines("./input_smol.txt")?;

    for line in lines.flatten() {
        println!("{line}");
    }

    Ok(())
}

#[derive(Debug, Clone)]
enum List {
    Cons(Value, Box<List>),
    Nil,
}

#[derive(Debug, Clone)]
enum Value {
    Int(i32),
    NestedList(Box<List>),
}

impl List {
    fn empty() -> List {
        Nil
    }

    fn cons(val: Value, list: List) -> List {
        Cons(val, Box::new(list))
    }

    fn append(list1: List, list2: List) -> List {
        if Self::is_empty(&list1) {
            return list2;
        }
        // (cons
        //   (car ls1)
        //   (append (cdr ls1) ls2)))))
        Self::cons(
            Self::head(&list1).expect("List already checked as non-empty"),
            Self::append(Self::tail(&list1).unwrap(), list2),
        )
    }

    fn is_empty(list: &List) -> bool {
        matches!(list, Nil)
    }

    // car/first
    fn head(list: &List) -> Option<Value> {
        match list {
            Cons(Value::Int(i), _) => Some(Value::Int(*i)),
            Cons(Value::NestedList(l), _) => Some(Value::NestedList(Box::new(*l.clone()))),
            _ => None,
        }
    }

    // cdr/rest
    fn tail(list: &List) -> Option<List> {
        match list {
            Cons(_, list) => Some(*list.clone()),
            Nil => None,
        }
    }

    fn are_ordered(list1: &List, list2: &List) -> bool {
        true
    }
}

#[derive(Debug)]
enum Token {
    Open,
    Close,
    Value(i32),
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
            }
            ' ' => continue,
            ',' => {
                if !current_value.is_empty() {
                    let s: String = current_value.iter().collect();
                    tokens.push(Token::Value(s.parse::<i32>().unwrap()));
                    current_value.clear();
                }
            }
            x => current_value.push(x),
        }
    }

    let mut list = List::empty();

    for token in &tokens {
        match token {
            Token::Open => (),
            Token::Close => (),
            Token::Value(i) => list = List::append(list, List::cons(Value::Int(*i), List::empty())),
        }
    }

    list
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
