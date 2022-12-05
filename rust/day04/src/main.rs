use aoc_utils::read_file;

#[derive(Debug)]
struct Elf {
    min: u32,
    max: u32,
}

struct ElfPair((Elf, Elf));

impl ElfPair {
    fn fully_overlap(&self) -> bool {
        let (left, right) = &self.0;

        (left.max <= right.max && left.min >= right.min)
            || (right.max <= left.max && right.min >= left.min)
    }

    fn overlapped(&self) -> bool {
        let (left, right) = &self.0;

        (left.min <= right.max && left.min >= right.min)
            || (right.min <= left.max && right.min >= left.min)
    }
}

fn main() -> Result<(), ()> {
    let input = read_file("day04/input")?;
    let pairs = parse(input);

    println!("Part 1: {}", part1(&pairs));
    println!("Part 2: {}", part2(&pairs));

    Ok(())
}

fn parse(input: String) -> Vec<ElfPair> {
    input
        .lines()
        .map(|line| {
            let mut elfs = line.split(",").map(|elf| {
                let mut digits = elf.split("-").map(|n| n.parse::<u32>().unwrap());
                let min = digits.next().unwrap();
                let max = digits.next().unwrap();

                Elf { min, max }
            });

            ElfPair((elfs.next().unwrap(), elfs.next().unwrap()))
        })
        .collect::<Vec<_>>()
}

fn part1(pairs: &Vec<ElfPair>) -> usize {
    pairs.iter().filter(|pair| pair.fully_overlap()).count()
}

fn part2(pairs: &Vec<ElfPair>) -> usize {
    pairs.iter().filter(|pair| pair.overlapped()).count()
}
