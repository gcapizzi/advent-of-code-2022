use anyhow::Result;
use std::collections::{HashSet, VecDeque};

struct Window<T> {
    size: usize,
    value: VecDeque<T>,
}

impl<T: Eq + std::hash::Hash> Window<T> {
    fn new(size: usize) -> Window<T> {
        Window {
            size,
            value: VecDeque::new(),
        }
    }

    fn push(&mut self, v: T) {
        self.value.push_back(v);
        if self.value.len() > self.size {
            self.value.pop_front();
        }
    }

    fn value_set(&self) -> HashSet<&T> {
        self.value.iter().collect()
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;

    dbg!(find_marker(input.chars(), 4));
    dbg!(find_marker(input.chars(), 14));

    Ok(())
}

fn find_marker<I: IntoIterator<Item = char>>(chars: I, size: usize) -> Option<usize> {
    let mut window = Window::new(size);
    for (c, i) in chars.into_iter().zip(1..) {
        window.push(c);
        if window.value_set().len() >= size {
            return Some(i);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::find_marker;

    #[test]
    fn test_find_marker() {
        for (buffer, window_size, marker_pos) in vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4, 7),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 4, 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 4, 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4, 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4, 11),
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14, 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 14, 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 14, 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14, 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14, 26),
        ] {
            assert_eq!(find_marker(buffer.chars(), window_size), Some(marker_pos));
        }
    }
}
