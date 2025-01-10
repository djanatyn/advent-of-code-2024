fn main() {
    let input = include_str!("day01.input");
    let pairs = input.lines().map(
        |line| match line.split_whitespace().collect::<Vec<&str>>()[..] {
            [first, second] => (
                first.parse::<i64>().unwrap(),
                second.parse::<i64>().unwrap(),
            ),
            _ => panic!("failed"),
        },
    );
    let (mut left, mut right): (Vec<i64>, Vec<i64>) = pairs.unzip();
    left.sort();
    right.sort();

    let total_distance: i64 = left
        .iter()
        .zip(right.iter())
        .map(|(left, right)| (left - right).abs())
        .sum();

    println!("part 1: {}", total_distance);
}
