use aoc_utils::read_file;

fn main() -> Result<(), ()> {
    let input = read_file("day06/input")?;

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn is_unique(alps: &[u32; 26]) -> bool {
    alps.iter().all(|n| *n <= 1)
}

fn find_marker(input: &str, size: usize) -> usize {
    let mut alps: [u32; 26] = [0; 26];
    let mut prev = input.chars().nth(0).unwrap();

    for c in input.chars().take(size) {
        alps[c as usize - 97] += 1;
    }

    if is_unique(&alps) {
        return size;
    }

    for (idx, c) in input.chars().enumerate().skip(size) {
        alps[prev as usize - 97] -= 1;
        alps[c as usize - 97] += 1;

        if is_unique(&alps) {
            return idx + 1;
        }

        match input.chars().nth(idx - size + 1) {
            Some(c) => prev = c,
            None => return 0,
        }
    }

    0
}

fn part1(input: &str) -> usize {
    find_marker(input, 4)
}

fn part2(input: &str) -> usize {
    find_marker(input, 14)
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

    #[test]
    fn test_part2() {
        let inputs = [
            "mjqjpqmgbljsphdztnvjfqwrcgsml",
            "bvwbjplbgvbhsrlpgdmjqwftvncz",
            "nppdvjthqldpwncqszvftbrmjlhg",
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
        ];

        let expected = [19, 23, 23, 29, 26];

        for (input, expected) in inputs.iter().zip(expected.iter()) {
            assert_eq!(part2(*input), *expected);
        }
    }
}
