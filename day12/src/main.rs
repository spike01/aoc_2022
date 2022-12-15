use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() -> std::io::Result<()> {
    println!("===part1===");
    println!("{}", part1()?);
    println!("===part2===");
    println!("{}", part2()?);
    Ok(())
}

#[derive(Debug, Eq, Copy, Clone)]
struct Position {
    height: u32,
    x: usize,
    y: usize,
    distance: usize,
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.height == other.height
    }
}

impl Hash for Position {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl Position {
    // you can move exactly one square up, down, left, or right. To avoid needing to get out your
    // climbing gear, the elevation of the destination square can be at most one higher than the
    // elevation of your current square; that is, if your current elevation is m, you could step to
    // elevation n, but not to elevation o. (This also means that the elevation of the destination
    // square can be much lower than the elevation of your current square.)
    fn valid_neighbours<'a>(&'a self, heightmap: &'a [Vec<Position>]) -> Vec<&Position> {
        let mut neighbours = Vec::new();
        // left
        if let Some(row) = heightmap.get(self.y) {
            if let Some(x) = self.x.checked_sub(1) {
                if let Some(left) = row.get(x) {
                    if left.height <= self.height + 1 {
                        neighbours.push(left);
                    }
                }
            }
        }
        // right
        if let Some(row) = heightmap.get(self.y) {
            if let Some(right) = row.get(self.x + 1) {
                if right.height <= self.height + 1 {
                    neighbours.push(right);
                }
            }
        }
        // up
        if let Some(y) = self.y.checked_sub(1) {
            if let Some(row) = heightmap.get(y) {
                if let Some(up) = row.get(self.x) {
                    if up.height <= self.height + 1 {
                        neighbours.push(up);
                    }
                }
            }
        }
        // down
        if let Some(row) = heightmap.get(self.y + 1) {
            if let Some(down) = row.get(self.x) {
                if down.height <= self.height + 1 {
                    neighbours.push(down);
                }
            }
        }
        neighbours
    }

    fn valid_downward_neighbours<'a>(&'a self, heightmap: &'a [Vec<Position>]) -> Vec<&Position> {
        let mut neighbours = Vec::new();
        // left
        if let Some(row) = heightmap.get(self.y) {
            if let Some(x) = self.x.checked_sub(1) {
                if let Some(left) = row.get(x) {
                    if left.height >= self.height - 1 {
                        neighbours.push(left);
                    }
                }
            }
        }
        // right
        if let Some(row) = heightmap.get(self.y) {
            if let Some(right) = row.get(self.x + 1) {
                if right.height >= self.height - 1 {
                    neighbours.push(right);
                }
            }
        }
        // up
        if let Some(y) = self.y.checked_sub(1) {
            if let Some(row) = heightmap.get(y) {
                if let Some(up) = row.get(self.x) {
                    if up.height >= self.height - 1 {
                        neighbours.push(up);
                    }
                }
            }
        }
        // down
        if let Some(row) = heightmap.get(self.y + 1) {
            if let Some(down) = row.get(self.x) {
                if down.height >= self.height - 1 {
                    neighbours.push(down);
                }
            }
        }
        neighbours
    }
}

fn part1() -> std::io::Result<usize> {
    let lines = read_lines("./input.txt")?;

    let mut heightmap: Vec<Vec<Position>> = Vec::new();
    let mut start_pos = Position {
        distance: 0,
        height: 0,
        x: 0,
        y: 0,
    };
    let mut end_pos = Position {
        distance: 0,
        height: 0,
        x: 0,
        y: 0,
    };

    for (y, line) in lines.flatten().enumerate() {
        let mut row: Vec<Position> = Vec::new();
        for (x, height) in line.chars().enumerate() {
            let h = match height {
                'S' => {
                    start_pos = Position {
                        x,
                        y,
                        height: 'a' as u32,
                        distance: 0,
                    };
                    'a' as u32
                }
                'E' => {
                    end_pos = Position {
                        x,
                        y,
                        height: 'z' as u32,
                        distance: 0,
                    };
                    'z' as u32
                }
                _ => height as u32,
            };
            row.push(Position {
                height: h,
                x,
                y,
                distance: usize::MAX,
            });
        }
        heightmap.push(row);
    }

    // Dijkstra's algorithm (implemented badly from Wikipedia!)
    // 1. Mark all nodes unvisited. Create a set of all the unvisited nodes called the unvisited
    //    set.
    let mut unvisited: HashSet<&Position> = heightmap.iter().flatten().collect();

    // 2. Assign to every node a tentative distance value: set it to zero for our initial node and
    //    to infinity for all other nodes. During the run of the algorithm, the tentative distance of
    //    a node v is the length of the shortest path discovered so far between the node v and the
    //    starting node. Since initially no path is known to any other vertex than the source itself
    //    (which is a path of length zero), all other tentative distances are initially set to
    //    infinity. Set the initial node as current.
    let initial = &heightmap[start_pos.y][start_pos.x];
    let mut current = Position {
        distance: 0,
        ..*initial
    };

    // 3. For the current node, consider all of its unvisited neighbors and calculate their
    //    tentative distances through the current node. Compare the newly calculated tentative
    //    distance to the one currently assigned to the neighbor and assign it the smaller one. For
    //    example, if the current node A is marked with a distance of 6, and the edge connecting it
    //    with a neighbor B has length 2, then the distance to B through A will be 6 + 2 = 8. If
    //    B was previously marked with a distance greater than 8 then change it to 8. Otherwise,
    //    the current value will be kept.
    let mut queue = VecDeque::new();
    let edge_length = 1;

    loop {
        if unvisited.contains(&current) {
            for neighbour in current.valid_neighbours(&heightmap) {
                let distance = if current.distance + edge_length < neighbour.distance {
                    current.distance + edge_length
                } else {
                    neighbour.distance
                };
                queue.push_back(Position {
                    distance,
                    ..*neighbour
                });
            }
            // 4. When we are done considering all of the unvisited neighbors of the current node, mark the
            //    current node as visited and remove it from the unvisited set. A visited node will never
            //    be checked again (this is valid and optimal in connection with the behavior in step 6.:
            //    that the next nodes to visit will always be in the order of 'smallest distance from
            //    initial node first' so any visits after would have a greater distance).
            unvisited.remove(&current);
        }

        // 5. If the destination node has been marked visited (when planning a route between two
        //    specific nodes) or if the smallest tentative distance among the nodes in the unvisited
        //    set is infinity (when planning a complete traversal; occurs when there is no connection
        //    between the initial node and remaining unvisited nodes), then stop. The algorithm has
        //    finished.
        if current.x == end_pos.x && current.y == end_pos.y {
            return Ok(current.distance);
        }
        if queue.is_empty() {
            unreachable!("I was promised a solution!");
        }

        // 6. Otherwise, select the unvisited node that is marked with the smallest tentative distance,
        //    set it as the new current node, and go back to step 3.
        current = queue.pop_front().unwrap();
    } // end of loop
}

fn part2() -> std::io::Result<usize> {
    let lines = read_lines("./input.txt")?;

    let mut heightmap: Vec<Vec<Position>> = Vec::new();
    let mut start_pos = Position {
        distance: 0,
        height: 0,
        x: 0,
        y: 0,
    };

    for (y, line) in lines.flatten().enumerate() {
        let mut row: Vec<Position> = Vec::new();
        for (x, height) in line.chars().enumerate() {
            let h = match height {
                'S' => 'a' as u32,
                'E' => {
                    start_pos = Position {
                        x,
                        y,
                        height: 'z' as u32,
                        distance: 0,
                    };
                    'z' as u32
                }
                _ => height as u32,
            };
            row.push(Position {
                height: h,
                x,
                y,
                distance: usize::MAX,
            });
        }
        heightmap.push(row);
    }

    let mut unvisited: HashSet<&Position> = heightmap.iter().flatten().collect();
    let mut steps = Vec::new();

    let initial = &heightmap[start_pos.y][start_pos.x];
    let mut current = Position {
        distance: 0,
        ..*initial
    };

    let mut queue = VecDeque::new();
    let edge_length = 1;

    loop {
        if unvisited.contains(&current) {
            for neighbour in current.valid_downward_neighbours(&heightmap) {
                let distance = if current.distance + edge_length < neighbour.distance {
                    current.distance + edge_length
                } else {
                    neighbour.distance
                };
                let adjusted_distance = Position {
                    distance,
                    ..*neighbour
                };
                queue.push_back(adjusted_distance);
            }
            unvisited.remove(&current);
        }

        if current.height == 'a' as u32 {
            steps.push(current.distance);
        }
        if queue.is_empty() {
           break;
        }

        current = queue.pop_front().unwrap();
    }

    Ok(*steps.iter().min().unwrap())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
