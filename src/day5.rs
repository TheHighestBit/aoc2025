#![allow(unused_variables)]
#![allow(dead_code)]

pub fn part1(input: &str) -> i32 {
    let mut ranges = Vec::new();
    let mut ingredients_start = 0;
    let mut fresh = 0;

    for (i, line) in input.lines().enumerate() {
        if !line.is_empty() {
            let mut parts = line.split('-');

            ranges.push((
                parts.next().unwrap().parse::<u64>().unwrap(),
                parts.next().unwrap().parse::<u64>().unwrap(),
            ));
        } else {
            ingredients_start = i + 1;
            break;
        }
    }

    let ingredients: Vec<u64> = input
        .lines()
        .skip(ingredients_start)
        .map(|val| val.parse::<u64>().unwrap())
        .collect();

    for ingredient in &ingredients {
        for range in &ranges {
            if &range.0 <= ingredient && ingredient <= &range.1 {
                fresh += 1;
                break;
            }
        }
    }

    fresh
}

pub fn part2(input: &str) -> u64 {
    let mut ranges = Vec::new();
    let mut fresh = 0;

    for (i, line) in input.lines().enumerate() {
        if !line.is_empty() {
            let mut parts = line.split('-');

            ranges.push((
                parts.next().unwrap().parse::<u64>().unwrap(),
                parts.next().unwrap().parse::<u64>().unwrap(),
            ));
        } else {
            break;
        }
    }

    ranges.sort();

    let mut ranges_merged = Vec::new();
    let mut merge_performed = true;

    // There is a much better way to do this by keeping the current range as a separate variable and treating
    // other ranges as a stack. Pop off ranges 1 by 1 and merge if possible. If not, put the current range
    // into the ready vec and treat the range we just popped off as the new current range.

    while merge_performed {
        merge_performed = false;
        let mut i = 0;

        while i < ranges.len() {
            let current = ranges[i];

            // Last range, nothing to merge with
            if i == ranges.len() - 1 {
                ranges_merged.push(current);
                break;
            }

            let next = ranges[i + 1];

            if next.0 <= current.1 {
                ranges_merged.push((current.0, current.1.max(next.1)));
                merge_performed = true;
                i += 1;
            } else {
                ranges_merged.push(current);
            }

            i += 1;
        }

        ranges = ranges_merged.clone();
        ranges_merged.clear();
    }

    for range in ranges {
        fresh += range.1 - range.0 + 1;
    }

    fresh
}
