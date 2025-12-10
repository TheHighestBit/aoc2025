use std::collections::HashSet;

pub fn part1(input: &str) -> i32 {
    let mut total = 0;

    'outer: for line in input.lines() {
        let (diagram, rest) = line.split_once(' ').unwrap();
        let (button_str, _) = rest.rsplit_once(' ').unwrap();

        let buttons: Vec<u32> = button_str
            .split(' ')
            .map(|btn| {
                btn[1..btn.len() - 1]
                    .split(',')
                    .map(|p| 1 << p.parse::<u32>().unwrap())
                    .fold(0, |a, b| a | b)
            })
            .collect();

        let target: u32 = diagram
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == '#')
            .map(|(i, _)| 1 << (i - 1))
            .fold(0, |a, b| a | b);

        let mut states = HashSet::new();
        states.insert(0u32);

        for presses in 1.. {
            let frontier = std::mem::take(&mut states);
            for state in frontier {
                for &btn in &buttons {
                    let new_state = state ^ btn;
                    if new_state == target {
                        total += presses;
                        continue 'outer;
                    }
                    states.insert(new_state);
                }
            }
        }
    }

    total
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn solve_min_presses(buttons: &[Vec<usize>], target: &[i64]) -> i64 {
    let n_counters = target.len();
    let n_buttons = buttons.len();

    // Build augmented matrix [A | b]
    let mut matrix: Vec<Vec<i64>> = vec![vec![0; n_buttons + 1]; n_counters];
    for (j, btn) in buttons.iter().enumerate() {
        for &i in btn {
            if i < n_counters {
                matrix[i][j] = 1;
            }
        }
    }
    for i in 0..n_counters {
        matrix[i][n_buttons] = target[i];
    }

    // Gaussian elimination
    let mut pivot_cols = Vec::new();
    let mut pivot_row = 0;

    for col in 0..n_buttons {
        if let Some(r) = (pivot_row..n_counters).find(|&r| matrix[r][col] != 0) {
            matrix.swap(pivot_row, r);
            let pv = matrix[pivot_row][col];

            for r in 0..n_counters {
                if r != pivot_row && matrix[r][col] != 0 {
                    let f = matrix[r][col];
                    for c in 0..=n_buttons {
                        matrix[r][c] = matrix[r][c] * pv - matrix[pivot_row][c] * f;
                    }
                    let g = (0..=n_buttons).map(|c| matrix[r][c].abs()).fold(0, gcd);
                    if g > 1 {
                        for c in 0..=n_buttons {
                            matrix[r][c] /= g;
                        }
                    }
                }
            }
            pivot_cols.push(col);
            pivot_row += 1;
        }
    }

    let pivot_set: HashSet<usize> = pivot_cols.iter().cloned().collect();
    let free_vars: Vec<usize> = (0..n_buttons).filter(|c| !pivot_set.contains(c)).collect();
    let max_val = *target.iter().max().unwrap_or(&0);

    fn enumerate(
        free_vars: &[usize],
        idx: usize,
        free_vals: &mut Vec<i64>,
        matrix: &[Vec<i64>],
        pivot_cols: &[usize],
        n_buttons: usize,
        max_val: i64,
        best: &mut Option<i64>,
    ) {
        if idx == free_vars.len() {
            let mut sol = vec![0i64; n_buttons];
            for (i, &c) in free_vars.iter().enumerate() {
                sol[c] = free_vals[i];
            }
            for (row, &pc) in pivot_cols.iter().enumerate().rev() {
                let sum: i64 = (0..n_buttons)
                    .filter(|&c| c != pc)
                    .map(|c| matrix[row][c] * sol[c])
                    .sum();
                let num = matrix[row][n_buttons] - sum;
                let pv = matrix[row][pc];
                if num % pv != 0 {
                    return;
                }
                sol[pc] = num / pv;
            }
            if sol.iter().any(|&x| x < 0) {
                return;
            }
            let total: i64 = sol.iter().sum();
            if best.is_none_or(|b| total < b) {
                *best = Some(total);
            }
            return;
        }

        let current_sum: i64 = free_vals.iter().sum();
        if best.is_some_and(|b| current_sum >= b) {
            return;
        }

        for val in 0..=max_val {
            free_vals.push(val);
            enumerate(
                free_vars,
                idx + 1,
                free_vals,
                matrix,
                pivot_cols,
                n_buttons,
                max_val,
                best,
            );
            free_vals.pop();
            if best.is_some_and(|b| current_sum + val >= b) {
                break;
            }
        }
    }

    let mut best = None;
    enumerate(
        &free_vars,
        0,
        &mut Vec::new(),
        &matrix,
        &pivot_cols,
        n_buttons,
        max_val,
        &mut best,
    );
    best.unwrap()
}

// Full disclosure, this is Opus 4.5.
pub fn part2(input: &str) -> i64 {
    let mut total = 0;

    for line in input.lines() {
        let (_, rest) = line.split_once(' ').unwrap();
        let (button_str, joltage) = rest.rsplit_once(' ').unwrap();

        let buttons: Vec<Vec<usize>> = button_str
            .split(' ')
            .map(|btn| {
                btn[1..btn.len() - 1]
                    .split(',')
                    .map(|p| p.parse().unwrap())
                    .collect()
            })
            .collect();

        let target: Vec<i64> = joltage[1..joltage.len() - 1]
            .split(',')
            .map(|v| v.parse().unwrap())
            .collect();

        total += solve_min_presses(&buttons, &target);
    }

    total
}
