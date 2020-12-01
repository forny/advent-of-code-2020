//! Solutions to 2020: Advent of Code day 1
//! By Peter Fornwall
//!
//! For day 1 I made two different solutions.

use std::collections::HashSet;
use std::fs;

// Find two entries in values that has the sum search_sum, and return their product.
pub fn find_product(values: &HashSet<i32>, search_sum: i32) -> Option<i32> {
    values
        .iter()
        .filter_map(|x| {
            let second_value = search_sum - *x;
            if values.contains(&second_value) {
                Some(x * second_value)
            } else {
                None
            }
        })
        .next()
}

pub fn part1(values: &HashSet<i32>, search_sum: i32) -> i32 {
    let product = find_product(values, search_sum).unwrap();
    println!("Part1 product: {}", product);
    return product;
}

pub fn part2(values: &HashSet<i32>, search_sum: i32) -> i32 {
    let product = values
        .iter()
        .filter_map(|x| {
            let second_value = search_sum - *x;
            if let Some(partial_product) = find_product(values, second_value) {
                return Some(x * partial_product);
            }
            None
        })
        .next()
        .unwrap();
    println!("Part2 product: {}", product);
    return product;
}

// Simpler (and in my opinion more readable) solution to part 1, but slower.
pub fn part1_simple(v: &Vec<i32>, search_sum: i32) -> i32 {
    for i in 0..(v.len() - 1) {
        for j in (i + 1)..v.len() {
            let x = v[i];
            let y = v[j];
            if x + y == search_sum {
                let product = x * y;
                println!("Part1 simple product: {}", product);
                return product;
            }
        }
    }
    panic!("Could not find sum!");
}

// Simpler (and in my opinion more readable) solution to part 2, but slower.
pub fn part2_simple(v: &Vec<i32>, search_sum: i32) -> i32 {
    for i in 0..(v.len() - 2) {
        for j in (i + 1)..(v.len() - 1) {
            for k in (j + 1)..(v.len()) {
                let x = v[i];
                let y = v[j];
                let z = v[k];
                if x + y + z == search_sum {
                    let product = x * y * z;
                    println!("Part2 simple product: {}", product);
                    return product;
                }
            }
        }
    }
    panic!("Could not find sum!");
}

fn main() {
    let content = fs::read_to_string("..\\..\\inputs\\day1_input.txt").expect("Cannot open file!");
    let h: HashSet<i32> = content.lines().map(|x| x.parse().unwrap()).collect();

    let result_p1 = part1(&h, 2020);
    let result_p2 = part2(&h, 2020);

    let v: Vec<i32> = content.lines().map(|x| x.parse().unwrap()).collect();

    let result_p1_simple = part1_simple(&v, 2020);
    let result_p2_simple = part2_simple(&v, 2020);

    assert_eq!(result_p1, result_p1_simple);
    assert_eq!(result_p2, result_p2_simple);
}
