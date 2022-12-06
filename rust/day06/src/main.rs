use std::collections::HashSet;

use aoc_utils::read_file;

fn main() -> Result<(), ()> {
    let input = read_file("day06/input")?;

    println!("Part 1: {}", part1(&input));

    Ok(())
}

fn find_marker(input: &str, size: usize) -> usize {
    let chars = input.chars().collect::<Vec<_>>();

    chars
        .windows(size)
        .enumerate()
        .find(|(_, chars)| {
            let mut set: HashSet<&char> = HashSet::new();

            for c in chars.iter() {
                set.insert(c);
            }

            set.len() == size
        })
        .map(|(idx, _)| idx + size)
        .unwrap_or(0)
}

fn part1(input: &str) -> usize {
    find_marker(input, 4)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let inputs = [
            "bvwbjplbgvbhsrlpgdmjqwftvncz",
            "nppdvjthqldpwncqszvftbrmjlhg",
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
        ];

        let expected = [5, 6, 10, 11];

        for (input, expected) in inputs.iter().zip(expected.iter()) {
            assert_eq!(part1(*input), *expected);
        }
    }
}
