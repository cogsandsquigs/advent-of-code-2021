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
            let surrounds: Vec<u32> = neighbors(&heightmap, (x, y))
                .into_iter()
                .map(|(x, y)| heightmap[y][x])
                .collect();

            if surrounds.iter().all(|surround| height < surround) {
                lowests.push(*height);
            }
        }
    }

    println!(
        "Part 1 answer: {}",
        lowests.iter().fold(0, |acc, x| acc + x + 1)
    );

    println!("{:?}", bfs_basin(&heightmap, (0, 0), &vec![]));

    Ok(())
}

/// BFS to find the size and points of and in the basin, given an individual starting point.
/// Note that because all basins are separate, we only need to expect the starting point to
/// not be in any explored basin/point before.
fn bfs_basin(
    heightmap: &Vec<Vec<u32>>,
    start_pt: (usize, usize),
    explored: &Vec<(usize, usize)>,
) -> (usize, Vec<(usize, usize)>) {
    // Size of basin
    let mut size = 0;

    // List of points to explore
    let mut to_explore: Vec<(usize, usize)> = vec![start_pt];

    // List of points we have explored
    let mut explored: Vec<(usize, usize)> = explored.to_vec();

    while !to_explore.is_empty() {
        let point = to_explore.pop().unwrap();

        if heightmap[point.1][point.0] >= 9 {
            continue;
        }

        size += 1;

        // Get its neighbors
        let neighbors = neighbors(heightmap, point);

        for neighbor in neighbors {
            if !explored.contains(&neighbor) {
                to_explore.push(neighbor);
            }
        }

        explored.push(point);

        println!("{:?}", point);
    }

    (size, explored)
}

/// Get the surrounding points in the heightmap
fn neighbors(heightmap: &Vec<Vec<u32>>, start_pt: (usize, usize)) -> Vec<(usize, usize)> {
    let (x, y) = start_pt;
    let mut surrounds: Vec<(usize, usize)> = vec![];

    // If we aren't idx 0 for x, we can get the prev. digit
    if x > 0 {
        surrounds.push((x - 1, y));
    }
    // If we aren't idx 0 for x, we can get the digit above us
    if y > 0 {
        surrounds.push((x, y - 1));
    }
    // If we aren't past row length, we can get next digit
    if x < heightmap[0].len() - 1 {
        surrounds.push((x + 1, y));
    }
    // If we aren't past height length, we can get below digit
    if y < heightmap.len() - 1 {
        surrounds.push((x, y + 1));
    }

    surrounds
}
