//! Solutions to 2020: Advent of Code day 17
//! By Peter Fornwall

use std::collections::HashSet;
use std::fs;

fn count_active_neighbours(
    active: &HashSet<[i32; 4]>,
    mut point: [i32; 4],
    dimension: usize,
    non_zero: bool,
) -> usize {
    let mut result = 0;
    let p = point[dimension];
    for delta in -1..=1 {
        point[dimension] = p + delta;
        if dimension == 0 {
            if (non_zero || delta != 0) && active.contains(&point) {
                result += 1;
            }
        } else {
            result +=
                count_active_neighbours(&active, point, dimension - 1, non_zero || (delta != 0));
        }
    }
    result
}

fn run_cycle(
    active: &HashSet<[i32; 4]>,
    new_active: &mut HashSet<[i32; 4]>,
    nr_dimensions: usize,
    mut point: [i32; 4],
    point_minmax: &[(i32, i32); 4],
    dimension: usize,
) {
    let p = point[dimension];
    for delta in point_minmax[dimension].0..=point_minmax[dimension].1 {
        point[dimension] = p + delta;
        if dimension == 0 {
            let count = count_active_neighbours(&active, point, nr_dimensions - 1, false);
            if active.contains(&point) {
                if count == 2 || count == 3 {
                    new_active.insert(point);
                }
            } else if count == 3 {
                new_active.insert(point);
            }
        } else {
            run_cycle(
                active,
                new_active,
                nr_dimensions,
                point,
                point_minmax,
                dimension - 1,
            );
        }
    }
}

fn calc_active(v: &[Vec<bool>], nr_dimensions: usize) -> usize {
    let mut active: HashSet<[i32; 4]> = HashSet::new();
    for (y, line) in v.iter().enumerate() {
        for (x, &b) in line.iter().enumerate() {
            if b {
                active.insert([x as i32, y as i32, 0, 0]);
            }
        }
    }
    let org_height = v.len() as i32;
    let org_width = v[0].len() as i32;

    let mut point_minmax = [(-1, org_width), (-1, org_height), (-1, 1), (-1, 1)];
    for _ in 0..6 {
        let mut new_active: HashSet<[i32; 4]> = HashSet::new();
        run_cycle(
            &active,
            &mut new_active,
            nr_dimensions,
            [0, 0, 0, 0],
            &point_minmax,
            nr_dimensions - 1,
        );
        active = new_active;
        for (index, (min, max)) in &mut point_minmax.iter_mut().enumerate() {
            if index < nr_dimensions {
                *min -= 1;
                *max += 1;
            }
        }
    }
    active.len()
}

fn part1(v: &[Vec<bool>]) -> usize {
    let result = calc_active(v, 3);
    println!("Part1: {}", result);
    result
}

fn part2(v: &[Vec<bool>]) -> usize {
    let result = calc_active(v, 4);
    println!("Part2: {}", result);
    result
}

fn parse(content: &str) -> Vec<Vec<bool>> {
    content
        .trim()
        .split('\n')
        .map(|x| x.trim().chars().map(|y| y == '#').collect())
        .collect()
}

fn main() {
    let content = fs::read_to_string("../../inputs/day17_input.txt").expect("Cannot open file!");
    let v = parse(&content);

    let result_p1 = part1(&v);
    assert_eq!(result_p1, 284);
    let result_p2 = part2(&v);
    assert_eq!(result_p2, 2240);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = ".#.
        ..#
        ###";
        let v = parse(input);
        let result = part1(&v);
        assert_eq!(result, 112);
    }

    #[test]
    fn test_part2() {
        let input = ".#.
        ..#
        ###";
        let v = parse(input);
        let result = part2(&v);
        assert_eq!(result, 848);
    }
}
