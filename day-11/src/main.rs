use std::{cell::RefCell, rc::Rc};

use anyhow::Result;

use utils::files::read_file_string;

fn main() -> Result<()> {
    let input = read_file_string("day-11/input.txt")?;

    println!("Part 1 answer: {}", part_1(&input));

    println!("Part 2 answer: {}", part_2(&input));

    Ok(())
}

fn part_2(input: &String) -> usize {
    todo!()
}

fn part_1(input: &String) -> usize {
    let octopi: Vec<Vec<Rc<RefCell<u32>>>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Rc::new(RefCell::new(c.to_digit(10).unwrap())))
                .collect()
        })
        .collect();

    let mut num_flashes = 0;

    convolve_in_place(&octopi, |center, neighbors| {
        // Increment center
        center.replace_with(|&mut x| x + 1);

        if *center.borrow() > 9 {
            num_flashes += 1;

            neighbors.into_iter().map(|loc| {
                loc.replace_with(|&mut x| x + 1);
                loc
            });

            center.replace(0);
        }
    });

    println!("{:?}", octopi);

    todo!();
}

fn convolve_in_place<T: Clone>(
    matrix: &Vec<Vec<Rc<RefCell<T>>>>,
    mut kernel: impl FnMut(Rc<RefCell<T>>, Vec<Rc<RefCell<T>>>),
) {
    let height = matrix.len();
    let width = matrix[0].len();
    for y in 0..height {
        for x in 0..width {
            let center = matrix[y][x].clone();
            let neighbors: Vec<Rc<RefCell<T>>> = neighbors(&matrix, x, y);

            kernel(center, neighbors)
        }
    }
}

fn neighbors<T>(matrix: &Vec<Vec<Rc<RefCell<T>>>>, x: usize, y: usize) -> Vec<Rc<RefCell<T>>> {
    let height = matrix.len() - 1;
    let width = matrix[0].len() - 1;
    let mut neighbors: Vec<Rc<RefCell<T>>> = vec![];

    if x > 0 {
        if y > 0 {
            neighbors.push(matrix[y - 1][x - 1].clone());
        }

        if y < height {
            neighbors.push(matrix[y + 1][x - 1].clone());
        }

        neighbors.push(matrix[y][x - 1].clone());
    }
    if y > 0 {
        if x < width {
            neighbors.push(matrix[y - 1][x + 1].clone());
        }

        neighbors.push(matrix[y - 1][y].clone());
    }
    if x < width {
        if y < height {
            neighbors.push(matrix[y + 1][x + 1].clone());
        }

        neighbors.push(matrix[y][x + 1].clone());
    }
    if y < height {
        neighbors.push(matrix[y + 1][x].clone());
    }

    neighbors
}
