//! Solutions to 2020: Advent of Code day 12
//! By Peter Fornwall

use std::fs;

enum Cmd {
    Translate(i32, i32),
    Rotate(i32),
    Forward(i32),
}

fn deg_to_delta(deg: i32) -> (i32, i32) {
    match deg {
        0 => (1, 0),
        90 => (0, 1),
        180 => (-1, 0),
        270 => (0, -1),
        _ => panic!("Unknown rotation!"),
    }
}

fn rotate(x: i32, y: i32, deg: i32) -> (i32, i32) {
    let deg = deg as f32;
    let deg_cos = deg.to_radians().cos();
    let deg_sin = deg.to_radians().sin();
    let x_new = deg_cos * x as f32 - deg_sin * y as f32;
    let y_new = deg_sin * x as f32 + deg_cos * y as f32;
    (x_new.round() as i32, y_new.round() as i32)
}

// Part1, translate and rotate are applied on position
fn part1(v: &[Cmd]) -> usize {
    let (x, y, _) = v.iter().fold((0, 0, 0), |acc, c| match c {
        Cmd::Translate(dx, dy) => (acc.0 + dx, acc.1 + dy, acc.2),
        Cmd::Rotate(deg) => (acc.0, acc.1, (acc.2 + deg) % 360),
        Cmd::Forward(nr) => {
            let delta = deg_to_delta(acc.2);
            (acc.0 + nr * delta.0, acc.1 + nr * delta.1, acc.2)
        }
    });
    let result: usize = (x.abs() + y.abs()) as usize;
    println!("Part1: {}", result);
    result
}

// Part 2, translate and rotate are applied on waypoint
fn part2(v: &[Cmd]) -> usize {
    let (x, y, _, _) = v.iter().fold((0, 0, 10, -1), |acc, c| match c {
        Cmd::Translate(dx, dy) => (acc.0, acc.1, acc.2 + dx, acc.3 + dy),
        Cmd::Rotate(deg) => {
            let (wx_new, wy_new) = rotate(acc.2, acc.3, *deg);
            (acc.0, acc.1, wx_new, wy_new)
        }
        Cmd::Forward(nr) => (acc.0 + nr * acc.2, acc.1 + nr * acc.3, acc.2, acc.3),
    });
    let result: usize = (x.abs() + y.abs()) as usize;
    println!("Part1: {}", result);
    result
}

fn parse(content: &str) -> Vec<Cmd> {
    let v: Vec<Cmd> = content
        .trim()
        .split('\n')
        .map(|x| {
            let ch = x.trim().chars().next().unwrap();
            let nr: i32 = x.trim()[1..].parse().unwrap();
            match ch {
                'N' => Cmd::Translate(0, -nr),
                'E' => Cmd::Translate(nr, 0),
                'S' => Cmd::Translate(0, nr),
                'W' => Cmd::Translate(-nr, 0),
                'R' => Cmd::Rotate(nr),
                'L' => Cmd::Rotate(360 - nr),
                'F' => Cmd::Forward(nr),
                _ => panic!("Unknown command!"),
            }
        })
        .collect();
    v
}

fn main() {
    let content = fs::read_to_string("../../inputs/day12_input.txt").expect("Cannot open file!");
    let v = parse(&content);

    let result_p1 = part1(&v);
    assert_eq!(result_p1, 562);
    let result_p2 = part2(&v);
    assert_eq!(result_p2, 101860);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "F10
        N3
        F7
        R90
        F11";
        let v = parse(input);
        let result = part1(&v);
        assert_eq!(result, 25);
    }

    #[test]
    fn test_part2() {
        let input = "F10
        N3
        F7
        R90
        F11";
        let v = parse(input);
        let result = part2(&v);
        assert_eq!(result, 286);
    }
}
