#![allow(unused_variables)]
#![allow(dead_code)]

use std::collections::HashSet;

pub fn part1(input: &str) -> i32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let w = grid[0].len();
    let h = grid.len();
    let mut beam_pos = Vec::new();
    let mut visited = HashSet::new();
    let mut total = 0;

    beam_pos.push((grid[0].iter().position(|&el| el == 'S').unwrap(), 1));
    while let Some(current_beam) = beam_pos.pop() {
        if visited.contains(&current_beam) {
            continue;
        }
        visited.insert(current_beam);

        if grid[current_beam.1 + 1][current_beam.0] == '^' {
            total += 1;

            // Looking at the input the splitters can never be at the very edges
            beam_pos.push((current_beam.0 - 1, current_beam.1 + 1));
            beam_pos.push((current_beam.0 + 1, current_beam.1 + 1));
        } else if current_beam.1 < h - 2 {
            beam_pos.push((current_beam.0, current_beam.1 + 1));
        }
    }

    total
}

pub fn part2(input: &str) -> u64 {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let w = grid[0].len();
    let h = grid.len();
    let mut dp = vec![vec![0u64; w]; h];

    let s_pos = grid[0].iter().position(|&el| el == 'S').unwrap();
    dp[1][s_pos] = 1;
    grid[1][s_pos] = '|';

    for y in 1..h - 1 {
        for x in 0..w {
            if grid[y][x] == '|' {
                if grid[y + 1][x] == '^' {
                    // Left
                    dp[y + 1][x - 1] += dp[y][x];
                    grid[y + 1][x - 1] = '|';

                    // Right
                    dp[y + 1][x + 1] += dp[y][x];
                    grid[y + 1][x + 1] = '|';
                } else {
                    // Just straight down
                    dp[y + 1][x] += dp[y][x];
                    grid[y + 1][x] = '|';
                }
            }
        }
    }

    dp.last().unwrap().iter().sum()
}
