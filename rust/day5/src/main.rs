//! Solutions to 2020: Advent of Code day 5
//! By Peter Fornwall

use std::collections::HashSet;
use std::fs;

fn bin_search(s: &[u8], upper_ch: char) -> usize {
    s.iter()
        .fold((0, 2usize.pow((s.len() - 1) as u32)), |acc, c| {
            if *c == upper_ch as u8 {
                return (acc.0 + acc.1, acc.1 / 2);
            }
            (acc.0, acc.1 / 2)
        })
        .0
}

fn get_id(x: &str) -> usize {
    let row = bin_search(&x.as_bytes()[0..7], 'B');
    let column = bin_search(&x.as_bytes()[7..10], 'R');
    let id = row * 8 + column;
    return id;
}

fn part1(v: &Vec<&str>) -> usize {
    let max_id = v.iter().map(|x| get_id(x)).max().unwrap();
    println!("Part1: {}", max_id);
    return max_id;
}

pub fn part2(v: &Vec<&str>) -> usize {
    let s: HashSet<usize> = v.iter().map(|x| get_id(x)).collect();
    let max_id = *s.iter().max().unwrap();
    let my_id = (1..max_id)
        .filter(|x| !s.contains(&x) && s.contains(&(*x - 1)) && s.contains(&(*x + 1)))
        .next()
        .unwrap();
    println!("Part2: {}", my_id);
    return my_id;
}

fn main() {
    let content = fs::read_to_string("../../inputs/day5_input.txt").expect("Cannot open file!");
    let v: Vec<&str> = content.trim().split("\n").collect();

    let _result_p1 = part1(&v);
    let _result_p2 = part2(&v);
}
