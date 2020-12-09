//! Solutions to 2020: Advent of Code day 9
//! By Peter Fornwall

use std::fs;

fn part1(v: &[i64], preamble: usize) -> i64 {
    let mut result = 0;

    for i in preamble..v.len() {
        let mut found = false;
        'outer: for x1 in (i - preamble)..i {
            for x2 in (x1 + 1)..i {
                let sum = v[x1] + v[x2];
                if sum == v[i] {
                    found = true;
                    break 'outer;
                }
            }
        }
        if !found {
            result = v[i];
        }
    }
    println!("Part1: {}", result);
    result
}

fn part2(v: &[i64], preamble: usize, search_sum: i64) -> i64 {
    let mut result: i64 = 0;

    'outer: for i in preamble..v.len() {
        for x1 in (i - preamble)..i {
            for x2 in (x1 + 1)..i {
                let sum: i64 = v[x1..x2].iter().sum();
                if sum == search_sum {
                    result = v[x1..x2].iter().min().unwrap() + v[x1..x2].iter().max().unwrap();
                    break 'outer;
                }
            }
        }
    }

    println!("Part2: {}", result);
    result
}

fn parse(content: &str) -> Vec<i64> {
    let v: Vec<i64> = content
        .trim()
        .split('\n')
        .map(|x| x.trim().parse().unwrap())
        .collect();
    v
}

fn main() {
    let content = fs::read_to_string("../../inputs/day9_input.txt").expect("Cannot open file!");
    let v = parse(&content);

    let result_p1 = part1(&v, 25);
    let _result_p2 = part2(&v, 25, result_p1);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "35
        20
        15
        25
        47
        40
        62
        55
        65
        95
        102
        117
        150
        182
        127
        219
        299
        277
        309
        576";
        let v = parse(input);
        let result = part1(&v, 5);
        assert_eq!(result, 127);
    }

    #[test]
    fn test_part2() {
        let input = "35
        20
        15
        25
        47
        40
        62
        55
        65
        95
        102
        117
        150
        182
        127
        219
        299
        277
        309
        576";
        let v = parse(input);
        let result_p1 = part1(&v, 5);
        let result = part2(&v, 5, result_p1);
        assert_eq!(result, 62);
    }
}
