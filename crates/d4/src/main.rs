use std::usize;

use common::matrix::FlatMatrix;
use common::read_input;

fn main() {
    read_input!(input);
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let matrix = FlatMatrix::new(input);
    let mut acc = 0usize;
    for i in 0..matrix.size() {
        if matrix.at(i) == '@'
            && matrix
                .neighbors(i)
                .iter()
                .filter(|c| c.unwrap_or('.') == '@')
                .count()
                < 4
        {
            acc += 1;
        }
    }
    acc
}

fn part2(input: &str) -> usize {
    let mut matrix = FlatMatrix::new(input);
    let mut acc = 0usize;
    loop {
        let start = acc;
        for i in 0..matrix.size() {
            if matrix.at(i) == '@'
                && matrix
                    .neighbors(i)
                    .iter()
                    .filter(|c| c.unwrap_or('.') == '@')
                    .count()
                    < 4
            {
                matrix.update(i, '.');
                acc += 1;
            }
        }
        if start == acc {
            break;
        }
    }
    acc
}
