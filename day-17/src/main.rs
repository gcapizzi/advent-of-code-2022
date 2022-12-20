use anyhow::{anyhow, Result};
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, Clone)]
pub enum Movement {
    Left,
    Right,
}

impl TryFrom<char> for Movement {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '<' => Ok(Movement::Left),
            '>' => Ok(Movement::Right),
            _ => Err(anyhow!("unrecognised char: '{}'", c)),
        }
    }
}

#[derive(Debug, Clone)]
struct Piece {
    coords: Vec<(i32, i32)>,
}

impl Piece {
    fn new<const N: usize>(coords: [(i32, i32); N]) -> Piece {
        Piece {
            coords: Vec::from(coords),
        }
    }

    fn mv(&self, dx: i32, dy: i32) -> Piece {
        Piece {
            coords: self.coords.iter().map(|(x, y)| (x + dx, y + dy)).collect(),
        }
    }

    fn apply(&self, movement: Movement) -> Piece {
        let (dx, dy) = match movement {
            Movement::Left => (-1, 0),
            Movement::Right => (1, 0),
        };
        self.mv(dx, dy)
    }

    fn x_min(&self) -> i32 {
        *self.coords.iter().map(|(x, _)| x).min().unwrap()
    }

    fn x_max(&self) -> i32 {
        *self.coords.iter().map(|(x, _)| x).max().unwrap()
    }

    fn y_min(&self) -> i32 {
        *self.coords.iter().map(|(_, y)| y).min().unwrap()
    }

    fn y_max(&self) -> i32 {
        *self.coords.iter().map(|(_, y)| y).max().unwrap()
    }

    fn coord_set(&self) -> HashSet<(i32, i32)> {
        self.coords.iter().cloned().collect()
    }
}

fn main() -> Result<()> {
    let minus = Piece::new([(0, 0), (1, 0), (2, 0), (3, 0)]);
    let plus = Piece::new([(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)]);
    let rev_l = Piece::new([(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)]);
    let pipe = Piece::new([(0, 0), (0, 1), (0, 2), (0, 3)]);
    let square = Piece::new([(0, 0), (0, 1), (1, 0), (1, 1)]);

    let pieces = vec![minus, plus, rev_l, pipe, square];
    let input = read_input("input-small.txt")?;
    let n = 2022;

    let tower = simulate(pieces.iter(), input.iter(), n);
    let y_max = tower.iter().map(|(_, y)| y).max().unwrap();
    dbg!(y_max + 1);

    Ok(())
}

fn simulate<'a, P: Iterator<Item = &'a Piece> + Clone, M: Iterator<Item = &'a Movement> + Clone>(
    pieces: P,
    movements: M,
    n: usize,
) -> HashSet<(i32, i32)> {
    let mut tower: HashSet<(i32, i32)> = HashSet::new();
    let x_max = 7;
    let mut y_max = -1;
    let mut movements = movements.cycle();
    for piece in pieces.cycle().take(n) {
        let mut p = piece.clone().mv(2, y_max + 4);
        loop {
            let movement = movements.next().unwrap();

            let next_p = p.apply(movement.clone());
            if next_p.x_min() >= 0
                && next_p.x_max() <= x_max - 1
                && next_p.coord_set().is_disjoint(&tower)
            {
                p = next_p;
            }

            let next_p = p.mv(0, -1);
            if next_p.y_min() < 0 || !next_p.coord_set().is_disjoint(&tower) {
                tower.extend(&p.coords);
                y_max = std::cmp::max(y_max, p.y_max());
                break;
            } else {
                p = next_p;
            }
        }
    }
    tower
}

fn print_screen(tower: &HashSet<(i32, i32)>, piece: &Piece, x_max: i32, y_max: i32) {
    for y in (0..=y_max).rev() {
        for x in 0..=x_max {
            if tower.contains(&(x, y)) {
                print!("#");
            } else if piece.coord_set().contains(&(x, y)) {
                print!("@");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn read_input<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<Movement>> {
    let file = File::open(path)?;
    let line = BufReader::new(file)
        .lines()
        .next()
        .ok_or(anyhow!("empty input"))?;
    line?.chars().map(Movement::try_from).collect()
}
