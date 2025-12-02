pub fn part1(input: &str) -> i32 {
    let mut dial = 50;
    let mut total_zeros = 0;

    for line in input.lines() {
        let rot: i32 = line[1..].parse().unwrap();

        match line.chars().next().unwrap() {
            'L' => {
                dial -= rot;
            }
            'R' => {
                dial += rot;
            }
            _ => panic!("Should not happen!"),
        }

        dial %= 100;

        if dial < 0 {
            dial += 100
        }

        if dial == 0 {
            total_zeros += 1;
        };
    }

    total_zeros
}

pub fn part2(input: &str) -> i32 {
    let mut dial = 50;
    let mut total_zeros = 0;

    for line in input.lines() {
        let mut rot: i32 = line[1..].parse().unwrap();
        total_zeros += rot / 100;
        rot %= 100;

        match line.chars().next().unwrap() {
            'L' => {
                if dial == 0 {
                    dial = (100 - rot).min(99);
                    continue;
                }

                dial -= rot;
                if dial <= 0 {
                    total_zeros += 1;
                }

                if dial < 0 {
                    dial = (100 + dial).min(99);
                }
            }
            'R' => {
                dial += rot;

                if dial >= 100 {
                    total_zeros += 1;
                    dial -= 100;
                }
            }
            _ => panic!("Should not happen!"),
        }
    }

    total_zeros
}
