use std::fs;

mod day1;

fn read_input() -> String {
    fs::read_to_string("src/input.txt").unwrap()
}

fn main() {
    let input = read_input();

    println!("Part1: {}", day1::part1(&input));
    println!("Part2: {}", day1::part2(&input));
}
