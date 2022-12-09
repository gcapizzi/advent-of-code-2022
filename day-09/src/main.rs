use anyhow::{anyhow, Result};
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl std::str::FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(anyhow!("unrecognised direction: '{}'", s)),
        }
    }
}

#[derive(Debug, Clone)]
struct Move {
    direction: Direction,
    length: u32,
}

impl Move {
    fn new(direction: Direction, length: u32) -> Move {
        Move { direction, length }
    }
}

impl std::str::FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_ascii_whitespace();

        let direction = words
            .next()
            .ok_or(anyhow!("invalid move: '{}'", s))?
            .parse()?;
        let length = words
            .next()
            .ok_or(anyhow!("invalid move: '{}'", s))?
            .parse()?;

        Ok(Move { direction, length })
    }
}

#[derive(Debug, PartialEq)]
struct Rope {
    knots: Vec<(i32, i32)>,
}

impl Rope {
    fn new<const N: usize>(knots: [(i32, i32); N]) -> Rope {
        Rope {
            knots: knots.to_vec(),
        }
    }

    fn tail(&self) -> (i32, i32) {
        self.knots.last().unwrap().clone()
    }

    fn mv(&self, direction: &Direction) -> Rope {
        let new_head = Self::mv_point(self.knots[0], direction);
        let mut new_knots = vec![new_head];
        for knot in self.knots.iter().skip(1) {
            new_knots.push(Self::mv_knot(*knot, *new_knots.last().unwrap()))
        }

        Rope { knots: new_knots }
    }

    fn mv_point((x, y): (i32, i32), direction: &Direction) -> (i32, i32) {
        match direction {
            Direction::Up => (x, y + 1),
            Direction::Down => (x, y - 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        }
    }

    fn mv_knot((x, y): (i32, i32), (hx, hy): (i32, i32)) -> (i32, i32) {
        let dx = hx - x;
        let dy = hy - y;
        if x == hx {
            if y < hy {
                return (x, hy - 1);
            }
            if y > hy {
                return (x, hy + 1);
            }
        } else if y == hy {
            if x < hx {
                return (hx - 1, y);
            }
            if x > hx {
                return (hx + 1, y);
            }
        } else if dx.abs() > 1 || dy.abs() > 1 {
            return (x + (dx / dx.abs()), y + (dy / dy.abs()));
        }

        (x, y)
    }
}

fn main() -> Result<()> {
    let input = read_input("input.txt")?;
    let moves = input
        .into_iter()
        .map(|l| l.parse::<Move>())
        .collect::<Result<Vec<Move>>>()?;

    dbg!(count_tail_positions(
        Rope::new([(0, 0), (0, 0)]),
        moves.clone()
    ));
    dbg!(count_tail_positions(
        Rope::new([
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0)
        ]),
        moves.clone()
    ));

    Ok(())
}

fn count_tail_positions<I: IntoIterator<Item = Move>>(start: Rope, moves: I) -> usize {
    let mut rope = start;
    let mut positions: HashSet<(i32, i32)> = HashSet::from([rope.tail()]);
    for m in moves {
        for _ in 0..m.length {
            rope = rope.mv(&m.direction);
            positions.insert(rope.tail());
        }
    }

    positions.len()
}

fn read_input<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<String>> {
    let file = File::open(path)?;
    BufReader::new(file).lines().map(|l| Ok(l?)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rope() {
        assert_eq!(
            Rope::new([(0, 0), (0, 0)]).mv(&Direction::Right),
            Rope::new([(1, 0), (0, 0)])
        );
        assert_eq!(
            Rope::new([(2, 1), (1, 1)]).mv(&Direction::Right),
            Rope::new([(3, 1), (2, 1)])
        );
        assert_eq!(
            Rope::new([(1, 2), (1, 3)]).mv(&Direction::Down),
            Rope::new([(1, 1), (1, 2)])
        );
        assert_eq!(
            Rope::new([(4, 0), (3, 0)]).mv(&Direction::Up),
            Rope::new([(4, 1), (3, 0)])
        );
        assert_eq!(
            Rope::new([(2, 2), (1, 1)]).mv(&Direction::Up),
            Rope::new([(2, 3), (2, 2)])
        );
        assert_eq!(
            Rope::new([(2, 2), (1, 1)]).mv(&Direction::Right),
            Rope::new([(3, 2), (2, 2)])
        );
    }

    #[test]
    fn test_count_positions() {
        assert_eq!(
            count_tail_positions(
                Rope::new([(0, 0), (0, 0)]),
                vec![
                    Move::new(Direction::Right, 4),
                    Move::new(Direction::Up, 4),
                    Move::new(Direction::Left, 3),
                    Move::new(Direction::Down, 1),
                    Move::new(Direction::Right, 4),
                    Move::new(Direction::Down, 1),
                    Move::new(Direction::Left, 5),
                    Move::new(Direction::Right, 2),
                ]
            ),
            13
        );
        assert_eq!(
            count_tail_positions(
                Rope::new([
                    (0, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0)
                ]),
                vec![
                    Move::new(Direction::Right, 4),
                    Move::new(Direction::Up, 4),
                    Move::new(Direction::Left, 3),
                    Move::new(Direction::Down, 1),
                    Move::new(Direction::Right, 4),
                    Move::new(Direction::Down, 1),
                    Move::new(Direction::Left, 5),
                    Move::new(Direction::Right, 2),
                ]
            ),
            1
        );
        assert_eq!(
            count_tail_positions(
                Rope::new([
                    (0, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0)
                ]),
                vec![
                    Move::new(Direction::Right, 5),
                    Move::new(Direction::Up, 8),
                    Move::new(Direction::Left, 8),
                    Move::new(Direction::Down, 3),
                    Move::new(Direction::Right, 17),
                    Move::new(Direction::Down, 10),
                    Move::new(Direction::Left, 25),
                    Move::new(Direction::Up, 20),
                ]
            ),
            36
        );
    }
}
