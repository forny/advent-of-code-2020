//! Solutions to 2020: Advent of Code day 3
//! By Peter Fornwall

use std::fs;

pub fn find(slope_map: &Vec<Vec<bool>>, delta_x: usize, delta_y: usize) -> i64 {
    let mut x = 0;
    let mut y = 0;
    let height = slope_map.len();
    let width = slope_map[0].len();
    let mut trees = 0;
    while y + delta_y < height {
        x += delta_x;
        x %= width;
        y += delta_y;
        if slope_map[y][x] {
            trees += 1;
        }
    }
    return trees;
}

pub fn part1(slope_map: &Vec<Vec<bool>>) -> i64 {
    let trees: i64 = find(slope_map, 3, 1);
    println!("Part1: {}", trees);
    return trees;
}

pub fn part2(trees: &Vec<Vec<bool>>) -> i64 {
    let result: i64 = find(trees, 1, 1)
        * find(trees, 3, 1)
        * find(trees, 5, 1)
        * find(trees, 7, 1)
        * find(trees, 1, 2);
    println!("Part2: {}", result);
    return result;
}

fn main() {
    let content = fs::read_to_string("../../inputs/day3_input.txt").expect("Cannot open file!");
    let mut v = Vec::<Vec<bool>>::new();
    for line in content.lines() {
        let vec_line: Vec<bool> = line
            .chars()
            .map(|c| if c == '#' { true } else { false })
            .collect();
        v.push(vec_line);
    }

    let _result_p1 = part1(&v);
    let _result_p2 = part2(&v);
}
