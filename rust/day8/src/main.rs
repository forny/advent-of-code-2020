//! Solutions to 2020: Advent of Code day 8
//! By Peter Fornwall

use std::collections::HashSet;
use std::fs;

fn run(v: &[(&str, i32)], swap_index: usize) -> (bool, i32) {
    let mut seen: HashSet<usize> = HashSet::new();
    let mut acc = 0;
    let mut ip = 0usize;
    let mut finished = false;
    loop {
        if seen.contains(&ip) {
            break;
        }
        seen.insert(ip);
        let mut inst = v[ip];
        if swap_index == ip {
            if inst.0 == "jmp" {
                inst.0 = "nop"
            } else if inst.0 == "nop" {
                inst.0 = "jmp";
            }
        }
        if inst.0 == "acc" || inst.0 == "nop" {
            if inst.0 == "acc" {
                acc += inst.1;
            }
            ip += 1;
        } else if inst.0 == "jmp" {
            ip = (ip as i32 + inst.1) as usize;
        }
        if ip >= v.len() {
            if ip == v.len() {
                finished = true;
            }
            break;
        }
    }
    (finished, acc)
}

fn part1(v: &[(&str, i32)]) -> i32 {
    let (finished, acc) = run(v, usize::MAX);
    assert_eq!(finished, false);
    println!("Part1: {}", acc);
    acc
}

fn part2(v: &[(&str, i32)]) -> i32 {
    let mut result = 0;
    for (swap_index, _) in v.iter().enumerate() {
        if v[swap_index].0 == "jmp" || v[swap_index].0 == "nop" {
            let (finished, acc) = run(v, swap_index);
            if finished {
                result = acc;
                break;
            }
        }
    }
    println!("Part2: {}", result);
    result
}

fn parse(content: &str) -> Vec<(&str, i32)> {
    let v: Vec<(&str, i32)> = content
        .trim()
        .split('\n')
        .map(|x| {
            let v: Vec<&str> = x.trim().split(' ').collect();
            (v[0], v[1].parse().unwrap())
        })
        .collect();
    v
}

fn main() {
    let content = fs::read_to_string("../../inputs/day8_input.txt").expect("Cannot open file!");
    let v = parse(&content);

    let _result_p1 = part1(&v);
    let _result_p2 = part2(&v);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "nop +0
        acc +1
        jmp +4
        acc +3
        jmp -3
        acc -99
        acc +1
        jmp -4
        acc +6";
        let v = parse(input);
        let result = part1(&v);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_part2() {
        let input = "nop +0
        acc +1
        jmp +4
        acc +3
        jmp -3
        acc -99
        acc +1
        jmp -4
        acc +6";
        let v = parse(input);
        let result = part2(&v);
        assert_eq!(result, 8);
    }
}
