use anyhow::Result;
use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

struct Move {
    n: usize,
    from: usize,
    to: usize,
}

impl std::str::FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_ascii_whitespace();

        words.next(); // "move"
        let n = words.next().unwrap().parse()?;
        words.next(); // "from"
        let from = words.next().unwrap().parse()?;
        words.next(); // "to"
        let to = words.next().unwrap().parse()?;

        Ok(Move { n, from, to })
    }
}

fn main() -> Result<()> {
    //                 [B]     [L]     [S]
    //         [Q] [J] [C]     [W]     [F]
    //     [F] [T] [B] [D]     [P]     [P]
    //     [S] [J] [Z] [T]     [B] [C] [H]
    //     [L] [H] [H] [Z] [G] [Z] [G] [R]
    // [R] [H] [D] [R] [F] [C] [V] [Q] [T]
    // [C] [J] [M] [G] [P] [H] [N] [J] [D]
    // [H] [B] [R] [S] [R] [T] [S] [R] [L]
    //  1   2   3   4   5   6   7   8   9

    let stacks = vec![
        "RCH".chars().collect::<VecDeque<char>>(),
        "FSLHJB".chars().collect::<VecDeque<char>>(),
        "QTJHDMR".chars().collect::<VecDeque<char>>(),
        "JBZHRGS".chars().collect::<VecDeque<char>>(),
        "BCDTZFPR".chars().collect::<VecDeque<char>>(),
        "GCHT".chars().collect::<VecDeque<char>>(),
        "LWPBZVNS".chars().collect::<VecDeque<char>>(),
        "CGQJR".chars().collect::<VecDeque<char>>(),
        "SFPHRTDL".chars().collect::<VecDeque<char>>(),
    ];

    let input = read_input("input.txt")?;
    let moves = input
        .iter()
        .map(|s| s.parse())
        .collect::<Result<Vec<Move>>>()?;

    let mut stacks_9000 = stacks.clone();
    apply_moves_9000(&mut stacks_9000, &moves);
    let tops_9000 = stacks_9000.iter().map(|s| s[0]).collect::<String>();
    dbg!(tops_9000);

    let mut stacks_9001 = stacks.clone();
    apply_moves_9001(&mut stacks_9001, &moves);
    let tops_9001 = stacks_9001.iter().map(|s| s[0]).collect::<String>();
    dbg!(tops_9001);

    Ok(())
}

fn apply_moves_9000(stacks: &mut Vec<VecDeque<char>>, moves: &Vec<Move>) {
    for m in moves {
        for _ in 0..m.n {
            let x = stacks[m.from - 1].pop_front().unwrap();
            stacks[m.to - 1].push_front(x);
        }
    }
}

fn apply_moves_9001(stacks: &mut Vec<VecDeque<char>>, moves: &Vec<Move>) {
    for m in moves {
        let xs: Vec<char> = stacks[m.from - 1].drain(0..m.n).rev().collect();
        for x in xs {
            stacks[m.to - 1].push_front(x);
        }
    }
}

fn read_input<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<String>> {
    let file = File::open(path)?;
    BufReader::new(file).lines().map(|l| Ok(l?)).collect()
}
