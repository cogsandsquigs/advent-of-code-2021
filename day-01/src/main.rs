use anyhow::Result;
use itertools::Itertools;
use utils::files::read_file_string;

fn main() -> Result<()> {
    let input = read_file_string("day-01/input.txt")?;

    println!("Part 1 answer: {}", part_1(&input));

    println!("Part 2 answer: {}", part_2(&input));

    Ok(())
}

fn part_2(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect_vec()
        .windows(3)
        .map(|list| list.iter().sum::<usize>())
        .tuple_windows()
        .map(|(a, b)| usize::from(a < b))
        .sum()
}

fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .tuple_windows()
        .map(|(a, b)| usize::from(a < b))
        .sum()
}
