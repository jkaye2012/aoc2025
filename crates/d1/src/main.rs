use common::read_input;

fn main() {
    read_input!(input);
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut dial = 50i16;
    let mut acc = 0usize;
    input.lines().for_each(|step| {
        let val = step[1..].parse::<i16>().unwrap()
            * ((step.chars().nth(0).unwrap() == 'L') as i16 * -2 + 1);
        dial += val;
        dial = dial.rem_euclid(100);
        acc += (dial == 0) as usize;
    });
    acc
}

fn part2(input: &str) -> usize {
    let mut dial = 50i16;
    let mut acc = 0usize;
    input.lines().for_each(|step| {
        let mut val: i16 = step[1..].parse().unwrap();
        acc += (val / 100) as usize;
        val %= 100;
        val *= (step.chars().nth(0).unwrap() == 'L') as i16 * -2 + 1;
        let orig = dial;
        dial += val;
        let prev = dial;
        dial = dial.rem_euclid(100);
        acc += (dial == 0 || (dial != prev && orig != 0)) as usize;
    });
    acc
}
