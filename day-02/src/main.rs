use anyhow::{anyhow, Result};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, PartialEq, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl std::str::FromStr for Shape {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Shape::Rock),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Shape::Scissors),
            _ => Err(anyhow!("{} not valid", s)),
        }
    }
}

impl Shape {
    fn score(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn against(&self, other: &Shape) -> Outcome {
        if self == other {
            return Outcome::Draw;
        }
        match (self, other) {
            (Shape::Rock, Shape::Scissors) => Outcome::Win,
            (Shape::Paper, Shape::Rock) => Outcome::Win,
            (Shape::Scissors, Shape::Paper) => Outcome::Win,
            _ => Outcome::Lose,
        }
    }

    fn wins_against(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    }

    fn loses_against(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    }
}

#[derive(Debug, Clone)]
struct Round {
    me: Shape,
    opponent: Shape,
}

impl std::str::FromStr for Round {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (opponent, me) = s.split_once(' ').ok_or(anyhow!("{} is invalid", s))?;
        Ok(Round {
            me: me.parse()?,
            opponent: opponent.parse()?,
        })
    }
}

impl Round {
    fn score(&self) -> u32 {
        self.me.score() + self.me.against(&self.opponent).score()
    }
}

enum Outcome {
    Win,
    Draw,
    Lose,
}

impl std::str::FromStr for Outcome {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(anyhow!("{} not valid", s)),
        }
    }
}

impl Outcome {
    fn score(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }
}

struct NewRound {
    opponent: Shape,
    expected: Outcome,
}

impl std::str::FromStr for NewRound {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (opponent, expected) = s.split_once(' ').ok_or(anyhow!("{} is invalid", s))?;
        Ok(NewRound {
            opponent: opponent.parse()?,
            expected: expected.parse()?,
        })
    }
}

impl NewRound {
    fn score(&self) -> u32 {
        let me = match self.expected {
            Outcome::Win => self.opponent.loses_against(),
            Outcome::Draw => self.opponent.clone(),
            Outcome::Lose => self.opponent.wins_against(),
        };
        me.score() + self.expected.score()
    }
}

fn main() -> Result<()> {
    let input = read_input("input.txt")?;
    dbg!(solution_1(&input)?);
    dbg!(solution_2(&input)?);
    Ok(())
}

fn read_input<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<String>> {
    let file = File::open(path)?;
    BufReader::new(file).lines().map(|l| Ok(l?)).collect()
}

fn solution_1(input: &Vec<String>) -> Result<u32> {
    Ok(input
        .iter()
        .filter_map(|s| s.parse::<Round>().ok())
        .map(|r| r.score())
        .sum())
}

fn solution_2(input: &Vec<String>) -> Result<u32> {
    Ok(input
        .iter()
        .filter_map(|s| s.parse::<NewRound>().ok())
        .map(|r| r.score())
        .sum())
}

#[cfg(test)]
mod tests {
    use crate::{solution_1, solution_2};
    use anyhow::Result;

    #[test]
    fn test_solution1() -> Result<()> {
        assert_eq!(
            15,
            solution_1(&vec![
                "A Y".to_string(),
                "B X".to_string(),
                "C Z".to_string(),
            ])?
        );
        Ok(())
    }

    #[test]
    fn test_solution2() -> Result<()> {
        assert_eq!(
            12,
            solution_2(&vec![
                "A Y".to_string(),
                "B X".to_string(),
                "C Z".to_string(),
            ])?
        );
        Ok(())
    }
}
