use anyhow::Result;
use utils::files::read_file_string;

fn main() -> Result<()> {
    let input = read_file_string("day-09/input.txt")?;

    // Convert to 2d array
    let heightmap: Vec<Vec<u32>> = input
        .split('\n')
        .map(|line| line.chars().map(|a| a.to_digit(10).unwrap()).collect())
        .collect();

    // Collect all the lowest pts
    let mut lowests: Vec<u32> = vec![];

    for (y, row) in heightmap.iter().enumerate() {
        for (x, height) in row.iter().enumerate() {
            // The surrounding heights
            let mut surrounds: Vec<u32> = vec![];

            // If we aren't idx 0 for x, we can get the prev. digit
            if x > 0 {
                surrounds.push(row[x - 1]);
            }
            // If we aren't idx 0 for x, we can get the digit above us
            if y > 0 {
                surrounds.push(heightmap[y - 1][x]);
            }
            // If we aren't past row length, we can get next digit
            if x < row.len() - 1 {
                surrounds.push(row[x + 1]);
            }
            // If we aren't past height length, we can get below digit
            if y < heightmap.len() - 1 {
                surrounds.push(heightmap[y + 1][x]);
            }

            if surrounds.iter().all(|surround| height < surround) {
                lowests.push(*height);
            }
        }
    }

    println!(
        "Part 1 answer: {}",
        lowests.iter().fold(0, |acc, x| acc + x + 1)
    );

    Ok(())
}
