use std::collections::HashSet;

use aoc_utils::read_file;

struct Compartment {
    left: String,
    right: String,
}

impl Compartment {
    fn common(&self) -> Option<char> {
        let left: HashSet<_> = self.left.chars().collect();
        let right: HashSet<_> = self.right.chars().collect();

        left.intersection(&right).next().cloned()
    }
}

fn priority(sc: Option<char>) -> u64 {
    match sc {
        None => 0,
        Some(c) => {
            if c.is_lowercase() {
                c as u64 - 96
            } else {
                c as u64 - 64 + 26
            }
        }
    }
}

fn main() -> Result<(), ()> {
    let input = read_file("day03/input")?;
    let elfs = parse(input);

    println!("Part 1: {}", part1(&elfs));

    Ok(())
}

fn parse(input: String) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect::<_>()
}

fn part1(elfs: &Vec<String>) -> u64 {
    elfs.iter()
        .map(|elf| {
            let (left, right) = elf.split_at(elf.len() / 2);

            let com = Compartment {
                left: left.to_string(),
                right: right.to_string(),
            };

            priority(com.common())
        })
        .sum()
}
