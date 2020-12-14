//! Solutions to 2020: Advent of Code day 14
//! By Peter Fornwall

use std::collections::HashMap;
use std::fs;

enum Cmd {
    // mask_x, mask_nr
    Mask(i64, i64),
    // addr, val
    Set(i64, i64),
}

fn part1(v: &[Cmd]) -> i64 {
    let mut m: HashMap<i64, i64> = HashMap::new();
    let mut mask_x = 0;
    let mut mask_nr = 0;
    for cmd in v {
        match cmd {
            Cmd::Mask(new_mask_x, new_mask_nr) => {
                mask_x = *new_mask_x;
                mask_nr = *new_mask_nr;
            }
            Cmd::Set(addr, val) => {
                // mask_x is what bits to keep, mask_nr what bits to set
                let nr = mask_nr + (val & mask_x);
                m.insert(*addr, nr);
            }
        }
    }

    let sum = m.values().sum();
    println!("Part1: {}", sum);
    sum
}

fn recursive_set(m: &mut HashMap<i64, i64>, mask_x: i64, addr: i64, val: i64, bit: i64) {
    if bit == 36 {
        m.insert(addr, val);
        return;
    }
    recursive_set(m, mask_x, addr, val, bit + 1);
    let this_bit = mask_x & (1 << bit);
    if this_bit != 0 {
        recursive_set(m, mask_x, addr + this_bit, val, bit + 1);
    }
}

fn part2(v: &[Cmd]) -> i64 {
    let mut m: HashMap<i64, i64> = HashMap::new();
    let mut mask_x = 0;
    let mut mask_nr = 0;
    for cmd in v {
        match cmd {
            Cmd::Mask(new_mask_x, new_mask_nr) => {
                mask_x = *new_mask_x;
                mask_nr = *new_mask_nr;
            }
            Cmd::Set(addr, val) => {
                let addr_not_floating = (addr | mask_nr) & (!mask_x);
                recursive_set(&mut m, mask_x, addr_not_floating, *val, 0);
            }
        }
    }

    let sum = m.values().sum();
    println!("Part2: {}", sum);
    sum
}

fn parse(content: &str) -> Vec<Cmd> {
    let v: Vec<Cmd> = content
        .trim()
        .split('\n')
        .map(|x| {
            let temp: Vec<&str> = x.split('=').collect();
            let x1 = temp[0].trim();
            let x2 = temp[1].trim();
            if x1.starts_with("mem") {
                let num1: i64 = x1[4..(x1.len() - 1)].parse().unwrap();
                let num2: i64 = x2.parse().unwrap();
                Cmd::Set(num1, num2)
            } else {
                assert!(x1.starts_with("mask"));
                let mut bitty: i64 = 1;
                let mut mask_nr: i64 = 0;
                let mut mask_x: i64 = 0;
                for i in x2.chars().rev() {
                    if i == 'X' {
                        mask_x |= bitty;
                    } else if i == '1' {
                        mask_nr |= bitty;
                    }
                    bitty <<= 1;
                }
                Cmd::Mask(mask_x, mask_nr)
            }
        })
        .collect();
    v
}

fn main() {
    let content = fs::read_to_string("../../inputs/day14_input.txt").expect("Cannot open file!");
    let v = parse(&content);

    let result_p1 = part1(&v);
    assert_eq!(result_p1, 11612740949946);
    let result_p2 = part2(&v);
    assert_eq!(result_p2, 3394509207186);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
        mem[8] = 11
        mem[7] = 101
        mem[8] = 0";
        let v = parse(input);
        let result = part1(&v);
        assert_eq!(result, 165);
    }

    #[test]
    fn test_part2() {
        let input = "mask = 000000000000000000000000000000X1001X
        mem[42] = 100
        mask = 00000000000000000000000000000000X0XX
        mem[26] = 1";
        let v = parse(input);
        let result = part2(&v);
        assert_eq!(result, 208);
    }
}
