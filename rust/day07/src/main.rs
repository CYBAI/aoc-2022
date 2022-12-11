use std::collections::HashMap;

use aoc_utils::read_file;

type Name = String;

#[derive(Debug, Clone)]
struct File {
    size: u64,
}

#[derive(Debug)]
struct Directory {
    name: Name,
    size: u64,
    files: Vec<File>,
    directories: HashMap<Name, Directory>,
}

impl Directory {
    fn new(name: Name) -> Directory {
        Directory {
            name,
            size: 0,
            files: Vec::new(),
            directories: HashMap::new(),
        }
    }

    fn file_size(&self) -> u64 {
        self.files.iter().map(|f| f.size).sum()
    }

    fn eval_size(&mut self) {
        self.size = self.file_size();

        for dir in self.directories.values_mut() {
            dir.eval_size();
            self.size += dir.size;
        }
    }
}

fn main() -> Result<(), ()> {
    let input = read_file("day07/input")?;
    let mut root = parse(input);
    root.eval_size();

    println!("Part 1: {:?}", part1(&root));
    println!("Part 2: {:?}", part2(&root));

    Ok(())
}

fn parse(input: String) -> Directory {
    let mut path = vec![];
    let mut root = Directory::new("/".to_string());
    let mut curr_dir = &mut root;

    let mut lines = input.lines().peekable();

    while let Some(line) = lines.next() {
        if line.starts_with("$ cd") {
            let dirname = &line[5..];

            if dirname == ".." {
                path.pop();
                continue;
            } else {
                path.push(dirname.to_string());

                if curr_dir.name != dirname {
                    curr_dir = path.iter().skip(1).fold(&mut root, |dir, name| {
                        dir.directories
                            .get_mut(name)
                            .expect("unexpected missing directory")
                    });
                }

                if let Some("$ ls") = lines.next() {
                    while let Some(peeked_line) = lines.peek() {
                        if peeked_line.starts_with("$ cd ") {
                            break;
                        } else {
                            let mut splitted = lines.next().unwrap().split(" ");
                            let fst = splitted.next().unwrap();
                            let snd = splitted.next().unwrap();

                            if fst == "dir" {
                                curr_dir
                                    .directories
                                    .entry(snd.to_string())
                                    .or_insert(Directory::new(snd.to_string()));
                            } else {
                                let size = fst.parse::<u64>().expect("unexpected size");
                                curr_dir.files.push(File { size });
                            }
                        }
                    }
                } else {
                    unreachable!("cd-ed without ls")
                }
            }
        }
    }

    root
}

fn part1(root: &Directory) -> u64 {
    let size = if root.size <= 100000 { root.size } else { 0 };

    size + root.directories.values().map(|d| part1(d)).sum::<u64>()
}

fn part2(root: &Directory) -> u64 {
    let unused = 70_000_000 - root.size;
    let required = (30_000_000 as u64)
        .checked_sub(unused)
        .unwrap_or(30_000_000);

    let mut min = u64::MAX;
    let mut dirs = vec![root];

    while let Some(dir) = dirs.pop() {
        if dir.size >= required {
            min = min.min(dir.size);
        }

        dirs.extend(dir.directories.values());
    }

    if min == u64::MAX {
        0
    } else {
        min
    }
}
