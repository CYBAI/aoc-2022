use std::collections::HashSet;

use aoc_utils::read_file;

fn main() -> Result<(), ()> {
    let input = read_file("day08/input")?;
    let grid = parse(input);

    println!("Part 1: {}", part1(&grid));

    Ok(())
}

fn parse(input: String) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<_>>()
}

fn part1(grid: &Vec<Vec<u8>>) -> usize {
    let n = grid.len();
    let m = grid[0].len();

    let count = (n + m) * 2 - 4;

    let mut visible_dots = HashSet::new();

    for i in 1..n - 1 {
        // left
        let mut max = grid[i][0];
        for j in 1..m - 1 {
            let k = grid[i][j];
            if max < k {
                visible_dots.insert((i, j));
                max = k;
            }
        }

        // right
        let mut max = grid[i][m - 1];
        for j in (1..m - 1).rev() {
            let k = grid[i][j];
            if max < k {
                visible_dots.insert((i, j));
                max = k;
            }
        }
    }

    for j in 1..m - 1 {
        // top
        let mut max = grid[0][j];
        for i in 1..n - 1 {
            let k = grid[i][j];
            if max < k {
                visible_dots.insert((i, j));
                max = k;
            }
        }

        // bottom
        let mut max = grid[n - 1][j];
        for i in (1..n - 1).rev() {
            let k = grid[i][j];
            if max < k {
                visible_dots.insert((i, j));
                max = k;
            }
        }
    }

    count + visible_dots.len()
}
