#![allow(unused_variables)]
#![allow(dead_code)]

use std::collections::HashMap;

fn find_routes(node: &str, graph: &HashMap<&str, Vec<&str>>) -> u32 {
    let mut total = 0;

    for out in graph.get(node).unwrap() {
        if out == &"out" {
            return 1;
        }

        total += find_routes(out, graph);
    }

    total
}

fn find_routes2<'a>(
    node: &'a str,
    graph: &HashMap<&'a str, Vec<&'a str>>,
    dac: bool,
    fft: bool,
    dp: &mut HashMap<(&'a str, bool, bool), u64>,
) -> u64 {
    if let Some(&cached) = dp.get(&(node, dac, fft)) {
        return cached;
    }

    let mut total: u64 = 0;

    for out in graph.get(node).unwrap() {
        if out == &"out" {
            let res = (dac && fft) as u64;
            dp.insert((node, dac, fft), res);
            return res;
        }

        let sub = find_routes2(out, graph, dac || node == "dac", fft || node == "fft", dp);
        total = total.saturating_add(sub);
    }

    dp.insert((node, dac, fft), total);
    total
}

pub fn part1(input: &str) -> u32 {
    let mut nodes: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in input.lines() {
        let (label, rest) = line.split_once(": ").unwrap();
        nodes.insert(label, rest.split(' ').collect());
    }

    find_routes("you", &nodes)
}

pub fn part2<'a>(input: &'a str) -> u64 {
    let mut nodes: HashMap<&'a str, Vec<&'a str>> = HashMap::new();

    for line in input.lines() {
        let (label, rest) = line.split_once(": ").unwrap();
        nodes.insert(label, rest.split(' ').collect::<Vec<&'a str>>());
    }

    let mut dp: HashMap<(&'a str, bool, bool), u64> = HashMap::new();
    find_routes2("svr", &nodes, false, false, &mut dp)
}
