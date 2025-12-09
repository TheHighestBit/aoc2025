#![allow(unused_variables)]
#![allow(dead_code)]

fn calc_area(p1: &(u64, u64), p2: &(u64, u64)) -> u64 {
    let x_vec = p1.0 as i64 - p2.0 as i64 + 1;
    let y_vec = p1.1 as i64 - p2.1 as i64 + 1;

    (x_vec.abs() * y_vec.abs()) as u64
}

pub fn part1(input: &str) -> u64 {
    let points: Vec<(u64, u64)> = input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            (
                parts.next().unwrap().parse::<u64>().unwrap(),
                parts.next().unwrap().parse::<u64>().unwrap(),
            )
        })
        .collect();

    // Super simple brute force since input is very small
    let mut largest = 0;
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let area = calc_area(&points[i], &points[j]);
            //println!("p1 {:?} p2 {:?} area {}", &points[i], &points[j], area);
            if area > largest {
                largest = area;
            }
        }
    }

    largest
}

pub fn part2(input: &str) -> i32 {
    todo!()
}
