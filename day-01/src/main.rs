use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<()> {
    let input = read_input("input.txt")?;

    let mut sums: Vec<i32> = input
        .split(|l| l == "")
        .map(|s| s.iter().map(|n| n.parse::<i32>().unwrap_or(0)).sum())
        .collect();
    sums.sort_by(|a, b| b.cmp(a));

    dbg!(sums.iter().next());
    dbg!(sums.iter().take(3).sum::<i32>());

    Ok(())
}

fn read_input<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<String>> {
    let file = File::open(path)?;
    BufReader::new(file).lines().map(|l| Ok(l?)).collect()
}
