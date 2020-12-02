//! Solutions to 2020: Advent of Code day 2
//! By Peter Fornwall

use std::fs;

struct Password {
    nr1: usize,
    nr2: usize,
    ch: char,
    pwd: String,
}

// Parse a line like "1-7 q: qqqqxvqrkbqqztlqlzq"
fn parse(line: &str) -> Password {
    let mut it = line.split(" ");
    let policy: Vec<&str> = it.next().unwrap().split("-").collect();
    let nr1: usize = policy[0].parse().unwrap();
    let nr2: usize = policy[1].parse().unwrap();
    let ch = it.next().unwrap().chars().next().unwrap();
    let pwd = String::from(it.next().unwrap());
    Password { nr1, nr2, ch, pwd }
}

fn part1(v: &Vec<Password>) {
    let nr_passwords = v
        .iter()
        .filter(|password| {
            let count = password.pwd.chars().filter(|x| *x == password.ch).count();
            return count >= password.nr1 && count <= password.nr2;
        })
        .count();

    println!("Part1, nr passwords: {}", nr_passwords);
}

fn part2(v: &Vec<Password>) {
    let nr_passwords = v
        .iter()
        .filter(|password| {
            let ch1 = password.pwd.as_bytes()[password.nr1 - 1] as char;
            let ch2 = password.pwd.as_bytes()[password.nr2 - 1] as char;
            return (ch1 == password.ch) ^ (ch2 == password.ch);
        })
        .count();

    println!("Part2, nr passwords: {}", nr_passwords);
}

fn main() {
    let content = fs::read_to_string("../../inputs/day2_input.txt").expect("Cannot open file!");
    let v: Vec<Password> = content.lines().map(|line| parse(line)).collect();

    part1(&v);
    part2(&v);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse() {
        let input = "1-7 q: qqqqxvqrkbqqztlqlzq";
        let p = crate::parse(input);
        assert_eq!(p.nr1, 1);
        assert_eq!(p.nr2, 7);
        assert_eq!(p.ch, 'q');
        assert_eq!(p.pwd, String::from("qqqqxvqrkbqqztlqlzq"));
    }
}
