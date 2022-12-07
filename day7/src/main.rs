use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    println!("===part1===");
    part1()?;
    println!("===part2===");
    part2()?;
    Ok(())
}

#[derive(Debug)]
struct State {
    pwd: String,
    filetree: HashMap<String, AocFile>,
    is_reading_output: bool,
    output: Vec<String>
}

impl State {
    fn print_filetree(&self) {
        println!("{:?}", self.filetree.get("/").unwrap().subdirs.as_ref().unwrap());
    }

    fn change_directory(&mut self, dest: &str) {
        self.pwd = dest.to_string();
    }

    fn process_output(&mut self) {
        for line in self.output.iter() {
            if let Some((info, name)) = line.split_once(" ") {
                if info == "dir" {
                    let file = AocFile {
                        name: name.to_string(),
                        size: 0,
                        is_directory: true,
                        subdirs: Some(HashMap::new())
                    };
                    println!("{:?}", file);

                    let root = self.filetree.get("/").unwrap();
                    let mut subdirs = root.subdirs.unwrap();
                    subdirs.insert(name.to_string(), file);
                    //subdirs.entry(name.to_string()).or_insert(file);

                } else {
                    //let file = AocFile {
                        //name: name.to_string(),
                        //size: info.parse().unwrap(),
                        //is_directory: false,
                        //subdirs: None
                    //};
                    //println!("{:?}", file);
                    //if let Entry::Occupied(e) = self.filetree.entry("/".to_string()) {
                       //let subdirs: &mut HashMap<String, AocFile> = &mut e.get().subdirs.unwrap();
                       //subdirs.insert(name.to_string(), file);
                    //}
                    //println!("{:?}", self.filetree.entry("/".to_string()));
                }
            }
        }
         self.print_filetree();
         self.is_reading_output = false;
         self.output.clear();
}
}

#[derive(Debug)]
struct AocFile {
    name: String,
    size: i32,
    is_directory: bool,
    subdirs: Option<HashMap<String, AocFile>>
}

fn part1() -> std::io::Result<()> {
    let lines = read_lines("./input_smol.txt");

    let mut file = AocFile {
        name: "/".to_string(),
        size: 0,
        is_directory: true,
        subdirs: Some(HashMap::new())
    };

    let mut state = State {
        pwd: "/".to_string(),
        filetree: HashMap::new(),
        is_reading_output: false,
        output: Vec::new()
    };

    state.filetree.insert("/".to_string(), file);

    for line in lines?.flatten() {
        if line.starts_with("$ cd") {
            if state.is_reading_output {
                state.process_output();
            }
            let (_, dest) = line.split_at(5);
            state.change_directory(dest);
            println!("pwd: {}", state.pwd);
        }
        if line.starts_with("$ ls") {
            state.is_reading_output = true;
            continue
        }
        if state.is_reading_output && !line.starts_with("$") {
            state.output.push(line.to_string());
            continue
        }
    }

    Ok(())
}


fn part2() -> std::io::Result<()> {
    let _lines = read_lines("./input_smol.txt");

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
