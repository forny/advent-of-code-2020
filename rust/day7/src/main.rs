//! Solutions to 2020: Advent of Code day 7
//! By Peter Fornwall

use std::collections::HashMap;
use std::fs;

fn search_gold(m: &HashMap<&str, Vec<(i32, &str)>>, s: &str) -> bool {
    if s == "shiny gold" {
        return true;
    }
    if let Some(inside) = m.get(s) {
        for (_, x) in inside {
            if search_gold(m, *x) {
                return true;
            }
        }
    }
    false
}

fn part1(m: &HashMap<&str, Vec<(i32, &str)>>) -> usize {
    let result = m
        .iter()
        .filter(|(_, inside)| inside.iter().any(|(_, bag)| search_gold(m, bag)))
        .count();
    println!("Part1: {}", result);
    result
}

fn count_bags(m: &HashMap<&str, Vec<(i32, &str)>>, bag: &str) -> i32 {
    m[bag].iter().fold(0, |acc, (nr, bag_inside)| {
        acc + nr * count_bags(m, bag_inside)
    }) + 1
}

fn part2(m: &HashMap<&str, Vec<(i32, &str)>>) -> i32 {
    let result = count_bags(m, "shiny gold") - 1;
    println!("Part2: {}", result);
    result
}

fn parse(content: &str) -> HashMap<&str, Vec<(i32, &str)>> {
    let mut m: HashMap<&str, Vec<(i32, &str)>> = HashMap::new();
    for i in content.trim().split('\n') {
        let mut it = i.split("bags contain");
        let bag_container = it.next().unwrap().trim();
        let rest = it.next().unwrap().trim();
        let items: Vec<(i32, &str)> = rest
            .split(',')
            .filter_map(|x| {
                if x.starts_with("no other") {
                    return None;
                };
                let x = x.trim();
                let space_index = x.find(' ').unwrap();
                let nr: i32 = x[0..space_index].parse().unwrap();
                let bag = &x[(space_index + 1)..].split(" bag").next().unwrap();
                Some((nr, *bag))
            })
            .collect();
        m.insert(bag_container, items);
    }
    m
}

fn main() {
    let content = fs::read_to_string("../../inputs/day7_input.txt").expect("Cannot open file!");
    let m = parse(&content);

    let _result_p1 = part1(&m);
    let _result_p2 = part2(&m);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
        dark orange bags contain 3 bright white bags, 4 muted yellow bags.
        bright white bags contain 1 shiny gold bag.
        muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
        shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
        dark olive bags contain 3 faded blue bags, 4 dotted black bags.
        vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
        faded blue bags contain no other bags.
        dotted black bags contain no other bags.
        ";
        let m = parse(input);
        let result = part1(&m);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_part2() {
        let input = "shiny gold bags contain 2 dark red bags.
        dark red bags contain 2 dark orange bags.
        dark orange bags contain 2 dark yellow bags.
        dark yellow bags contain 2 dark green bags.
        dark green bags contain 2 dark blue bags.
        dark blue bags contain 2 dark violet bags.
        dark violet bags contain no other bags.
        ";
        let m = parse(input);
        let result = part2(&m);
        assert_eq!(result, 126);
    }
}
