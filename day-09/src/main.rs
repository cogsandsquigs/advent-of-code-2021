use anyhow::Result;
use std::collections::HashSet;
use utils::files::read_file_string;

fn main() -> Result<()> {
    let input = read_file_string("day-09/input.txt")?;

    // Convert to 2d array
    let heightmap: Vec<Vec<u32>> = input
        .split('\n')
        .map(|line| line.chars().map(|a| a.to_digit(10).unwrap()).collect())
        .collect();

    // Collect all the lowest pts
    let mut lowests: Vec<(usize, usize)> = vec![];

    for (y, row) in heightmap.iter().enumerate() {
        for (x, height) in row.iter().enumerate() {
            // The surrounding heights
            let surrounds: Vec<(usize, usize)> = neighbors(&heightmap, (x, y));

            if surrounds
                .iter()
                .all(|surround| height < &heightmap[surround.1][surround.0])
            {
                lowests.push((x, y));
            }
        }
    }

    println!(
        "Part 1 answer: {}",
        lowests
            .iter()
            .fold(0, |acc, pt| acc + heightmap[pt.1][pt.0] + 1)
    );

    let mut basins: Vec<usize> = vec![];

    // Use the lowest points to find the basins
    for start_pt in lowests {
        let size = bfs_basin(&heightmap, start_pt);
        basins.push(size);
    }

    // Sort the top 3
    basins.sort();
    basins.reverse();

    println!(
        "Part 2 answer: {}",
        basins.into_iter().take(3).fold(1, |acc, x| acc * x)
    );

    Ok(())
}

/// BFS to find the size and points of and in the basin, given an individual starting point.
/// Note that because all basins are separate, we only need to expect the starting point to
/// not be in any explored basin/point before. Updates `explored` with the new explored values
fn bfs_basin(heightmap: &Vec<Vec<u32>>, start_pt: (usize, usize)) -> usize {
    let mut explored: HashSet<(usize, usize)> = HashSet::new();
    let mut explore_queue: Vec<(usize, usize)> = vec![start_pt];
    let mut size = 0;

    while !explore_queue.is_empty() {
        let pt = explore_queue.pop().unwrap();

        if explored.contains(&pt) || heightmap[pt.1][pt.0] == 9 {
            continue;
        }

        explored.insert(pt);
        size += 1;

        // Add all the neighbors to the queue
        for neighbor in neighbors(heightmap, pt) {
            if !explored.contains(&neighbor) {
                explore_queue.push(neighbor);
            }
        }
    }

    size
}

/// Get the surrounding points in the heightmap
fn neighbors(heightmap: &Vec<Vec<u32>>, start_pt: (usize, usize)) -> Vec<(usize, usize)> {
    let (x, y) = start_pt;
    let mut neighbors: Vec<(usize, usize)> = vec![];

    // If we aren't idx 0 for x, we can get the prev. digit
    if x > 0 {
        neighbors.push((x - 1, y));
    }
    // If we aren't idx 0 for x, we can get the digit above us
    if y > 0 {
        neighbors.push((x, y - 1));
    }
    // If we aren't past row length, we can get next digit
    if x < heightmap[0].len() - 1 {
        neighbors.push((x + 1, y));
    }
    // If we aren't past height length, we can get below digit
    if y < heightmap.len() - 1 {
        neighbors.push((x, y + 1));
    }

    neighbors
}
