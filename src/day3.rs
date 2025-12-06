#![allow(unused_variables)]
#![allow(dead_code)]

pub fn part1(input: &str) -> u32 {
    let mut total = 0;
    for bank in input.lines() {
        let mut largest: Option<char> = None;
        let mut largest_after = None;

        for (index, chr) in bank.char_indices() {
            if let Some(large) = largest {
                if chr > large && index != bank.len() - 1 {
                    largest = Some(chr);
                    largest_after = None;
                    continue;
                }

                if let Some(large_after) = largest_after {
                    if chr > large_after {
                        largest_after = Some(chr);
                    }
                } else {
                    largest_after = Some(chr);
                }
            } else {
                largest = Some(chr);
            }
        }

        total += largest.unwrap().to_digit(10).unwrap() * 10
            + largest_after.unwrap().to_digit(10).unwrap();
    }

    total
}

pub fn part2(input: &str) -> u64 {
    let mut total = 0;
    for bank in input.lines() {
        let mut values: [u32; 12] = [0; 12];
        let mut last_index = 0;

        for i in (1..13).rev() {
            let mut largest = None;
            let mut last_index_local = last_index;

            for (index, chr) in bank[last_index..bank.len() - i + 1].char_indices() {
                if let Some(large) = largest {
                    if chr > large {
                        largest = Some(chr);
                        last_index_local = last_index + index + 1;
                    }
                } else {
                    largest = Some(chr);
                    last_index_local = last_index + index + 1;
                }
            }

            last_index = last_index_local;
            values[12 - i] = largest.unwrap().to_digit(10).unwrap();
        }

        let mut value: u64 = 0;
        (0..12).for_each(|i| {
            value += values[i] as u64 * 10u64.pow(11 - i as u32);
        });

        total += value;
    }

    total
}
