use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

fn main() -> std::io::Result<()> {
    println!("===part1===");
    println!("{}", part1()?);
    println!("===part2===");
    part2()?;
    Ok(())
}

fn part1() -> std::io::Result<usize> {
    let lines = read_lines("./input.txt")?;

    let head = Position { x: 0, y: 0 };
    let tail = Position { x: 0, y: 0 };
    let head_visited = HashSet::new();
    let tail_visited = HashSet::new();

    let mut rope = Rope {
        head,
        tail,
        head_visited,
        tail_visited,
    };

    for line in lines.flatten() {
        if let Some((left, right)) = line.split_once(' ') {
            let direction = Direction::from_str(left).expect("Should be one of LRUD");
            let steps = right.parse::<usize>().expect("Should be parsable");

            rope.apply(direction, steps);
        }
    }

    Ok(rope.tail_visited_count())
}

fn part2() -> std::io::Result<()> {
    let _lines = read_lines("./input_smol.txt");

    Ok(())
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(input: &str) -> Result<Direction, Self::Err> {
        match input {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn update(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }
}

#[derive(Debug)]
struct Rope {
    head: Position,
    tail: Position,
    head_visited: HashSet<Position>,
    tail_visited: HashSet<Position>,
}

impl Rope {
    fn apply(&mut self, direction: Direction, steps: usize) {
        println!("{direction:?} {steps}");
        match direction {
            Direction::Left => self.move_head(-1, 0, steps),
            Direction::Right => self.move_head(1, 0, steps),
            Direction::Up => self.move_head(0, 1, steps),
            Direction::Down => self.move_head(0, -1, steps),
        }
    }

    fn move_head(&mut self, x: i32, y: i32, steps: usize) {
        for _ in 1..=steps {
            self.head.update(x, y);
            println!("head at: {:?}", self.head);
            self.head_visited.insert(self.head);
            self.update_tail();
            //self.print_state();
        }
    }

    fn update_tail(&mut self) {
        match (self.head.x - self.tail.x, self.head.y - self.tail.y) {
            // check head is +2/-2 in any x/y axis
            // left
            (-2, 0) => self.tail.update(-1, 0),
            // right
            // .....    .....    .....
            // .TH.. -> .T.H. -> ..TH.
            // .....    .....    .....
            (2, 0) => self.tail.update(1, 0),

            // up
            (0, 2) => self.tail.update(0, 1),

            // down
            // ...    ...    ...
            // .T.    .T.    ...
            // .H. -> ... -> .T.
            // ...    .H.    .H.
            // ...    ...    ...
            (0, -2) => self.tail.update(0, -1),

            // check head diagonals:
            // ......
            // .H.H..
            // H...H.
            // ..T...
            // H...H.
            // .H.H..

            // 1 up, 2 left
            (-2, 1) => self.tail.update(-1, 1),
            // 1 up, 2 right
            (2, 1) => self.tail.update(1, 1),
            // 1 down, 2 left
            (-2, -1) => self.tail.update(-1, -1),
            // 1 down, 2 right
            (2, -1) => self.tail.update(1, -1),

            // Same moves, but different head position
            // ......
            // .H.H..
            // H...H.
            // ..T...
            // H...H.
            // .H.H..
            // 2 up, 1 left
            (-1, 2) => self.tail.update(-1, 1),
            // 2 up, 1 right
            // .....    .....    .....
            // .....    ..H..    ..H..
            // ..H.. -> ..... -> ..T..
            // .T...    .T...    .....
            // .....    .....    .....
            (1, 2) => self.tail.update(1, 1),
            // 2 down, 1 left
            (-1, -2) => self.tail.update(-1, -1),
            // 2 down, 1 right
            (1, -2) => self.tail.update(1, -1),

            _ => (), // staying still
        }
        println!("tail at: {:?}", self.tail);
        self.tail_visited.insert(self.tail);
    }

    fn head_visited_count(&self) -> usize {
        self.head_visited.len()
    }

    fn tail_visited_count(&self) -> usize {
        self.tail_visited.len()
    }

    fn print_state(&self) {
        let mut board = vec![
            vec!['.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.'],
        ];
        board[self.head.x as usize][self.head.y as usize] = 'H';
        board[self.tail.x as usize][self.tail.y as usize] = 'T';
        for row in board.iter().rev() {
            println!("{:?}", row);
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
