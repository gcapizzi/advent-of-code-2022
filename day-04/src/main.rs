use anyhow::{anyhow, Result};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

struct Range {
    from: u32,
    to: u32,
}

impl std::str::FromStr for Range {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (from, to) = s.split_once('-').ok_or(anyhow!("invalid range: {}", s))?;
        Ok(Range {
            from: from.parse()?,
            to: to.parse()?,
        })
    }
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.from <= other.from && self.to >= other.to
    }

    fn overlaps_with(&self, other: &Range) -> bool {
        (self.from <= other.from && self.to >= other.from)
            || (other.from <= self.from && other.to >= self.from)
    }
}

struct Pair {
    left: Range,
    right: Range,
}

impl std::str::FromStr for Pair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(',').ok_or(anyhow!("invalid pair: {}", s))?;
        Ok(Pair {
            left: left.parse()?,
            right: right.parse()?,
        })
    }
}

impl Pair {
    fn fully_overlaps(&self) -> bool {
        self.left.contains(&self.right) || self.right.contains(&self.left)
    }

    fn partially_overlaps(&self) -> bool {
        self.left.overlaps_with(&self.right)
    }
}

fn main() -> Result<()> {
    let input = read_input("input.txt")?;
    let pairs = input
        .into_iter()
        .map(|s| s.parse())
        .collect::<Result<Vec<Pair>>>()?;

    let n1 = pairs.iter().filter(|p| p.fully_overlaps()).count();
    dbg!(n1);

    let n2 = pairs.iter().filter(|p| p.partially_overlaps()).count();
    dbg!(n2);

    Ok(())
}

fn read_input<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<String>> {
    let file = File::open(path)?;
    BufReader::new(file).lines().map(|l| Ok(l?)).collect()
}
