use std::collections::HashMap;

fn main() {
    let input = include_str!("day01.input");

    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}

fn parse(input: &str) -> (Vec<i64>, Vec<i64>) {
    let pairs = input.lines().map(
        |line| match line.split_whitespace().collect::<Vec<&str>>()[..] {
            [first, second] => (
                first.parse::<i64>().unwrap(),
                second.parse::<i64>().unwrap(),
            ),
            _ => panic!("failed"),
        },
    );
    pairs.unzip()
}

fn part1(input: &str) -> i64 {
    let (mut left, mut right) = parse(input);
    left.sort();
    right.sort();

    let total_distance: i64 = left
        .iter()
        .zip(right.iter())
        .map(|(left, right)| (left - right).abs())
        .sum();

    total_distance
}

fn part2(input: &str) -> i64 {
    let (left, right) = parse(input);
    let mut counts: HashMap<i64, i64> = HashMap::new();
    right.iter().for_each(|num| {
        counts
            .entry(*num)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    });
    let total_similarity: i64 = left
        .iter()
        .map(|search| {
            let count = counts.get(search).unwrap_or(&0);
            count * search
        })
        .sum();

    total_similarity
}
