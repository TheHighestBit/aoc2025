#![allow(unused_variables)]

fn next_power_of_10(input: u64) -> u64 {
    if input == 0 {
        return 1;
    }
    let mut power: u64 = 1;
    while power <= input {
        power = power
            .checked_mul(10)
            .expect("overflow in finding next power of 10!!!");
    }

    power
}

pub fn part1(input: &str) -> u64 {
    let mut total: u64 = 0;

    for part in input.split(',') {
        let mut iter = part.split('-');

        let start = iter.next().unwrap();
        let mut start_num: u64 = start.parse().unwrap();
        let end = iter.next().unwrap();
        let end_num: u64 = end.parse().unwrap();

        while start_num <= end_num {
            let digits = start_num.ilog10() + 1;

            if !digits.is_multiple_of(2) {
                start_num = next_power_of_10(start_num);
                continue;
            }

            let divisor = 10u64.pow(digits / 2);
            let left = start_num / divisor;
            let right = start_num % divisor;

            if left == right {
                total += start_num;
            }

            start_num += 1;
        }
    }

    total
}

pub fn part2(input: &str) -> u64 {
    let mut total: u64 = 0;

    for part in input.split(',') {
        let mut iter = part.split('-');

        let start = iter.next().unwrap();
        let mut start_num: u64 = start.parse().unwrap();
        let end = iter.next().unwrap();
        let end_num: u64 = end.parse().unwrap();

        while start_num <= end_num {
            let digits: usize = (start_num.ilog10() + 1) as usize;
            let start_num_str = start_num.to_string();
            let mut slice_len: usize = 1;

            while slice_len <= digits / 2 {
                let slice = &start_num_str[..slice_len];
                let test_str = slice.repeat(digits / slice_len);

                if start_num_str == test_str {
                    total += start_num;
                    break;
                }

                slice_len += 1;
                while !digits.is_multiple_of(slice_len) && slice_len <= digits / 2 {
                    slice_len += 1;
                }
            }

            start_num += 1;
        }
    }

    total
}
