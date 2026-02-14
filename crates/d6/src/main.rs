use std::usize;

use common::read_input;

enum Op {
    Add,
    Mul,
}

impl TryFrom<&str> for Op {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "+" => Ok(Self::Add),
            "*" => Ok(Self::Mul),
            _ => Err(value.to_string()),
        }
    }
}

struct Problem {
    nums: Vec<usize>,
    op: Op,
}

impl Problem {
    fn evaluate(&self) -> usize {
        match self.op {
            Op::Add => self.nums.iter().fold(0usize, |acc, num| acc + num),
            Op::Mul => self.nums.iter().fold(1usize, |acc, num| acc * num),
        }
    }
}

fn parse(input: &str) -> Vec<Problem> {
    let mut parts: Vec<Vec<&str>> = input
        .split('\n')
        .map(|line| line.split_whitespace().collect())
        .collect();
    let length = parts.first().unwrap().len();
    let ops = parts.remove(parts.len() - 1);
    (0..length)
        .map(|i| {
            let mut problem = Problem {
                nums: Vec::new(),
                op: ops[i].try_into().unwrap(),
            };
            for part in parts.iter() {
                problem.nums.push(part[i].parse().unwrap());
            }
            problem
        })
        .collect()
}

fn parse2(input: &str) -> Vec<Problem> {
    let lines: Vec<&str> = input.split('\n').collect();
    let ops = lines.last().unwrap();
    let mut col_indices = Vec::new();
    let mut prev = 0;
    let mut curr = 0;
    for i in 1..ops.len() {
        if &ops[i..=i] != " " {
            col_indices.push((prev, curr - 1));
            prev = i;
            curr = i;
        } else {
            curr += 1;
        }
    }
    col_indices.push((prev, curr));

    let mut problems = Vec::new();
    for (start, end) in col_indices {
        let mut nums = vec![0usize; end - start + 1];
        let op = ops[start..=start].try_into().unwrap();
        for line in lines.iter().take(lines.len() - 1) {
            for idx in (start..=end).rev() {
                let c = &line[idx..=idx];
                if c != " " {
                    nums[idx - start] *= 10;
                    nums[idx - start] += c.parse::<usize>().unwrap();
                }
            }
        }
        problems.push(Problem { nums, op });
    }
    problems
}

fn main() {
    read_input!(input);
    let problems = parse(input);
    println!("Part 1: {}", solve(&problems));
    let problems2 = parse2(input);
    println!("Part 2: {}", solve(&problems2));
}

fn solve(problems: &[Problem]) -> usize {
    problems
        .iter()
        .fold(0usize, |acc, problem| acc + problem.evaluate())
}
