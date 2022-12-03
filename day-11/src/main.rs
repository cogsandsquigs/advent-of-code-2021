use anyhow::Result;

use utils::files::read_file_string;

fn main() -> Result<()> {
    let input = read_file_string("day-10/input.txt")?;

    println!("Part 1 answer: {}", part_1(&input));

    println!("Part 2 answer: {}", part_2(&input));

    Ok(())
}

fn part_2(input: &String) -> usize {
    todo!()
}

fn part_1(input: &String) -> usize {
    todo!()
}
