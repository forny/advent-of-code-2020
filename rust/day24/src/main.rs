//! Solutions to 2020: Advent of Code day 24
//! By Peter Fornwall

use std::collections::HashSet;
use std::fs;

type COORD = i8;
// map direction to coordinate delta
static DIRS: [(&str, (COORD, COORD)); 6] = [
    ("se", (0, 1)),
    ("sw", (-1, 1)),
    ("nw", (0, -1)),
    ("ne", (1, -1)),
    ("e", (1, 0)),
    ("w", (-1, 0)),
];

fn get_black_tiles(init_flips: &[&str]) -> HashSet<(COORD, COORD)> {
    let mut black_tiles: HashSet<(COORD, COORD)> = HashSet::new();
    for line in init_flips {
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
        if black_tiles.contains(&pos) {
            black_tiles.remove(&pos);
        } else {
            black_tiles.insert(pos);
        }
    }
    black_tiles
}

fn part1(init_flips: &[&str]) -> usize {
    let result = get_black_tiles(init_flips).len();
    println!("Part1: {}", result);
    result
}

fn count_black_neighbors(black_tiles: &HashSet<(COORD, COORD)>, check: &(COORD, COORD)) -> usize {
    DIRS.iter()
        .filter(|(_, dir)| black_tiles.contains(&(check.0 + dir.0, check.1 + dir.1)))
        .count()
}

fn part2(init_flips: &[&str]) -> usize {
    let mut black_tiles = get_black_tiles(init_flips);
    for _ in 0..100 {
        let mut new_black_tiles = HashSet::new();
        for check in &black_tiles {
            let nr_black_neighbors = count_black_neighbors(&black_tiles, check);

            // Check if we should keep this tile black
            if nr_black_neighbors == 1 || nr_black_neighbors == 2 {
                new_black_tiles.insert(*check);
            }

            // For all white neighbors, check if we should flip it to black
            for (_, dir) in DIRS.iter() {
                let white_check = (check.0 + dir.0, check.1 + dir.1);
                if !black_tiles.contains(&white_check)
                    && count_black_neighbors(&black_tiles, &white_check) == 2
                {
                    new_black_tiles.insert(white_check);
                }
            }
        }
        black_tiles = new_black_tiles;
    }
    let result = black_tiles.len();
    println!("Part2: {}", result);
    result
}

fn parse(content: &str) -> Vec<&str> {
    content.trim().split('\n').map(|x| x.trim()).collect()
}

fn main() {
    let content = fs::read_to_string("../../inputs/day24_input.txt").expect("Cannot open file!");
    let init_flips = parse(&content);

    let result_p1 = part1(&init_flips);
    assert_eq!(result_p1, 523);
    let result_p2 = part2(&init_flips);
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
        let init_flips = parse(input);
        let result = part1(&init_flips);
        assert_eq!(result, 10);

        let result = part2(&init_flips);
        assert_eq!(result, 2208);
    }
}
