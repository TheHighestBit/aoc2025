use std::fs;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn read_input() -> String {
    fs::read_to_string("src/input.txt").unwrap()
}

fn main() {
    let input = read_input();

    println!("Part1: {}", day5::part1(&input));
    println!("Part2: {}", day5::part2(&input));
}
