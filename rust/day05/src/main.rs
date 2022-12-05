use regex::Regex;

use aoc_utils::read_file;

trait Parse<T> {
    fn parse(_: T) -> Self;
}

#[derive(Debug, Clone)]
struct Board(Vec<Vec<char>>);

impl Parse<&str> for Board {
    fn parse(board: &str) -> Board {
        let rows = board
            .lines()
            .rev()
            .skip(1)
            .map(|line| {
                line.chars()
                    .into_iter()
                    .skip(1)
                    .step_by(4)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let mut board = vec![vec![]; rows[0].len()];

        for row in rows.iter() {
            for (idx, &c) in row.iter().enumerate() {
                if c != ' ' {
                    board[idx].push(c);
                }
            }
        }

        Board(board)
    }
}

#[derive(Debug)]
struct Movement {
    /// how many items will be moved
    count: usize,

    from: usize,
    to: usize,
}

impl Parse<&str> for Movement {
    fn parse(line: &str) -> Self {
        let re = Regex::new(r"^move (?P<count>\d+) from (?P<from>\d+) to (?P<to>\d+)$").unwrap();
        let caps = re.captures(line).unwrap();

        Movement {
            count: caps["count"]
                .parse::<usize>()
                .expect("unexpected `count` value"),

            from: caps["from"]
                .parse::<usize>()
                .expect("unexpected `from` value")
                - 1,
            to: caps["to"].parse::<usize>().expect("unexpected `to` value") - 1,
        }
    }
}

#[derive(Debug)]
struct Crane {
    board: Board,
    movements: Vec<Movement>,
}

impl Parse<String> for Crane {
    fn parse(input: String) -> Self {
        let mut splitted = input.split("\n\n");

        let board = splitted.next().unwrap();
        let movements = splitted.next().unwrap();

        Crane {
            board: Board::parse(board),
            movements: movements.lines().map(Movement::parse).collect::<Vec<_>>(),
        }
    }
}

fn main() -> Result<(), ()> {
    let input = read_file("day05/input")?;
    let crane = Crane::parse(input);

    println!("Part 1: {:?}", part1(&crane));

    Ok(())
}

fn part1(crane: &Crane) -> String {
    let mut board = crane.board.0.clone();

    for movement in &crane.movements {
        let from = movement.from;
        let to = movement.to;
        let len = board[from].len();

        let mut items = board[from].split_off(len.checked_sub(movement.count).unwrap_or(len));
        items.reverse();
        board[to].append(&mut items);
    }

    board
        .iter()
        .filter_map(|row| row.last())
        .collect::<String>()
}
