use anyhow::{anyhow, Result};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
enum Instruction {
    AddX(i32),
    Noop,
}

impl std::str::FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_ascii_whitespace();

        let instr = words.next().unwrap();
        match instr {
            "addx" => {
                let n = words.next().unwrap().parse()?;
                Ok(Instruction::AddX(n))
            }
            "noop" => Ok(Instruction::Noop),
            &_ => Err(anyhow!("unrecognised instruction: '{}'", instr)),
        }
    }
}

fn main() -> Result<()> {
    let input = read_input("input.txt")?;
    let instructions = input
        .map(|i| i.and_then(|ii| ii.parse()))
        .collect::<Result<Vec<Instruction>>>()?;

    let reg_values = registry_values(1, instructions);

    let total_signal_strength = 20 * reg_values[19]
        + 60 * reg_values[59]
        + 100 * reg_values[99]
        + 140 * reg_values[139]
        + 180 * reg_values[179]
        + 220 * reg_values[219];
    dbg!(total_signal_strength);

    print!("{}", render(reg_values));

    Ok(())
}

fn render<I: IntoIterator<Item = i32>>(reg_values: I) -> String {
    let mut screen = vec![];
    for (sprite_pos, cycle) in reg_values.into_iter().zip(0..) {
        let pixel = cycle % 40;
        if pixel == 0 {
            screen.push('\n');
        }
        if sprite_pos - 1 <= pixel && pixel <= sprite_pos + 1 {
            screen.push('#')
        } else {
            screen.push('.')
        }
    }
    screen.push('\n');
    screen.iter().collect()
}

fn registry_values<I: IntoIterator<Item = Instruction>>(
    init_value: i32,
    instructions: I,
) -> Vec<i32> {
    let mut x = init_value;
    let mut x_values = vec![];
    for i in instructions {
        match i {
            Instruction::AddX(n) => {
                x_values.push(x);
                x_values.push(x);
                x += n;
            }
            Instruction::Noop => x_values.push(x),
        }
    }
    x_values
}

fn read_input<P: AsRef<std::path::Path>>(path: P) -> Result<impl Iterator<Item = Result<String>>> {
    let file = File::open(path)?;
    Ok(BufReader::new(file).lines().map(|l| Ok(l?)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test() -> Result<()> {
        let instructions = vec![
            Instruction::AddX(15),
            Instruction::AddX(-11),
            Instruction::AddX(6),
            Instruction::AddX(-3),
            Instruction::AddX(5),
            Instruction::AddX(-1),
            Instruction::AddX(-8),
            Instruction::AddX(13),
            Instruction::AddX(4),
            Instruction::Noop,
            Instruction::AddX(-1),
            Instruction::AddX(5),
            Instruction::AddX(-1),
            Instruction::AddX(5),
            Instruction::AddX(-1),
            Instruction::AddX(5),
            Instruction::AddX(-1),
            Instruction::AddX(5),
            Instruction::AddX(-1),
            Instruction::AddX(-35),
            Instruction::AddX(1),
            Instruction::AddX(24),
            Instruction::AddX(-19),
            Instruction::AddX(1),
            Instruction::AddX(16),
            Instruction::AddX(-11),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::AddX(21),
            Instruction::AddX(-15),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::AddX(-3),
            Instruction::AddX(9),
            Instruction::AddX(1),
            Instruction::AddX(-3),
            Instruction::AddX(8),
            Instruction::AddX(1),
            Instruction::AddX(5),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Noop,
            Instruction::AddX(-36),
            Instruction::Noop,
            Instruction::AddX(1),
            Instruction::AddX(7),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Noop,
            Instruction::AddX(2),
            Instruction::AddX(6),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Noop,
            Instruction::AddX(1),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::AddX(7),
            Instruction::AddX(1),
            Instruction::Noop,
            Instruction::AddX(-13),
            Instruction::AddX(13),
            Instruction::AddX(7),
            Instruction::Noop,
            Instruction::AddX(1),
            Instruction::AddX(-33),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Noop,
            Instruction::AddX(2),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Noop,
            Instruction::AddX(8),
            Instruction::Noop,
            Instruction::AddX(-1),
            Instruction::AddX(2),
            Instruction::AddX(1),
            Instruction::Noop,
            Instruction::AddX(17),
            Instruction::AddX(-9),
            Instruction::AddX(1),
            Instruction::AddX(1),
            Instruction::AddX(-3),
            Instruction::AddX(11),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::AddX(1),
            Instruction::Noop,
            Instruction::AddX(1),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::AddX(-13),
            Instruction::AddX(-19),
            Instruction::AddX(1),
            Instruction::AddX(3),
            Instruction::AddX(26),
            Instruction::AddX(-30),
            Instruction::AddX(12),
            Instruction::AddX(-1),
            Instruction::AddX(3),
            Instruction::AddX(1),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Noop,
            Instruction::AddX(-9),
            Instruction::AddX(18),
            Instruction::AddX(1),
            Instruction::AddX(2),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::AddX(9),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Noop,
            Instruction::AddX(-1),
            Instruction::AddX(2),
            Instruction::AddX(-37),
            Instruction::AddX(1),
            Instruction::AddX(3),
            Instruction::Noop,
            Instruction::AddX(15),
            Instruction::AddX(-21),
            Instruction::AddX(22),
            Instruction::AddX(-6),
            Instruction::AddX(1),
            Instruction::Noop,
            Instruction::AddX(2),
            Instruction::AddX(1),
            Instruction::Noop,
            Instruction::AddX(-10),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::AddX(20),
            Instruction::AddX(1),
            Instruction::AddX(2),
            Instruction::AddX(2),
            Instruction::AddX(-6),
            Instruction::AddX(-11),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Noop,
        ];

        let reg_values = registry_values(1, instructions);

        assert_eq!(reg_values[19], 21);
        assert_eq!(reg_values[59], 19);
        assert_eq!(reg_values[99], 18);
        assert_eq!(reg_values[139], 21);
        assert_eq!(reg_values[179], 16);
        assert_eq!(reg_values[219], 18);

        assert_eq!(
            render(reg_values),
            r#"
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"#
        );

        Ok(())
    }
}
