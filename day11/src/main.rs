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
enum Sign {
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug)]
enum Operand {
    Old,
    Number(i32),
}

#[derive(Debug)]
struct Operation {
    sign: Sign,
    operand: Operand,
}

enum MonkeyField {
    Name,
    Items,
    Operation,
    Test,
    IfTrue,
    IfFalse,
}

#[derive(Debug)]
struct Monkey {
    name: usize,
    items: Vec<i32>,
    operation: Operation,
    test: i32,
    if_true: usize,
    if_false: usize,
}

fn part1() -> std::io::Result<()> {
    let lines = read_lines("./input_smol.txt")?;

    let mut counter = 0;
    let mut monkeys: Vec<Monkey> = Vec::new();

    let mut name = 0;
    let mut items: Vec<i32> = Vec::new();
    let mut operation = Operation {
        sign: Sign::Plus,
        operand: Operand::Old,
    };
    let mut test = 0;
    let mut if_true = 0;
    let mut if_false = 0;

    for line in lines.flatten() {
        match counter {
            _ if counter == MonkeyField::Name as usize => {
                name = monkeys.len();
            }
            _ if counter == MonkeyField::Items as usize => {
                if let Some((_, end)) = line.split_once(": ") {
                    end.split(", ")
                        .for_each(|i| items.push(i.parse::<i32>().unwrap()));
                }
            }
            _ if counter == MonkeyField::Operation as usize => {
                if let Some((_, end)) = line.split_once("old ") {
                    if let Some((s, o)) = end.split_once(' ') {
                        let sign = match s {
                            "+" => Sign::Plus,
                            "-" => Sign::Minus,
                            "/" => Sign::Divide,
                            "*" => Sign::Multiply,
                            _ => unreachable!(),
                        };
                        let operand = match o {
                            "old" => Operand::Old,
                            _ => Operand::Number(o.parse::<i32>().unwrap()),
                        };
                        operation = Operation { sign, operand };
                    }
                }
            }
            _ if counter == MonkeyField::Test as usize => {
                if let Some((_, divisor)) = line.split_once("by ") {
                    test = divisor.parse::<i32>().unwrap();
                }
            }
            _ if counter == MonkeyField::IfTrue as usize => {
                if let Some((_, monkey)) = line.split_once("monkey ") {
                    if_true = monkey.parse::<usize>().unwrap();
                }
            }
            _ if counter == MonkeyField::IfFalse as usize => {
                if let Some((_, monkey)) = line.split_once("monkey ") {
                    if_false = monkey.parse::<usize>().unwrap();
                }
            }
            _ => {
                monkeys.push(Monkey {
                    name,
                    items,
                    operation,
                    test,
                    if_true,
                    if_false,
                });

                name = 0;
                items = Vec::new();
                operation = Operation {
                    sign: Sign::Plus,
                    operand: Operand::Old,
                };
                test = 0;
                if_true = 0;
                if_false = 0;

                counter = 0;
                continue;
            }
        }
        counter += 1;
    }
    monkeys.push(Monkey {
        name,
        items,
        operation,
        test,
        if_true,
        if_false,
    });
    println!("{:#?}", monkeys);

    Ok(())
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
