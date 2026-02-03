use std::{str::FromStr, usize};

use common::read_input;

struct Input {
    fresh_ranges: Vec<(i64, i64)>,
    ingredients: Vec<i64>,
}

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (f, i) = s.split_once("\n\n").unwrap();
        let fresh_ranges = f
            .split('\n')
            .map(|r| {
                let (l, r) = r.split_once('-').unwrap();
                (l.parse::<i64>().unwrap(), r.parse::<i64>().unwrap())
            })
            .collect();
        let ingredients = i.split('\n').map(|i| i.parse().unwrap()).collect();

        Ok(Self {
            fresh_ranges,
            ingredients,
        })
    }
}

fn main() {
    read_input!(input);
    let mut parsed = input.parse().unwrap();
    println!("Part 1: {}", part1(&parsed));
    println!("Part 2: {}", part2(&mut parsed));
}

fn part1(input: &Input) -> usize {
    input
        .ingredients
        .iter()
        .filter(|i| {
            for r in input.fresh_ranges.iter() {
                if **i >= r.0 && **i <= r.1 {
                    return true;
                }
            }
            false
        })
        .count()
}

fn part2(input: &mut Input) -> usize {
    input.fresh_ranges.sort();
    let (mut curr_lo, mut curr_hi) = input.fresh_ranges[0];
    let mut total = 0usize;
    for (lo, hi) in input.fresh_ranges.iter().skip(1) {
        if *lo > curr_hi {
            total += (curr_hi - curr_lo) as usize + 1;
            curr_lo = *lo;
            curr_hi = *hi;
        } else if *hi > curr_hi {
            curr_hi = *hi;
        }
    }
    total += (curr_hi - curr_lo) as usize + 1;
    total
}
