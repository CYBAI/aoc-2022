use aoc_utils::read_file;

fn main() {
    let input = read_file("day01/input").unwrap();
    let nums = parse(input);

    println!("Part 1: {}", part1(&nums));
    println!("Part 2: {}", part2(&nums));
}

fn part1(nums: &Vec<u64>) -> u64 {
    *nums.iter().max().unwrap()
}

fn part2(nums: &Vec<u64>) -> u64 {
    let mut nums = nums.clone();
    nums.sort();
    nums.iter().rev().take(3).sum()
}

fn parse(input: String) -> Vec<u64> {
    input
        .split("\n\n")
        .map(|lines| lines.split("\n").map(|n| n.parse::<u64>().unwrap()).sum())
        .collect()
}
