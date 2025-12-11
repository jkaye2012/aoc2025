use common::read_input;

fn main() {
    read_input!(input);
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn joltage(battery: &str, mut digits: u32) -> usize {
    let mut start_idx = 0;
    let mut result = 0usize;
    let bytes = battery.as_bytes();
    while digits >= 1 {
        let mut max = '0';
        for idx in (start_idx..=(battery.len() - digits as usize)).rev() {
            if bytes[idx] as char >= max {
                max = bytes[idx] as char;
                start_idx = idx + 1;
            }
        }
        result += (max as usize - '0' as usize) * 10usize.pow(digits - 1);
        digits -= 1;
    }
    result
}
fn part1(input: &str) -> usize {
    input
        .lines()
        .fold(0usize, |acc, line| acc + joltage(line, 2))
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .fold(0usize, |acc, line| acc + joltage(line, 12))
}
