use anyhow::{anyhow, Result};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter;

enum Direction {
    North,
    South,
    East,
    West,
}

fn main() -> Result<()> {
    let input = read_input("input.txt")?;

    let visibles = (0..input.len())
        .flat_map(|r| (0..input[r].len()).map(move |c| (r, c)))
        .filter(|c| is_visible(&input, *c))
        .count();
    dbg!(visibles);

    let max_score = (0..input.len())
        .flat_map(|r| (0..input[r].len()).map(move |c| (r, c)))
        .map(|c| scenic_score(&input, c))
        .max();
    dbg!(max_score);

    Ok(())
}

fn scenic_score(matrix: &Vec<Vec<u32>>, coord: (usize, usize)) -> usize {
    viewing_distance(matrix, coord, Direction::North)
        * viewing_distance(matrix, coord, Direction::South)
        * viewing_distance(matrix, coord, Direction::East)
        * viewing_distance(matrix, coord, Direction::West)
}

fn viewing_distance(matrix: &Vec<Vec<u32>>, coord: (usize, usize), direction: Direction) -> usize {
    let te = to_edge(matrix, coord, direction);
    te.iter()
        .position(|h| h >= &&matrix[coord.0][coord.1])
        .map(|p| p + 1)
        .unwrap_or(te.len())
}

fn is_visible(matrix: &Vec<Vec<u32>>, coord: (usize, usize)) -> bool {
    is_visible_from(matrix, coord, Direction::North)
        || is_visible_from(matrix, coord, Direction::South)
        || is_visible_from(matrix, coord, Direction::East)
        || is_visible_from(matrix, coord, Direction::West)
}

fn is_visible_from(matrix: &Vec<Vec<u32>>, coord: (usize, usize), direction: Direction) -> bool {
    to_edge(&matrix, coord, direction)
        .iter()
        .all(|h| h < &matrix[coord.0][coord.1])
}

fn to_edge(matrix: &Vec<Vec<u32>>, coord: (usize, usize), direction: Direction) -> Vec<u32> {
    let (row_range, col_range): (
        Box<dyn Iterator<Item = usize>>,
        Box<dyn Iterator<Item = usize>>,
    ) = match direction {
        Direction::North => (
            Box::new((0..coord.0).rev()),
            Box::new(iter::repeat(coord.1)),
        ),
        Direction::South => (
            Box::new((coord.0 + 1..=matrix.len() - 1).into_iter()),
            Box::new(iter::repeat(coord.1)),
        ),
        Direction::East => (
            Box::new(iter::repeat(coord.0)),
            Box::new((coord.1 + 1..=matrix[coord.0].len() - 1).into_iter()),
        ),
        Direction::West => (
            Box::new(iter::repeat(coord.0)),
            Box::new((0..coord.1).rev()),
        ),
    };

    row_range
        .zip(col_range)
        .map(|(r, c)| matrix[r][c])
        .collect()
}

fn read_input<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<Vec<u32>>> {
    let file = File::open(path)?;
    BufReader::new(file)
        .lines()
        .map(|l| {
            Ok(l?
                .chars()
                .map(|c| c.to_digit(10).ok_or(anyhow!("{} is not a digit", c)))
                .collect::<Result<Vec<u32>>>()?)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test() -> Result<()> {
        let matrix = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];

        assert_eq!(to_edge(&matrix, (2, 2), Direction::North), vec![5, 3]);
        assert_eq!(to_edge(&matrix, (2, 2), Direction::South), vec![5, 3]);
        assert_eq!(to_edge(&matrix, (2, 2), Direction::East), vec![3, 2]);
        assert_eq!(to_edge(&matrix, (2, 2), Direction::West), vec![5, 6]);

        assert!(is_visible(&matrix, (0, 0)));
        assert!(is_visible(&matrix, (0, 1)));
        assert!(is_visible(&matrix, (0, 2)));
        assert!(is_visible(&matrix, (0, 3)));
        assert!(is_visible(&matrix, (0, 4)));
        assert!(is_visible(&matrix, (1, 0)));
        assert!(is_visible(&matrix, (1, 1)));
        assert!(is_visible(&matrix, (1, 2)));
        assert!(!is_visible(&matrix, (1, 3)));
        assert!(is_visible(&matrix, (1, 4)));
        assert!(is_visible(&matrix, (2, 0)));
        assert!(is_visible(&matrix, (2, 1)));
        assert!(!is_visible(&matrix, (2, 2)));
        assert!(is_visible(&matrix, (2, 3)));
        assert!(is_visible(&matrix, (2, 4)));
        assert!(is_visible(&matrix, (3, 0)));
        assert!(!is_visible(&matrix, (3, 1)));
        assert!(is_visible(&matrix, (3, 2)));
        assert!(!is_visible(&matrix, (3, 3)));
        assert!(is_visible(&matrix, (3, 4)));
        assert!(is_visible(&matrix, (4, 0)));
        assert!(is_visible(&matrix, (4, 1)));
        assert!(is_visible(&matrix, (4, 2)));
        assert!(is_visible(&matrix, (4, 3)));
        assert!(is_visible(&matrix, (4, 4)));

        assert_eq!(viewing_distance(&matrix, (1, 2), Direction::North), 1);
        assert_eq!(viewing_distance(&matrix, (1, 2), Direction::West), 1);
        assert_eq!(viewing_distance(&matrix, (1, 2), Direction::East), 2);
        assert_eq!(viewing_distance(&matrix, (1, 2), Direction::South), 2);

        assert_eq!(scenic_score(&matrix, (1, 2)), 4);
        assert_eq!(scenic_score(&matrix, (3, 2)), 8);

        Ok(())
    }
}
