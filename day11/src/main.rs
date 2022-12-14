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

#[derive(Debug, Clone)]
enum Sign {
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug, Clone)]
enum Operand {
    Old,
    Number(i64),
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
struct Monkey {
    name: usize,
    items: Vec<i64>,
    operation: Operation,
    test: i64,
    if_true: usize,
    if_false: usize,
}

impl Monkey {
    fn moves(&self) -> Vec<(usize, i64)> {
        self.items
            .clone()
            .into_iter()
            .map(|item| {
                let thrown = self.inspect(item);
                let dest = self.test_divisible(thrown);
                (dest, thrown)
            })
            .collect()
    }

    fn inspect(&self, item: i64) -> i64 {
        println!("  Monkey inspects an item with a worry level of {}", item);
        let inspection_result = self.apply_operation(item);
        println!(
            "    Monkey gets bored with item. Worry level is divided by 3 to {}.",
            inspection_result / 3
        );
        inspection_result / 3
    }

    fn apply_operation(&self, item: i64) -> i64 {
        let operand = match self.operation.operand {
            Operand::Old => item,
            Operand::Number(i) => i,
        };
        match &self.operation.sign {
            Sign::Plus => {
                println!(
                    "    Worry level increases by {operand} to {}.",
                    item + operand
                );
                item + operand
            }
            Sign::Minus => {
                println!(
                    "    Worry level decreases by {operand} to {}.",
                    item - operand
                );
                item - operand
            }
            Sign::Divide => {
                println!(
                    "    Worry level is divided by {operand} to {}.",
                    item / operand
                );
                item / operand
            }
            Sign::Multiply => {
                println!(
                    "    Worry level is multiplied by {operand} to {}.",
                    item * operand
                );
                item * operand
            }
        }
    }

    fn test_divisible(&self, item: i64) -> usize {
        match item % self.test == 0 {
            true => {
                println!("    Current worry level is divisible by {}", self.test);
                println!(
                    "    Item with worry level {item} is thrown to monkey {}",
                    self.if_true
                );
                self.if_true
            }
            false => {
                println!("    Current worry level is not divisible by {}", self.test);
                println!(
                    "    Item with worry level {item} is thrown to monkey {}",
                    self.if_false
                );
                self.if_false
            }
        }
    }

    fn accept(&mut self, thrown: i64) {
        self.items.push(thrown);
    }
}

fn part1() -> std::io::Result<i64> {
    let lines = read_lines("./input.txt")?;
    let mut monkeys = parse_monkeys(lines);
    let mut counts = vec![0; monkeys.len()];

    for i in 1..=20 {
        println!("Turn {i}:");

        for i in 0..monkeys.len() {
            let new_monkey = monkeys[i].clone();
            let moves = new_monkey.moves();
            monkeys[i].items.clear();

            for (dest, thrown) in moves {
                monkeys[dest].accept(thrown);
                counts[new_monkey.name] += 1;
            }
        }

        for m in &monkeys {
            println!("Monkey {}: {:?}", m.name, m.items);
        }
    }

    println!("counts: {counts:?}");
    counts.sort();
    let result = counts.iter().rev().take(2).product();

    Ok(result)
}

fn part2() -> std::io::Result<()> {
    let lines = read_lines("./input_smol.txt")?;
    let _monkeys = parse_monkeys(lines);

    Ok(())
}

fn parse_monkeys(lines: std::io::Lines<BufReader<File>>) -> Vec<Monkey> {
    let mut counter = 0;
    let mut monkeys: Vec<Monkey> = Vec::new();

    let mut name = 0;
    let mut items: Vec<i64> = Vec::new();
    let mut operation = Operation {
        sign: Sign::Plus,
        operand: Operand::Old,
    };
    let mut test = 0;
    let mut if_true = 0;
    let mut if_false = 0;

    for line in lines.flatten() {
        // the most bargain basement parsing you'll see this week
        // because reading more than a line at a time is too easy =P
        match counter {
            _ if counter == MonkeyField::Name as usize => {
                name = monkeys.len();
            }
            _ if counter == MonkeyField::Items as usize => {
                if let Some((_, end)) = line.split_once(": ") {
                    end.split(", ")
                        .for_each(|i| items.push(i.parse::<i64>().unwrap()));
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
                            _ => Operand::Number(o.parse::<i64>().unwrap()),
                        };
                        operation = Operation { sign, operand };
                    }
                }
            }
            _ if counter == MonkeyField::Test as usize => {
                if let Some((_, divisor)) = line.split_once("by ") {
                    test = divisor.parse::<i64>().unwrap();
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

                // reset fields for next monkey
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

    // all my remaining monkeys (1), unflushed
    monkeys.push(Monkey {
        name,
        items,
        operation,
        test,
        if_true,
        if_false,
    });

    monkeys
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
