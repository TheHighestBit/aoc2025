use std::fs;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn read_input() -> String {
    fs::read_to_string("src/input.txt").unwrap()
}

fn main() {
    let input = read_input();

    println!("Part1: {}", day9::part1(&input));
    println!("Part2: {}", day9::part2(&input));
}
