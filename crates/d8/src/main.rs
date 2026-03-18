use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    usize,
};

use common::{
    cartesian::{Point3D, PointDistance},
    read_input,
};

struct Circuits {
    circuits: HashMap<usize, HashSet<Point3D>>,
    assignments: HashMap<Point3D, usize>,
}

impl Circuits {
    pub fn new(points: &[Point3D]) -> Self {
        let mut circuits = HashMap::new();
        let mut assignments = HashMap::new();
        for (idx, point) in points.iter().enumerate() {
            assignments.insert(*point, idx);
            let mut set = HashSet::new();
            set.insert(*point);
            circuits.insert(idx, set);
        }
        Self {
            circuits,
            assignments,
        }
    }

    pub fn connect(&mut self, a: &Point3D, b: &Point3D) {
        let a_assn = self.assignments.get(&a).unwrap().clone();
        let b_assn = self.assignments.get(&b).unwrap().clone();
        if a_assn != b_assn {
            let b_set = self.circuits.get(&b_assn).unwrap().clone();
            self.circuits.get_mut(&a_assn).unwrap().extend(&b_set);
            for (_, v) in self.assignments.iter_mut() {
                if *v == b_assn {
                    *v = a_assn;
                }
            }
            self.circuits.remove(&b_assn);
        }
    }

    pub fn score(&self) -> usize {
        let mut values: Vec<&HashSet<Point3D>> = self.circuits.values().collect();
        values.sort_by(|a, b| b.len().cmp(&a.len()));
        values.iter().take(3).fold(1, |acc, s| acc * s.len())
    }
}

fn main() {
    read_input!(input);
    let points: Vec<Point3D> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut heap = BinaryHeap::new();
    for idx in 0..points.len() {
        let a = points[idx];
        for b in points.iter().skip(idx + 1) {
            heap.push(Reverse(PointDistance::new(a, *b)));
        }
    }

    println!("Part 1: {}", part1(heap.clone(), Circuits::new(&points)));
    println!("Part 2: {}", part2(heap, Circuits::new(&points)));
}

fn part1(mut heap: BinaryHeap<Reverse<PointDistance>>, mut circuits: Circuits) -> usize {
    for _ in 0..1000 {
        let pd = heap.pop().unwrap();
        circuits.connect(pd.0.a(), pd.0.b());
    }

    circuits.score()
}

fn part2(mut heap: BinaryHeap<Reverse<PointDistance>>, mut circuits: Circuits) -> i64 {
    loop {
        let pd = heap.pop().unwrap();
        circuits.connect(pd.0.a(), pd.0.b());
        if circuits.circuits.len() == 1 {
            return pd.0.a().x() * pd.0.b().x();
        }
    }
}
