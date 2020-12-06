//! Solutions to 2020: Advent of Code day 6
//! By Peter Fornwall

use std::collections::HashSet;
use std::fs;

fn count_yes_answers(
    groups: &[&str],
    group_op: fn(&HashSet<char>, &HashSet<char>) -> HashSet<char>,
    start_set: &HashSet<char>,
) -> usize {
    let mut result = 0;
    for group in groups {
        let answers: Vec<&str> = group.split('\n').collect();
        let mut group_yes: HashSet<char> = start_set.clone();
        for answer in answers {
            let answer_set: HashSet<char> = answer.chars().collect();
            group_yes = group_op(&group_yes, &answer_set);
        }
        result += group_yes.len();
    }
    result
}

fn part1(groups: &[&str]) -> usize {
    fn group_union(s: &HashSet<char>, s2: &HashSet<char>) -> HashSet<char> {
        s.union(&s2).cloned().collect()
    }
    let result = count_yes_answers(groups, group_union, &HashSet::new());
    println!("Part1: {}", result);
    result
}

pub fn part2(groups: &[&str]) -> usize {
    let alphabet_set: HashSet<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    fn group_intersection(s: &HashSet<char>, s2: &HashSet<char>) -> HashSet<char> {
        s.intersection(&s2).cloned().collect()
    }
    let result = count_yes_answers(groups, group_intersection, &alphabet_set);
    println!("Part2: {}", result);
    result
}

fn main() {
    let content = fs::read_to_string("../../inputs/day6_input.txt").expect("Cannot open file!");
    let v: Vec<&str> = content.trim().split("\n\n").collect();

    let _result_p1 = part1(&v);
    let _result_p2 = part2(&v);
}
