use anyhow::{anyhow, Result};
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<()> {
    let paths = parse_input("input.txt")?;
    let rocks = paths
        .into_iter()
        .map(interpolate_path)
        .reduce(|a, b| &a | &b)
        .ok_or(anyhow!("empty input"))?;

    let sand = simulate_sand(&rocks, (500, 0));
    dbg!(sand.len());

    let sand_with_floor = simulate_sand_with_floor(&rocks, (500, 0));
    dbg!(sand_with_floor.len());

    Ok(())
}

fn simulate_sand(rocks: &HashSet<(u32, u32)>, p: (u32, u32)) -> HashSet<(u32, u32)> {
    let lowest_rock_y = rocks.iter().map(|(_, y)| y).max().unwrap();
    let mut sand = HashSet::new();
    loop {
        let mut d = p;
        loop {
            if d.1 > *lowest_rock_y {
                return sand;
            } else if !rocks.contains(&(d.0, d.1 + 1)) && !sand.contains(&(d.0, d.1 + 1)) {
                d = (d.0, d.1 + 1);
            } else if !rocks.contains(&(d.0 - 1, d.1 + 1)) && !sand.contains(&(d.0 - 1, d.1 + 1)) {
                d = (d.0 - 1, d.1 + 1);
            } else if !rocks.contains(&(d.0 + 1, d.1 + 1)) && !sand.contains(&(d.0 + 1, d.1 + 1)) {
                d = (d.0 + 1, d.1 + 1);
            } else {
                sand.insert(d);
                break;
            }
        }
    }
}

fn simulate_sand_with_floor(rocks: &HashSet<(u32, u32)>, p: (u32, u32)) -> HashSet<(u32, u32)> {
    let floor_y = rocks.iter().map(|(_, y)| y).max().unwrap() + 2;
    let mut sand = HashSet::new();
    loop {
        let mut d = p;
        loop {
            if d.1 + 1 == floor_y {
                sand.insert(d);
                break;
            } else if !rocks.contains(&(d.0, d.1 + 1)) && !sand.contains(&(d.0, d.1 + 1)) {
                d = (d.0, d.1 + 1);
            } else if !rocks.contains(&(d.0 - 1, d.1 + 1)) && !sand.contains(&(d.0 - 1, d.1 + 1)) {
                d = (d.0 - 1, d.1 + 1);
            } else if !rocks.contains(&(d.0 + 1, d.1 + 1)) && !sand.contains(&(d.0 + 1, d.1 + 1)) {
                d = (d.0 + 1, d.1 + 1);
            } else {
                if !sand.insert(d) {
                    return sand;
                }
                break;
            }
        }
    }
}

fn interpolate_path<I: IntoIterator<Item = (u32, u32)>>(path: I) -> HashSet<(u32, u32)> {
    let mut points = HashSet::new();
    let mut start: Option<(u32, u32)> = None;
    for end @ (ex, ey) in path {
        if let Some((sx, sy)) = start {
            if sx == ex {
                if sy < ey {
                    points.extend((sy..=ey).map(|y| (sx, y)))
                } else {
                    points.extend((ey..=sy).map(|y| (sx, y)))
                }
            } else {
                if sx < ex {
                    points.extend((sx..=ex).map(|x| (x, sy)))
                } else {
                    points.extend((ex..=sx).map(|x| (x, sy)))
                }
            }
        } else {
            points.insert(end);
        }
        start = Some(end);
    }
    points
}

fn parse_input<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<Vec<(u32, u32)>>> {
    let file = File::open(path)?;
    BufReader::new(file)
        .lines()
        .map(|l| Ok(parse_segment(&l?)?))
        .collect()
}

fn parse_segment(s: &str) -> Result<Vec<(u32, u32)>> {
    s.split(" -> ").map(parse_point).collect()
}

fn parse_point(s: &str) -> Result<(u32, u32)> {
    let (x, y) = s.split_once(",").ok_or(anyhow!("invalid point: {}", s))?;
    Ok((x.parse()?, y.parse()?))
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    #[test]
    fn test_foo() -> Result<()> {
        Ok(())
    }
}
