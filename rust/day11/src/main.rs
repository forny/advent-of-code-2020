//! Solutions to 2020: Advent of Code day 11
//! By Peter Fornwall

use std::fs;

fn count_occupied(
    floor_map: &[Vec<u8>],
    x_check: usize,
    y_check: usize,
    check_line: bool,
) -> usize {
    let width = floor_map[0].len();
    let height = floor_map.len();
    let mut result = 0;
    for delta_y in -1..=1 {
        for delta_x in -1..=1 {
            if delta_x == 0 && delta_y == 0 {
                continue;
            }
            let seen = false;
            let mut cur_x = x_check as i32;
            let mut cur_y = y_check as i32;
            loop {
                cur_x += delta_x;
                cur_y += delta_y;
                if cur_x < 0 || cur_x >= width as i32 || cur_y < 0 || cur_y >= height as i32 {
                    break;
                }
                let item = floor_map[cur_y as usize][cur_x as usize];
                if item == b'#' {
                    result += 1;
                    break;
                } else if !check_line || item == b'L' {
                    break;
                }
            }
            if seen {
                result += 1;
            }
        }
    }
    result
}

fn find_occupied_ferry_seats(
    input_map: &[Vec<u8>],
    check_lines: bool,
    nr_neighbours_to_count: usize,
) -> usize {
    let width = input_map[0].len();
    let height = input_map.len();
    let mut maps: [Vec<Vec<u8>>; 2] = [input_map.to_owned(), input_map.to_owned()];

    let mut iterations = 0;
    loop {
        iterations += 1;
        let mut changed = false;
        let source_index = iterations % 2;
        let dest_index = (iterations + 1) % 2;
        for y in 0..height {
            for x in 0..width {
                let ch = maps[source_index][y][x];
                if ch != b'.' {
                    let nr = count_occupied(&maps[source_index], x, y, check_lines);
                    if ch == b'L' {
                        if nr == 0 {
                            maps[dest_index][y][x] = b'#';
                            changed = true;
                        } else {
                            maps[dest_index][y][x] = b'L';
                        }
                    } else if ch == b'#' {
                        if nr >= nr_neighbours_to_count {
                            maps[dest_index][y][x] = b'L';
                            changed = true;
                        } else {
                            maps[dest_index][y][x] = b'#';
                        }
                    }
                }
            }
        }
        if !changed {
            let result = maps[dest_index]
                .iter()
                .flatten()
                .filter(|x| **x == b'#')
                .count();
            return result;
        }
    }
}

fn part1(input_map: &[Vec<u8>]) -> usize {
    let result = find_occupied_ferry_seats(input_map, false, 4);
    println!("Part1: {}", result);
    result
}

fn part2(input_map: &[Vec<u8>]) -> usize {
    let result = find_occupied_ferry_seats(input_map, true, 5);
    println!("Part2: {}", result);
    result
}

fn parse(content: &str) -> Vec<Vec<u8>> {
    let v: Vec<Vec<u8>> = content
        .trim()
        .split('\n')
        .map(|x| x.trim().as_bytes().iter().copied().collect())
        .collect();
    v
}

fn main() {
    let content = fs::read_to_string("../../inputs/day11_input.txt").expect("Cannot open file!");
    let v = parse(&content);

    let result_p1 = part1(&v);
    assert_eq!(result_p1, 2247);
    let result_p2 = part2(&v);
    assert_eq!(result_p2, 2011);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "L.LL.LL.LL
        LLLLLLL.LL
        L.L.L..L..
        LLLL.LL.LL
        L.LL.LL.LL
        L.LLLLL.LL
        ..L.L.....
        LLLLLLLLLL
        L.LLLLLL.L
        L.LLLLL.LL";
        let v = parse(input);
        let result = part1(&v);
        assert_eq!(result, 37);
    }

    #[test]
    fn test_part2() {
        let input = "L.LL.LL.LL
        LLLLLLL.LL
        L.L.L..L..
        LLLL.LL.LL
        L.LL.LL.LL
        L.LLLLL.LL
        ..L.L.....
        LLLLLLLLLL
        L.LLLLLL.L
        L.LLLLL.LL";
        let v = parse(input);
        let result = part2(&v);
        assert_eq!(result, 26);
    }
}
