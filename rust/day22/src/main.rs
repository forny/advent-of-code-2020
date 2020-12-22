//! Solutions to 2020: Advent of Code day 22
//! By Peter Fornwall

use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;

fn get_score(cards: &[VecDeque<usize>], p1_wins: bool) -> usize {
    let winner_cards = if p1_wins { &cards[0] } else { &cards[1] };
    let mut score = 0;
    for i in 0..winner_cards.len() {
        score += winner_cards[winner_cards.len() - 1 - i] * (i + 1);
    }
    score
}

fn rec_play(v: &[VecDeque<usize>], nr1: usize, nr2: usize, do_recurse: bool) -> (usize, bool) {
    let mut cards: Vec<VecDeque<usize>> = Vec::new();
    cards.push(v[0].iter().take(nr1).copied().clone().collect());
    cards.push(v[1].iter().take(nr2).copied().clone().collect());

    let mut seen = HashSet::new();
    while !cards[0].is_empty() && !cards[1].is_empty() {
        if seen.contains(&cards) {
            return (get_score(&cards, true), true);
        }
        seen.insert(cards.clone());

        let c1 = cards[0].pop_front().unwrap();
        let c2 = cards[1].pop_front().unwrap();
        let p1_wins: bool;
        if do_recurse && c1 <= cards[0].len() && c2 <= cards[1].len() {
            p1_wins = rec_play(&cards, c1, c2, do_recurse).1;
        } else {
            p1_wins = c1 > c2;
        }
        if p1_wins {
            cards[0].push_back(c1);
            cards[0].push_back(c2);
        } else {
            cards[1].push_back(c2);
            cards[1].push_back(c1);
        };
    }

    let p1_wins = cards[1].is_empty();
    (get_score(&cards, p1_wins), p1_wins)
}

fn part1(v: &[VecDeque<usize>]) -> usize {
    let (score, _) = rec_play(v, v[0].len(), v[1].len(), false);
    println!("Part1: {}", score);
    score
}

fn part2(v: &[VecDeque<usize>]) -> usize {
    let (score, _) = rec_play(v, v[0].len(), v[1].len(), true);
    println!("Part2: {}", score);
    score
}

fn parse(content: &str) -> Vec<VecDeque<usize>> {
    content
        .split("\n\n")
        .map(|x| {
            x.trim()
                .split('\n')
                .skip(1)
                .map(|x| x.trim().parse().unwrap())
                .collect()
        })
        .collect()
}

fn main() {
    let content = fs::read_to_string("../../inputs/day22_input.txt").expect("Cannot open file!");
    let v = parse(&content);

    let result_p1 = part1(&v);
    assert_eq!(result_p1, 32102);
    let result_p2 = part2(&v);
    assert_eq!(result_p2, 34173);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";
        let v = parse(input);
        let result = part1(&v);
        assert_eq!(result, 306);
    }

    #[test]
    fn test_part2() {
        let input = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";
        let v = parse(input);
        let result = part2(&v);
        assert_eq!(result, 291);
    }
}
