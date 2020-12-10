//! Solutions to 2020: Advent of Code day 10
//! By Peter Fornwall

use std::collections::HashMap;
use std::fs;

fn part1(v: &[i32]) -> i32 {
    let mut sorted: Vec<i32> = v.to_owned();
    sorted.sort();
    sorted.push(sorted.last().unwrap() + 3);

    let mut cur = 0;
    let mut m: HashMap<i32, i32> = HashMap::new();
    for x in sorted {
        let diff = x - cur;
        *m.entry(diff).or_insert(0) += 1;
        cur = x;
    }
    let result = m[&1] * m[&3];

    println!("Part1: {}", result);
    result
}

fn part2(v: &[i32]) -> i64 {
    let mut sorted: Vec<i32> = v.to_owned();
    sorted.push(0);
    sorted.sort();
    let last_value = sorted.last().unwrap() + 3;
    sorted.push(last_value);

    let mut m: HashMap<i32, i64> = HashMap::new();
    for x in &sorted {
        m.insert(*x, 0);
    }
    *m.get_mut(&0).unwrap() = 1;

    for x in sorted {
        let this_count = m[&x];
        for i in 1..=3 {
            let next = x + i;
            if let Some(entry) = m.get_mut(&next) {
                *entry += this_count;
            }
        }
    }

    let result: i64 = m[&last_value];
    println!("Part2: {}", result);
    result
}

fn parse(content: &str) -> Vec<i32> {
    let v: Vec<i32> = content
        .trim()
        .split('\n')
        .map(|x| x.trim().parse().unwrap())
        .collect();
    v
}

fn main() {
    let content = fs::read_to_string("../../inputs/day10_input.txt").expect("Cannot open file!");
    let v = parse(&content);

    let result_p1 = part1(&v);
    assert_eq!(result_p1, 1917);
    let result_p2 = part2(&v);
    assert_eq!(result_p2, 113387824750592);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "16
        10
        15
        5
        1
        11
        7
        19
        6
        12
        4";
        let v = parse(input);
        let result = part1(&v);
        assert_eq!(result, 35);
    }

    #[test]
    fn test_part2() {
        let input = "28
        33
        18
        42
        31
        14
        46
        20
        48
        47
        24
        23
        49
        45
        19
        38
        39
        11
        1
        32
        25
        35
        8
        17
        7
        9
        4
        2
        34
        10
        3";
        let v = parse(input);
        let result = part2(&v);
        assert_eq!(result, 19208);
    }
}
