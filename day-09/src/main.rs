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

    let mut explored = HashSet::new();
    let mut top_3: Vec<usize> = vec![];

    while let Some(start_pt) = next_pt(&heightmap, &explored) {
        let size = bfs_basin(&heightmap, start_pt, &mut explored);

        top_3.push(size);
        top_3.sort();
        top_3.reverse();
        top_3 = top_3.into_iter().take(3).collect();

        println!("{:?}", top_3);
    }

    println!(
        "Part 2 answer: {}",
        top_3.into_iter().fold(1, |acc, x| acc * x)
    );

    Ok(())
}

/// Gets the next point to explore
fn next_pt(
    heightmap: &Vec<Vec<u32>>,
    explored: &HashSet<(usize, usize)>,
) -> Option<(usize, usize)> {
    if explored.is_empty() {
        return Some((0, 0));
    }

    for point in explored {
        let neighbors = neighbors(heightmap, *point);

        for neighbor in neighbors {
            if !explored.contains(&neighbor) {
                return Some(neighbor);
            }
        }
    }

    None
}

/// BFS to find the size and points of and in the basin, given an individual starting point.
/// Note that because all basins are separate, we only need to expect the starting point to
/// not be in any explored basin/point before. Updates `explored` with the new explored values
fn bfs_basin(
    heightmap: &Vec<Vec<u32>>,
    start_pt: (usize, usize),
    explored: &mut HashSet<(usize, usize)>,
) -> usize {
    // Size of basin
    let mut size = 0;

    // List of points to explore
    let mut to_explore: Vec<(usize, usize)> = vec![start_pt];

    while !to_explore.is_empty() {
        let point = to_explore.pop().unwrap();
        explored.insert(point);

        // If it's 9 (or greater, should be impossible), then we know we can't explore
        // further than that.
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
    }

    size
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
