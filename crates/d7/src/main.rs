use std::usize;

use common::{matrix::FlatMatrix, read_input};

fn main() {
    read_input!(input);
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let matrix = FlatMatrix::new(input);
    let mut beams: Vec<Vec<bool>> = vec![vec![false; matrix.width()]; matrix.height() / 2];
    let mut splits: usize = 0;
    for y in 0..matrix.height() / 2 {
        for x in 0..matrix.width() {
            match matrix.at_coord(x, y * 2) {
                'S' => beams[y][x] = true,
                '^' => {
                    if beams[y - 1][x] {
                        beams[y][x - 1] = true;
                        beams[y][x + 1] = true;
                        splits += 1;
                    }
                }
                '.' => {
                    if y > 0 && beams[y - 1][x] {
                        beams[y][x] = true
                    }
                }
                _ => panic!("unexpected character"),
            }
        }
    }
    splits
}

fn part2(input: &str) -> usize {
    let matrix = FlatMatrix::new(input);
    let mut beams: Vec<Vec<usize>> = vec![vec![0; matrix.width()]; matrix.height() / 2];
    for y in 0..matrix.height() / 2 {
        for x in 0..matrix.width() {
            match matrix.at_coord(x, y * 2) {
                'S' => beams[y][x] = 1,
                '^' => {
                    if beams[y - 1][x] > 0 {
                        beams[y][x - 1] += beams[y - 1][x];
                        beams[y][x + 1] += beams[y - 1][x];
                    }
                }
                '.' => {
                    if y > 0 && beams[y - 1][x] > 0 {
                        beams[y][x] += beams[y - 1][x]
                    }
                }
                _ => panic!("unexpected character"),
            }
        }
    }
    beams.iter().last().unwrap().iter().sum()
}
