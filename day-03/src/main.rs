use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<()> {
    let input = read_input("input.txt")?;

    let priorities: HashMap<char, u32> = ('a'..='z')
        .zip(1..=26)
        .chain(('A'..='Z').zip(27..=52))
        .collect();

    let sum1: u32 = input
        .iter()
        .map(split_in_half)
        .filter_map(common_char)
        .filter_map(|c| priorities.get(&c))
        .sum();

    dbg!(sum1);

    let sum2: u32 = input
        .chunks(3)
        .map(Vec::from)
        .filter_map(common_char)
        .filter_map(|c| priorities.get(&c))
        .sum();

    dbg!(sum2);

    Ok(())
}

fn split_in_half<'a, S: AsRef<str>>(s: S) -> [String; 2] {
    let (left, right) = s.as_ref().split_at(s.as_ref().len() / 2);
    [left.to_string(), right.to_string()]
}

fn common_char<I: IntoIterator<Item = String>>(strings: I) -> Option<char> {
    let intersection = strings
        .into_iter()
        .map(|s| s.chars().collect::<HashSet<char>>())
        .reduce(|i, g| &i & &g)?;
    intersection.into_iter().next()
}

fn read_input<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<String>> {
    let file = File::open(path)?;
    BufReader::new(file).lines().map(|l| Ok(l?)).collect()
}
