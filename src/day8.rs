#![allow(unused_variables)]
#![allow(dead_code)]

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

fn get_distance<T: Into<f64> + Copy>(p1: &(T, T, T), p2: &(T, T, T)) -> f64 {
    let dx = p1.0.into() - p2.0.into();
    let dy = p1.1.into() - p2.1.into();
    let dz = p1.2.into() - p2.2.into();
    (dx * dx + dy * dy + dz * dz).sqrt()
}

struct Pair {
    p1: (u32, u32, u32),
    p2: (u32, u32, u32),
    dist: f64,
}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
    }
}
impl Eq for Pair {}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // invert so BinaryHeap (max-heap) will pop the smallest distance
        other.dist.partial_cmp(&self.dist)
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

pub fn part1(input: &str) -> i32 {
    let mut heap = BinaryHeap::new(); // Max heap by default
    let mut circuits = HashMap::new();

    let boxes: Vec<(u32, u32, u32)> = input
        .lines()
        .map(|line| {
            let mut it = line.split(',').map(|v| v.trim().parse::<u32>().unwrap());
            (it.next().unwrap(), it.next().unwrap(), it.next().unwrap()) // Itertools would be nice...
        })
        .collect();

    // Assign an initial label to each point
    for (label, j_box) in boxes.iter().enumerate() {
        circuits.insert(j_box, label);
    }

    for i in 0..boxes.len() {
        for j in i + 1..boxes.len() {
            let pair = Pair {
                p1: boxes[i],
                p2: boxes[j],
                dist: get_distance(&boxes[i], &boxes[j]),
            };

            heap.push(pair);
        }
    }

    for _ in 0..1000 {
        let pair = heap.pop().unwrap();

        // Merge the two circuits (doesn't matter the label)
        let new_label = *circuits.get(&pair.p1).unwrap();
        let old_label = *circuits.get(&pair.p2).unwrap();

        if old_label != new_label {
            for val in circuits.values_mut() {
                if *val == old_label {
                    *val = new_label;
                }
            }
        }
    }

    // Calculate circuit sizes
    let mut counts: HashMap<usize, usize> = HashMap::new();
    for &label in circuits.values() {
        *counts.entry(label).or_insert(0) += 1;
    }

    let mut sizes: Vec<usize> = counts.into_values().collect();
    sizes.sort_unstable_by(|a, b| b.cmp(a));

    sizes.iter().take(3).product::<usize>() as i32
}

pub fn part2(input: &str) -> u64 {
    let mut heap = BinaryHeap::new(); // Max heap by default
    let mut circuits = HashMap::new();

    let boxes: Vec<(u32, u32, u32)> = input
        .lines()
        .map(|line| {
            let mut it = line.split(',').map(|v| v.trim().parse::<u32>().unwrap());
            (it.next().unwrap(), it.next().unwrap(), it.next().unwrap()) // Itertools would be nice...
        })
        .collect();

    // Assign an initial label to each point
    for (label, j_box) in boxes.iter().enumerate() {
        circuits.insert(j_box, label);
    }

    for i in 0..boxes.len() {
        for j in i + 1..boxes.len() {
            let pair = Pair {
                p1: boxes[i],
                p2: boxes[j],
                dist: get_distance(&boxes[i], &boxes[j]),
            };

            heap.push(pair);
        }
    }

    loop {
        let pair = heap.pop().unwrap();

        // Merge the two circuits (doesn't matter the label)
        let new_label = *circuits.get(&pair.p1).unwrap();
        let old_label = *circuits.get(&pair.p2).unwrap();

        if old_label != new_label {
            for val in circuits.values_mut() {
                if *val == old_label {
                    *val = new_label;
                }
            }
        }

        if circuits.values().copied().collect::<HashSet<usize>>().len() == 1 {
            return pair.p1.0 as u64 * pair.p2.0 as u64;
        }
    }
}
