use common::read_input;

fn main() {
    read_input!(input);
    println!("Part 1: {}", part1(parse(input)));
    println!("Part 2: {}", part2(parse(input)));
}

fn parse(input: &str) -> impl Iterator<Item = (usize, usize)> + use<'_> {
    input.split(',').map(|s| {
        let (b, e) = s.split_once('-').unwrap();
        (b.parse::<usize>().unwrap(), e.parse::<usize>().unwrap())
    })
}

fn repeat_score(i: usize) -> usize {
    let digits = i.checked_ilog10().unwrap_or(0) + 1;
    let div = 10usize.pow(digits / 2);
    (i / div == i % div) as usize * i
}

fn part1<I: Iterator<Item = (usize, usize)>>(input: I) -> usize {
    input.fold(0usize, |mut acc, (b, e)| {
        for i in b..=e {
            acc += repeat_score(i);
        }
        acc
    })
}

fn repeat_score_silly(i: usize) -> usize {
    let digits = i.checked_ilog10().unwrap_or(0) + 1;
    for digit in (1..=(digits / 2)).rev() {
        let div = 10usize.pow(digit);
        if digits % digit == 0 && silly_by_div(i, div) {
            return i;
        }
    }
    0
}

fn silly_by_div(mut i: usize, div: usize) -> bool {
    let target = i % div;
    i /= div;
    while i != 0 {
        if i % div != target {
            return false;
        }
        i /= div;
    }
    true
}

fn part2<I: Iterator<Item = (usize, usize)>>(input: I) -> usize {
    input.fold(0usize, |mut acc, (b, e)| {
        for i in b..=e {
            acc += repeat_score_silly(i);
        }
        acc
    })
}
