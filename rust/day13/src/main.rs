//! Solutions to 2020: Advent of Code day 13
//! By Peter Fornwall

use modinverse::modinverse;
use std::fs;

fn part1(timestamp: i64, bus_table: &[i64]) -> i64 {
    let mut min_wait = i64::MAX;
    let mut result = i64::MAX;

    for bus in bus_table {
        if *bus != -1 {
            let mut wait = timestamp % bus;
            if wait != 0 {
                wait = bus - wait;
            }
            if wait < min_wait {
                min_wait = wait;
                result = min_wait * bus;
            }
        }
    }
    println!("Part1: {}", result);
    result
}

// This solution works, since bus numbers are prime.
// see e.g.: // https://en.wikipedia.org/wiki/Modular_multiplicative_inverse
fn part2(bus_table: &[i64]) -> i64 {
    let mut k: i64 = 0;
    let mut m: i64 = 0;
    let mut first_delay = 0;
    for (bus_delay, bus_nr) in bus_table.iter().enumerate() {
        if *bus_nr != -1 {
            if k == 0 && m == 0 {
                // First bus
                // For any positive integer x, bus leaves at time t:
                //    t = k*x + m
                k = *bus_nr;
                m = bus_delay as i64;
                first_delay = m;
            } else {
                // This bus should leave when (for any positive integer t):
                //    t + bus_delay = 0 (mod bus_nr)
                // That gives (with t = k*x + m):
                //    k * x + m + bus_delay = 0 (mod bus_nr)
                let mut kx_equals = (-m - bus_delay as i64) % bus_nr;
                if kx_equals < 0 {
                    kx_equals += bus_nr;
                }
                // x = (1/k)_mod(bus_nr) * (-m - bus_delay)  (mod bus_nr)
                if let Some(inverse) = modinverse(k, *bus_nr) {
                    let this_m = inverse * kx_equals % *bus_nr;
                    let this_k = bus_nr;
                    // For this bus (for any positive integer t):
                    //    x = this_k * t + this_m
                    // Insert into t = kx+m, and we get:
                    //    t = k * this_k + k*this_m + m
                    m = k * this_m + m;
                    k = k * this_k;
                } else {
                    panic!("no inverse, not a prime bus!!! ;)");
                }
            }
        }
    }

    let result: i64 = m + first_delay;
    println!("Part2: {}", result);
    result
}

fn parse(content: &str) -> (i64, Vec<i64>) {
    let mut lines = content.trim().lines();
    let t = lines.next().unwrap().trim().parse().unwrap();
    let v = lines
        .next()
        .unwrap()
        .trim()
        .split(',')
        .map(|x| if let Ok(nr) = x.parse() { nr } else { -1 })
        .collect();
    (t, v)
}

fn main() {
    let content = fs::read_to_string("../../inputs/day13_input.txt").expect("Cannot open file!");
    let v = parse(&content);

    let result_p1 = part1(v.0, &v.1);
    assert_eq!(result_p1, 3966);
    let result_p2 = part2(&v.1);
    assert_eq!(result_p2, 800177252346225);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "939
        7,13,x,x,59,x,31,19";
        let v = parse(input);
        let result = part1(v.0, &v.1);
        assert_eq!(result, 295);
    }

    #[test]
    fn test_part2() {
        let input = "939
        7,13,x,x,59,x,31,19";
        let v = parse(input);
        let result = part2(&v.1);
        assert_eq!(result, 1068781);

        let input = "234
        17,x,13,19";
        let v = parse(input);
        let result = part2(&v.1);
        assert_eq!(result, 3417);

        let input = "234
        67,7,59,61";
        let v = parse(input);
        let result = part2(&v.1);
        assert_eq!(result, 754018);

        let input = "234
        67,x,7,59,61";
        let v = parse(input);
        let result = part2(&v.1);
        assert_eq!(result, 779210);

        let input = "234
        67,7,x,59,61";
        let v = parse(input);
        let result = part2(&v.1);
        assert_eq!(result, 1261476);

        let input = "234
        1789,37,47,1889";
        let v = parse(input);
        let result = part2(&v.1);
        assert_eq!(result, 1202161486);
    }
}
