use anyhow::Result;
use std::collections::HashMap;

fn main() -> Result<()> {
    let terminal_output = std::fs::read_to_string("input.txt")?;
    let dirs = parse_sizes(terminal_output);

    dbg!(dirs
        .iter()
        .filter(|(_, s)| s <= &&100000)
        .map(|(_, s)| s)
        .sum::<u32>());

    let unused_space = 70000000 - dirs.get("/").unwrap();
    let needed_space = 30000000 - unused_space;
    dbg!(dirs
        .iter()
        .filter(|(_, s)| s >= &&needed_space)
        .map(|(_, s)| s)
        .min());

    Ok(())
}

fn parse_sizes<S: AsRef<str>>(output: S) -> HashMap<String, u32> {
    let lines = output.as_ref().lines();
    let mut path = std::path::PathBuf::new();
    let mut sizes: HashMap<String, u32> = HashMap::new();

    for l in lines {
        if l == "" {
            continue;
        }

        if l == "$ cd .." {
            let dir_size = sizes
                .get(&path.to_str().unwrap().to_string())
                .unwrap_or(&0)
                .clone();
            path.pop();
            *sizes.entry(path.to_str().unwrap().to_string()).or_insert(0) += dir_size;
            continue;
        }

        if l.starts_with("$ cd ") {
            path.push(l.split_ascii_whitespace().nth(2).unwrap());
            continue;
        }

        if !l.starts_with("$") && !l.starts_with("dir ") {
            let size = l
                .split_ascii_whitespace()
                .next()
                .unwrap()
                .parse::<u32>()
                .unwrap();
            *sizes.entry(path.to_str().unwrap().to_string()).or_insert(0) += size;
        }
    }

    let dir_size = sizes
        .get(&path.to_str().unwrap().to_string())
        .unwrap_or(&0)
        .clone();
    path.pop();
    *sizes.entry(path.to_str().unwrap().to_string()).or_insert(0) += dir_size;

    sizes
}

#[cfg(test)]
mod tests {
    use super::parse_sizes;
    use anyhow::Result;
    use std::collections::HashMap;

    #[test]
    fn test() -> Result<()> {
        let terminal_output = r#"
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
"#;

        let dirs = parse_sizes(terminal_output);
        assert_eq!(
            dirs,
            HashMap::from([
                ("/a/e".to_string(), 584),
                ("/a".to_string(), 94853),
                ("/d".to_string(), 24933642),
                ("/".to_string(), 48381165),
            ])
        );

        Ok(())
    }
}
