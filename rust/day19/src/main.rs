//! Solutions to 2020: Advent of Code day 19
//! By Peter Fornwall

use std::fs;
use std::{collections::HashMap, collections::HashSet};

// Get all end positions of matches for matching rule_nr at specified index in message
fn get_match_ends(
    rules: &HashMap<String, Vec<String>>,
    rule_nr: &str,
    message: &str,
    index: usize,
) -> HashSet<usize> {
    let mut results: HashSet<usize> = HashSet::new();
    if index >= message.len() {
        return results;
    }

    let this_rule = &rules[rule_nr];
    if this_rule[0] == "a" || this_rule[0] == "b" {
        if message[index..(index + 1)] == this_rule[0] {
            results.insert(index + 1);
        }
    } else {
        let mut is_first_in_or = true;
        let mut possible_end_match: HashSet<usize> = HashSet::new();

        for rule_item in this_rule.iter() {
            if rule_item == "|" {
                results.extend(&possible_end_match);
                is_first_in_or = true;
            } else if is_first_in_or {
                possible_end_match = get_match_ends(rules, rule_item, message, index);
                is_first_in_or = false;
            } else {
                let mut second_possible_end_match: HashSet<usize> = HashSet::new();
                for x in possible_end_match {
                    let checked_possible_end_match = get_match_ends(rules, rule_item, message, x);
                    second_possible_end_match.extend(&checked_possible_end_match);
                }
                possible_end_match = second_possible_end_match;
            }
        }
        results.extend(&possible_end_match);
    }
    results
}

fn sum_matches(rules: &HashMap<String, Vec<String>>, messages: &[String]) -> usize {
    let mut sum: usize = 0;
    for message in messages {
        let possible = get_match_ends(rules, "0", message, 0);
        if possible.contains(&message.len()) {
            sum += 1;
        }
    }
    sum
}

fn parse(content: &str, is_part2: bool) -> (HashMap<String, Vec<String>>, Vec<String>) {
    let mut parts_it = content.trim().split("\n\n");
    let rules_str = parts_it.next().unwrap();
    let mut rules_string = rules_str.to_string();
    let messages_str = parts_it.next().unwrap();
    if is_part2 {
        rules_string += "\n8: 42 | 42 8\n11: 42 31 | 42 11 31";
    }

    let mut rules = HashMap::new();
    for rule in rules_string.lines() {
        let mut rule_it = rule.trim().split(": ");
        let rule_nr = rule_it.next().unwrap();
        let rule_def: Vec<String> = rule_it
            .next()
            .unwrap()
            .split(' ')
            .map(|x| x.replace("\"", ""))
            .collect();
        rules.insert(rule_nr.to_string(), rule_def);
    }
    let messages = messages_str
        .split('\n')
        .map(|x| x.trim().to_string())
        .collect();
    (rules, messages)
}

fn main() {
    let content = fs::read_to_string("../../inputs/day19_input.txt").expect("Cannot open file!");

    let (rules, messages) = parse(&content, false);
    let result_p1 = sum_matches(&rules, &messages);
    println!("Part1: {}", result_p1);
    assert_eq!(result_p1, 107);

    let (rules, messages) = parse(&content, true);
    let result_p2 = sum_matches(&rules, &messages);
    println!("Part2: {}", result_p2);
    assert_eq!(result_p2, 321);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"0: 1 2
        1: "a"
        2: 1 3 | 3 1
        3: "b"

aa
bb
aab
aba"#;
        let (rules, messages) = parse(input, false);
        let result = sum_matches(&rules, &messages);
        assert_eq!(result, 2);

        let input = r#"0: 4 1 5
        1: 2 3 | 3 2
        2: 4 4 | 5 5
        3: 4 5 | 5 4
        4: "a"
        5: "b"

aaab
ababbb
aab
aba"#;
        let (rules, messages) = parse(input, false);
        let result = sum_matches(&rules, &messages);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_part2() {
        let input = r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;
        let (rules, messages) = parse(input, true);
        let result = sum_matches(&rules, &messages);
        assert_eq!(result, 12);
    }
}
