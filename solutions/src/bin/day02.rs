#![feature(iter_map_windows)]
use std::cmp::Ordering;

#[derive(Debug)]
struct Levels(Vec<i64>);

impl Levels {
    fn parse(line: &str) -> Self {
        Self(
            line.split_whitespace()
                .filter_map(|level| level.parse::<i64>().ok())
                .collect(),
        )
    }

    fn monotonic(&self) -> bool {
        let mut ordering: Vec<Ordering> = self
            .0
            .iter()
            .map_windows(|[a, b]| a.partial_cmp(b).unwrap())
            .collect();
        ordering.dedup();
        ordering == vec![Ordering::Less] || ordering == vec![Ordering::Greater]
    }

    fn adjacency_check(&self) -> bool {
        self.0
            .iter()
            .map_windows(|[a, b]| (*a - *b).abs())
            .all(|d| (1..=3).contains(&d))
    }

    fn safe(&self) -> bool {
        self.monotonic() && self.adjacency_check()
    }

    fn remove_index(&self, removed_index: usize) -> Self {
        let levels = self
            .0
            .iter()
            .enumerate()
            .filter_map(|(idx, lvl)| {
                if idx == removed_index {
                    None
                } else {
                    Some(*lvl)
                }
            })
            .collect();
        Self(levels)
    }

    // part two
    fn dampened_safe(&self) -> bool {
        let mut indexes = 0..self.0.len();
        self.safe() || indexes.any(|idx| self.remove_index(idx).safe())
    }
}

#[derive(Debug)]
struct Reports(Vec<Levels>);

impl Reports {
    fn parse(input: &str) -> Self {
        Self(input.lines().map(Levels::parse).collect())
    }
}

fn part1(input: &Reports) -> usize {
    input.0.iter().filter(|levels| levels.safe()).count()
}

fn part2(input: &Reports) -> usize {
    input
        .0
        .iter()
        .filter(|levels| levels.dampened_safe())
        .count()
}

fn main() {
    let input = Reports::parse(include_str!("day02.input"));

    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let safety: Vec<bool> = Reports::parse(
            r#"
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"#
            .trim(),
        )
        .0
        .iter()
        .map(|l| l.safe())
        .collect();
        assert_eq!(safety, vec![true, false, false, false, false, true]);
    }
}
