use std::collections::HashSet;

use aoc_utils::read_file;

fn main() -> Result<(), ()> {
    let input = read_file("day08/input")?;
    let grid = parse(input);

    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {}", part2(&grid));

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

    (n + m) * 2 - 4 + find_visible_dots(grid).len()
}

fn part2(grid: &Vec<Vec<u8>>) -> u32 {
    find_visible_dots(grid)
        .iter()
        .map(|coord| calc_scenic_score(*coord, grid))
        .max()
        .unwrap_or(0)
}

fn calc_scenic_score((x, y): (usize, usize), grid: &Vec<Vec<u8>>) -> u32 {
    let n = grid.len();
    let m = grid[0].len();

    let curr = grid[x][y];

    let mut left = 0;
    for j in (0..y).rev() {
        left += 1;
        if grid[x][j] >= curr {
            break;
        }
    }

    let mut right = 0;
    for j in (y + 1)..m {
        right += 1;
        if grid[x][j] >= curr {
            break;
        }
    }

    let mut top = 0;
    for i in (0..x).rev() {
        top += 1;
        if grid[i][y] >= curr {
            break;
        }
    }

    let mut bottom = 0;
    for i in (x + 1)..n {
        bottom += 1;
        if grid[i][y] >= curr {
            break;
        }
    }

    left * right * top * bottom
}

fn find_visible_dots(grid: &Vec<Vec<u8>>) -> HashSet<(usize, usize)> {
    let n = grid.len();
    let m = grid[0].len();

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

    visible_dots
}
