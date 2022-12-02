use std::cmp::Ordering;

use aoc_utils::read_file;

enum Janken {
    Rock,
    Paper,
    Scissor,
}

impl PartialEq for Janken {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Janken::Rock, Janken::Rock) => true,
            (Janken::Paper, Janken::Paper) => true,
            (Janken::Scissor, Janken::Scissor) => true,
            _ => false,
        }
    }
}

impl PartialOrd for Janken {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Janken::Rock, Janken::Paper) => Some(Ordering::Less),
            (Janken::Rock, Janken::Scissor) => Some(Ordering::Greater),
            (Janken::Paper, Janken::Scissor) => Some(Ordering::Less),
            (Janken::Paper, Janken::Rock) => Some(Ordering::Greater),
            (Janken::Scissor, Janken::Rock) => Some(Ordering::Less),
            (Janken::Scissor, Janken::Paper) => Some(Ordering::Greater),
            _ => Some(Ordering::Equal),
        }
    }
}

impl From<char> for Janken {
    fn from(s: char) -> Self {
        match s {
            'A' => Janken::Rock,
            'B' => Janken::Paper,
            'C' => Janken::Scissor,
            _ => panic!("Invalid character"),
        }
    }
}

impl Janken {
    fn score(&self) -> u32 {
        match self {
            Janken::Rock => 1,
            Janken::Paper => 2,
            Janken::Scissor => 3,
        }
    }

    fn game_score(&self, other: &Self) -> u32 {
        match self.partial_cmp(other) {
            Some(Ordering::Less) => 0,
            Some(Ordering::Equal) => 3,
            Some(Ordering::Greater) => 6,
            None => panic!("Invalid comparison"),
        }
    }
}

fn main() -> Result<(), ()> {
    let input = read_file("day02/input")?;
    let lines = parse(input);

    println!("Part 1: {}", part1(&lines));

    Ok(())
}

fn parse(input: String) -> Vec<Vec<char>> {
    input
        .split("\n")
        .map(|line| {
            line.split(" ")
                .map(|s| s.chars().next().unwrap())
                .collect::<Vec<char>>()
        })
        .collect()
}

fn part1(lines: &Vec<Vec<char>>) -> u32 {
    lines
        .iter()
        .map(|chars| {
            let other = Janken::from(chars[0]);
            let me = Janken::from(match chars[1] {
                'X' => 'A',
                'Y' => 'B',
                'Z' => 'C',
                _ => panic!("Invalid character"),
            });

            me.score() + me.game_score(&other)
        })
        .sum()
}
