//! Solutions to 2020: Advent of Code day 24
//! By Peter Fornwall

use std::collections::HashSet;
use std::fs;

type COORD = i8;
static DIRS: [(&str, (COORD, COORD)); 6] = [
    ("se", (0, 1)),
    ("sw", (-1, 1)),
    ("nw", (0, -1)),
    ("ne", (1, -1)),
    ("e", (1, 0)),
    ("w", (-1, 0)),
];

fn get_map(v: &[&str]) -> HashSet<(COORD, COORD)> {
    let mut m: HashSet<(COORD, COORD)> = HashSet::new();
    for line in v {
        let mut i = 0;
        let mut pos = (0, 0);
        while i < line.len() {
            for dir in DIRS.iter() {
                if line[i..].starts_with(dir.0) {
                    pos = (pos.0 + dir.1 .0, pos.1 + dir.1 .1);
                    i += dir.0.len();
                }
            }
        }
        if m.contains(&pos) {
            m.remove(&pos);
        } else {
            m.insert(pos);
        }
    }
    m
}

fn part1(v: &[&str]) -> usize {
    let m = get_map(v);
    let result = m.len();
    println!("Part1: {}", result);
    result
}

fn count_black_neighbors(m: &HashSet<(COORD, COORD)>, check: &(COORD, COORD)) -> usize {
    DIRS.iter()
        .filter(|(_, dir)| m.contains(&(check.0 + dir.0, check.1 + dir.1)))
        .count()
}

fn part2(v: &[&str]) -> usize {
    let mut m = get_map(v);
    for _ in 0..100 {
        let mut new_m = HashSet::new();
        for check in &m {
            let nr_black_neighbors = count_black_neighbors(&m, check);
            if nr_black_neighbors == 1 || nr_black_neighbors == 2 {
                new_m.insert(*check);
            }
            for (_, dir) in DIRS.iter() {
                let white_check = (check.0 + dir.0, check.1 + dir.1);
                if !m.contains(&white_check) && count_black_neighbors(&m, &white_check) == 2 {
                    new_m.insert(white_check);
                }
            }
        }
        m = new_m;
    }
    let result = m.len();
    println!("Part2: {}", result);
    result
}

fn parse(content: &str) -> Vec<&str> {
    content.trim().split('\n').map(|x| x.trim()).collect()
}

fn main() {
    let content = fs::read_to_string("../../inputs/day24_input.txt").expect("Cannot open file!");
    let v = parse(&content);

    let result_p1 = part1(&v);
    assert_eq!(result_p1, 523);
    let result_p2 = part2(&v);
    assert_eq!(result_p2, 4225);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parts() {
        let input = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";
        let v = parse(input);
        let result = part1(&v);
        assert_eq!(result, 10);

        let result = part2(&v);
        assert_eq!(result, 2208);
    }
}
