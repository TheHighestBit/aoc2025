#![allow(unused_variables)]
#![allow(dead_code)]

pub fn part1(input: &str) -> u64 {
    let mut lines = input.lines().rev();

    // First get the operator for column so we can keep running totals
    // I think this is slightly slower but uses less memory

    let operators: Vec<&str> = lines.next().unwrap().split_whitespace().collect();
    let mut totals: Vec<u64> = operators
        .iter()
        .map(|op| match *op {
            "+" => 0,
            "*" => 1,
            _ => panic!("Something aint right here..."),
        })
        .collect();

    for line in lines {
        let numbers = line
            .split_whitespace()
            .map(|val| val.parse::<u64>().unwrap());

        for (i, val) in numbers.enumerate() {
            match operators[i] {
                "+" => totals[i] += val,
                "*" => totals[i] *= val,
                _ => panic!("Something aint right here..."),
            }
        }
    }

    totals.iter().sum()
}

pub fn part2(input: &str) -> u64 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let w = grid[0].len();
    let h = grid.len();
    let mut total = 0;

    // Find separating indexes (full columns... life is hard when you can't read)
    let mut indexes = Vec::new();
    'outer: for x in 0..grid[0].len() {
        let tmp = 0..grid.len();
        for y in tmp {
            if grid[y][x] != ' ' {
                continue 'outer;
            }
        }

        indexes.push(x);
    }

    // Add last one separately
    indexes.push(w);

    let mut i_prev = 0;
    for i in indexes {
        let operator = grid[h - 1][i_prev];
        let mut running_total: u64 = (operator == '*') as u64;

        let mut j = i as i32 - 1;
        while j >= i_prev as i32 {
            let mut current_val = String::with_capacity(h);

            (0..h - 1).for_each(|y| {
                if grid[y][j as usize] != ' ' {
                    current_val.push(grid[y][j as usize]);
                }
            });

            match operator {
                '*' => running_total *= current_val.parse::<u64>().unwrap(),
                '+' => running_total += current_val.parse::<u64>().unwrap(),
                _ => panic!("SOMETHING IS VERY BAD! HELP PLS!"),
            }

            j -= 1;
        }

        i_prev = i + 1;
        total += running_total;
    }

    total
}
