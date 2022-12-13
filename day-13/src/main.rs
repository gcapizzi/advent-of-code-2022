use anyhow::{anyhow, Result};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    Integer(u32),
    List(Vec<Box<Value>>),
}

peg::parser! {
  grammar parser() for str {
    pub rule package() -> Value
      = "[" l:(value() ** ",") "]" { Value::List(l) }

    rule value() -> Box<Value>
      = integer() / list()

    rule integer() -> Box<Value>
      = n:$(['0'..='9']+) {? n.parse().or(Err("u32")).map(|nn| Box::new(Value::Integer(nn))) }

    rule list() -> Box<Value>
      = "[" l:(value() ** ",") "]" { Box::new(Value::List(l)) }
  }
}

impl std::str::FromStr for Value {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parser::package(s).map_err(|e| anyhow!(e))
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Integer(i) => write!(f, "{}", i),
            Value::List(l) => {
                if l.is_empty() {
                    return write!(f, "[]");
                }
                let (last, init) = l.split_last().unwrap();
                write!(
                    f,
                    "[{}{}]",
                    init.iter()
                        .map(|v| format!("{},", v))
                        .collect::<Vec<String>>()
                        .join(""),
                    last
                )
            }
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Value::Integer(l), Value::Integer(r)) => {
                if l == r {
                    None
                } else {
                    Some(l.cmp(&r))
                }
            }
            (l @ Value::Integer(_), r @ Value::List(_)) => {
                Value::List(vec![Box::new(l.clone())]).partial_cmp(r)
            }
            (l @ Value::List(_), r @ Value::Integer(_)) => {
                l.partial_cmp(&Value::List(vec![Box::new(r.clone())]))
            }
            (Value::List(l), Value::List(r)) => {
                if l.is_empty() && r.is_empty() {
                    return None;
                }

                if l.is_empty() {
                    return Some(std::cmp::Ordering::Less);
                }

                if r.is_empty() {
                    return Some(std::cmp::Ordering::Greater);
                }

                l[0].partial_cmp(&r[0]).or_else(|| {
                    Value::List(l[1..].to_vec()).partial_cmp(&Value::List(r[1..].to_vec()))
                })
            }
        }
    }
}

impl std::cmp::Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}

fn main() -> Result<()> {
    let input = read_input("input.txt")?;

    let sol1: usize = input
        .clone()
        .split(String::is_empty)
        .into_iter()
        .map(|p| Ok((p[0].parse()?, p[1].parse()?)))
        .filter_map(|p: Result<(Value, Value)>| p.ok())
        .enumerate()
        .filter(|(_, (l, r))| l < r)
        .map(|(i, _)| i + 1)
        .sum();
    dbg!(sol1);

    let mut input2 = input.clone();
    input2.push("[[2]]".to_string());
    input2.push("[[6]]".to_string());
    let mut values = input2
        .into_iter()
        .filter_map(|s| s.parse().ok())
        .collect::<Vec<Value>>();

    values.sort();
    let i1 = values
        .iter()
        .enumerate()
        .find(|(_, v)| v.to_string() == "[[2]]")
        .unwrap()
        .0;
    let i2 = values
        .iter()
        .enumerate()
        .find(|(_, v)| v.to_string() == "[[6]]")
        .unwrap()
        .0;
    let sol2 = (i1 + 1) * (i2 + 1);
    dbg!(sol2);

    Ok(())
}

fn read_input<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<String>> {
    let file = File::open(path)?;
    BufReader::new(file).lines().map(|l| Ok(l?)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_parse_and_display() -> Result<()> {
        assert_eq!("[1,1,3,1,1]".parse::<Value>()?.to_string(), "[1,1,3,1,1]");
        assert_eq!("[[1],4]".parse::<Value>()?.to_string(), "[[1],4]");
        Ok(())
    }

    #[test]
    fn test_ordering() -> Result<()> {
        assert!("[1,1,3,1,1]".parse::<Value>()? < "[1,1,5,1,1]".parse::<Value>()?);
        assert!("[[1],[2,3,4]]".parse::<Value>()? < "[[1],4]".parse::<Value>()?);
        assert!("[9]".parse::<Value>()? > "[[8,7,6]]".parse::<Value>()?);
        assert!("[[4,4],4,4]".parse::<Value>()? < "[[4,4],4,4,4]".parse::<Value>()?);
        assert!("[7,7,7,7]".parse::<Value>()? > "[7,7,7]".parse::<Value>()?);
        assert!("[]".parse::<Value>()? < "[3]".parse::<Value>()?);
        assert!("[[[]]]".parse::<Value>()? > "[[]]".parse::<Value>()?);
        assert!(
            "[1,[2,[3,[4,[5,6,7]]]],8,9]".parse::<Value>()?
                > "[1,[2,[3,[4,[5,6,0]]]],8,9]".parse::<Value>()?,
        );
        assert!("[1,2,3]".parse::<Value>()? == "[1,2,3]".parse::<Value>()?);
        Ok(())
    }
}
