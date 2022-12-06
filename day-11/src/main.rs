use std::{cell::RefCell, rc::Rc};

use anyhow::Result;

use utils::files::read_file_string;

fn main() -> Result<()> {
    let input = read_file_string("day-11/input.test.txt")?;

    println!("Part 1 answer: {}", part_1(&input));

    println!("Part 2 answer: {}", part_2(&input));

    Ok(())
}

fn part_2(input: &String) -> usize {
    todo!()
}

fn part_1(input: &String) -> usize {
    let octopi: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    println!("{:?}", octopi);

    todo!();
}

fn convolve(
    matrix: &Vec<Vec<Rc<RefCell<u32>>>>,
    mut kernel: impl FnMut(&Vec<Vec<Rc<RefCell<u32>>>>, (usize, usize), Vec<(usize, usize)>),
) {
    for y in 0..matrix.len() {
        for x in 0..matrix[0].len() {
            convolve_at(matrix, &mut kernel, (x, y))
        }
    }
}

fn convolve_at(
    matrix: &Vec<Vec<Rc<RefCell<u32>>>>,
    kernel: &mut impl FnMut(&Vec<Vec<Rc<RefCell<u32>>>>, (usize, usize), Vec<(usize, usize)>),
    point: (usize, usize),
) {
    let neighbors = neighbors(&matrix, point);

    kernel(matrix, point, neighbors);
}

fn neighbors<T>(matrix: &Vec<Vec<T>>, point: (usize, usize)) -> Vec<(usize, usize)> {
    let (x, y) = point;
    let height = matrix.len() - 1;
    let width = matrix[0].len() - 1;
    let mut neighbors: Vec<(usize, usize)> = vec![];

    if x > 0 {
        if y > 0 {
            neighbors.push((x - 1, y - 1));
        }

        if y < height {
            neighbors.push((x - 1, y + 1));
        }

        neighbors.push((x - 1, y));
    }
    if y > 0 {
        if x < width {
            neighbors.push((x + 1, y - 1));
        }

        neighbors.push((x, y - 1));
    }
    if x < width {
        if y < height {
            neighbors.push((x + 1, y + 1));
        }

        neighbors.push((x + 1, y));
    }
    if y < height {
        neighbors.push((x, y + 1));
    }

    neighbors
}
