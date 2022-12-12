use anyhow::{anyhow, Result};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<()> {
    let grid = read_input("input.txt")?;

    let (start, end) = find_start_end(&grid);
    let path = find_path(&grid, start, end)?;
    dbg!(path.len() - 1);

    let (starts, end) = find_starts_end(&grid);
    let shortest_path = find_shortest_path(&grid, starts, end)?;
    dbg!(shortest_path.len() - 1);

    Ok(())
}

fn find_shortest_path(
    grid: &Vec<Vec<char>>,
    starts: Vec<(usize, usize)>,
    end: (usize, usize),
) -> Result<Vec<(usize, usize)>> {
    starts
        .iter()
        .filter_map(|start| find_path(grid, *start, end).ok())
        .min_by_key(|p| p.len())
        .ok_or(anyhow!("no path"))
}

// https://en.wikipedia.org/wiki/A*_search_algorithm#Pseudocode
fn find_path(
    grid: &Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
) -> Result<Vec<(usize, usize)>> {
    let mut open_set = HashSet::from([start]);
    let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut g_score: HashMap<(usize, usize), f64> = HashMap::from([(start, 0.0)]);
    let mut f_score: HashMap<(usize, usize), f64> = HashMap::from([(start, h(start, end))]);

    while !open_set.is_empty() {
        let open_set_c = open_set.clone();
        let current = open_set_c
            .iter()
            .min_by(|x, y| {
                f_score
                    .get(x)
                    .unwrap()
                    .partial_cmp(f_score.get(y).unwrap())
                    .unwrap()
            })
            .unwrap();
        if current == &end {
            return Ok(reconstruct_path(came_from, *current));
        }

        open_set.remove(current);
        let neighbours = neighbours(grid, *current);
        for neighbour in neighbours {
            let tentative_g_score = g_score.get(current).unwrap() + d(grid, *current, neighbour);
            if &tentative_g_score < g_score.get(&neighbour).unwrap_or(&f64::INFINITY) {
                came_from.insert(neighbour, *current);
                g_score.insert(neighbour, tentative_g_score);
                f_score.insert(neighbour, tentative_g_score + h(neighbour, end));
                open_set.insert(neighbour);
            }
        }
    }

    Err(anyhow!("no path found"))
}

fn d(
    grid: &Vec<Vec<char>>,
    (from_row, from_col): (usize, usize),
    (to_row, to_col): (usize, usize),
) -> f64 {
    let mut from = grid[from_row][from_col];
    let mut to = grid[to_row][to_col];

    if from == 'S' {
        from = 'a'
    }

    if to == 'E' {
        to = 'z'
    }

    if from as u32 >= to as u32 - 1 {
        1.0
    } else {
        f64::INFINITY
    }
}

fn neighbours(grid: &Vec<Vec<char>>, (row, col): (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbours = vec![];

    if row > 0 {
        neighbours.push((row - 1, col));
    }

    if col > 0 {
        neighbours.push((row, col - 1));
    }

    if row < grid.len() - 1 {
        neighbours.push((row + 1, col));
    }

    if col < grid[row].len() - 1 {
        neighbours.push((row, col + 1));
    }

    neighbours
}

fn reconstruct_path(
    came_from: HashMap<(usize, usize), (usize, usize)>,
    current: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut c = current;
    let mut total_path = vec![c];
    loop {
        if let Some(x) = came_from.get(&c) {
            total_path.push(*x);
            c = *x;
        } else {
            break;
        }
    }
    total_path.reverse();
    total_path
}

fn h((from_row, from_col): (usize, usize), (to_row, to_col): (usize, usize)) -> f64 {
    ((to_row as i32 - from_row as i32).abs() + (to_col as i32 - from_col as i32).abs()) as f64
}

fn find_start_end(grid: &Vec<Vec<char>>) -> ((usize, usize), (usize, usize)) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if c == &'S' {
                start = (i, j)
            }

            if c == &'E' {
                end = (i, j)
            }
        }
    }
    (start, end)
}

fn find_starts_end(grid: &Vec<Vec<char>>) -> (Vec<(usize, usize)>, (usize, usize)) {
    let mut starts = vec![];
    let mut end = (0, 0);
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if c == &'S' {
                starts.push((i, j))
            }

            if c == &'E' {
                end = (i, j)
            }

            if c == &'a' {
                starts.push((i, j));
            }
        }
    }
    (starts, end)
}

fn read_input<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<Vec<char>>> {
    let file = File::open(path)?;
    BufReader::new(file)
        .lines()
        .map(|l| Ok(l?.chars().collect()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_path() {
        let grid = vec![
            vec!['S', 'a', 'b', 'q', 'p', 'o', 'n', 'm'],
            vec!['a', 'b', 'c', 'r', 'y', 'x', 'x', 'l'],
            vec!['a', 'c', 'c', 's', 'z', 'E', 'x', 'k'],
            vec!['a', 'c', 'c', 't', 'u', 'v', 'w', 'j'],
            vec!['a', 'b', 'd', 'e', 'f', 'g', 'h', 'i'],
        ];
        let (start, end) = find_start_end(&grid);
        let path = find_path(&grid, start, end).unwrap();

        assert_eq!(path.len(), 32);
    }

    #[test]
    fn test_find_shortest_path() {
        let grid = vec![
            vec!['S', 'a', 'b', 'q', 'p', 'o', 'n', 'm'],
            vec!['a', 'b', 'c', 'r', 'y', 'x', 'x', 'l'],
            vec!['a', 'c', 'c', 's', 'z', 'E', 'x', 'k'],
            vec!['a', 'c', 'c', 't', 'u', 'v', 'w', 'j'],
            vec!['a', 'b', 'd', 'e', 'f', 'g', 'h', 'i'],
        ];
        let (starts, end) = find_starts_end(&grid);
        let path = find_shortest_path(&grid, starts, end).unwrap();

        assert_eq!(path.len(), 30);
    }
}
