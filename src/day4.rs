#![allow(unused_variables)]

pub fn part1(input: &str) -> i32 {
    let mut total = 0;
    let mut grid: Vec<Vec<char>> = Vec::new();

    input
        .lines()
        .for_each(|line| grid.push(line.chars().collect()));

    let w = grid[0].len();
    let h = grid.len();

    for (y, outer) in grid.iter().enumerate() {
        for (x, inner) in outer.iter().enumerate() {
            if inner == &'.' {
                continue;
            }

            let mut local = 0;

            // Check above
            if y > 0 {
                // Top left corner
                if x > 0 && grid[y - 1][x - 1] == '@' {
                    local += 1;
                }

                // Directly above
                if grid[y - 1][x] == '@' {
                    local += 1;
                }

                // Top right corner
                if x + 1 < w && grid[y - 1][x + 1] == '@' {
                    local += 1;
                }
            }

            // Check to the left
            if x > 0 {
                // Directly left
                if grid[y][x - 1] == '@' {
                    local += 1;
                }

                // Bottom left corner
                if y + 1 < h && grid[y + 1][x - 1] == '@' {
                    local += 1;
                }
            }

            // Check below
            if y + 1 < h {
                // Directly below
                if grid[y + 1][x] == '@' {
                    local += 1;
                }

                // Bottom right corner
                if x + 1 < w && grid[y + 1][x + 1] == '@' {
                    local += 1;
                }
            }

            // Check right
            if x + 1 < w && grid[y][x + 1] == '@' {
                local += 1;
            }

            if local < 4 {
                total += 1;
            }
        }
    }

    total
}

pub fn part2(input: &str) -> i32 {
    // The size of the input is so small that brute force is perfectly fine here
    let mut total = 0;
    let mut grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let w = grid[0].len();
    let h = grid.len();
    let mut roll_removed = true;

    while roll_removed {
        roll_removed = false;

        for y in 0..h {
            for x in 0..w {
                if grid[y][x] == '.' {
                    continue;
                }

                let mut local = 0;

                // Check above
                if y > 0 {
                    // Top left corner
                    if x > 0 && grid[y - 1][x - 1] == '@' {
                        local += 1;
                    }

                    // Directly above
                    if grid[y - 1][x] == '@' {
                        local += 1;
                    }

                    // Top right corner
                    if x + 1 < w && grid[y - 1][x + 1] == '@' {
                        local += 1;
                    }
                }

                // Check to the left
                if x > 0 {
                    // Directly left
                    if grid[y][x - 1] == '@' {
                        local += 1;
                    }

                    // Bottom left corner
                    if y + 1 < h && grid[y + 1][x - 1] == '@' {
                        local += 1;
                    }
                }

                // Check below
                if y + 1 < h {
                    // Directly below
                    if grid[y + 1][x] == '@' {
                        local += 1;
                    }

                    // Bottom right corner
                    if x + 1 < w && grid[y + 1][x + 1] == '@' {
                        local += 1;
                    }
                }

                // Check right
                if x + 1 < w && grid[y][x + 1] == '@' {
                    local += 1;
                }

                if local < 4 {
                    total += 1;
                    roll_removed = true;
                    grid[y][x] = '.';
                }
            }
        }
    }

    total
}
