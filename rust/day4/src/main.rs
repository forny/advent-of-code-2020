//! Solutions to 2020: Advent of Code day 4
//! By Peter Fornwall

use std::collections::HashMap;
use std::fs;

#[derive(PartialEq)]
enum HeightUnit {
    Cm,
    Inch,
}

#[derive(PartialEq, Debug)]
enum EyeColor {
    Amb,
    Blu,
    Brn,
    Gry,
    Grn,
    Hzl,
    Oth,
}

struct Passport {
    byr: i32, // (Birth Year)
    iyr: i32, // (Issue Year)
    eyr: i32, // (Expiration Year)
    hgt: i32, // (Height)
    hgt_unit: HeightUnit,
    hcl: String,   // (Hair Color)
    ecl: EyeColor, // (Eye Color)
    pid: String,   // (Passport ID)
    cid: String,   // Optional cid (Country ID)
}

fn parse_nr_range(key: &str, min: i32, max: i32) -> Result<i32, ()> {
    let value = key.parse::<i32>().map_err(|_x| ())?;
    if value >= min && value <= max {
        return Ok(value);
    }
    return Err(());
}

fn parse_map_value_nr_range(
    h: &HashMap<&str, &str>,
    key: &str,
    min: i32,
    max: i32,
) -> Result<i32, ()> {
    if h.contains_key(key) {
        return parse_nr_range(h[key], min, max);
    }
    return Err(());
}

fn parse_height(h: &HashMap<&str, &str>) -> Result<(i32, HeightUnit), ()> {
    if let Some(str_value) = h.get("hgt") {
        let len = str_value.len();
        if len > 2 {
            let hgt_value_str = &str_value[0..(len - 2)];
            match &str_value[(len - 2)..] {
                "cm" => {
                    let hgt = parse_nr_range(hgt_value_str, 150, 193)?;
                    return Ok((hgt, HeightUnit::Cm));
                }
                "in" => {
                    let hgt = parse_nr_range(hgt_value_str, 59, 76)?;
                    return Ok((hgt, HeightUnit::Inch));
                }
                _ => {
                    return Err(());
                }
            };
        }
    }
    return Err(());
}

fn parse_hair_color(h: &HashMap<&str, &str>) -> Result<String, ()> {
    if let Some(hcl_str) = h.get("hcl") {
        if hcl_str.len() == 7 && hcl_str.starts_with("#") {
            let hcl_nr_str = &hcl_str[1..];
            if hcl_nr_str
                .chars()
                .filter(|x| (*x >= '0' && *x <= '9') || (*x >= 'a' && *x <= 'f'))
                .count()
                == 6
            {
                return Ok(String::from(hcl_nr_str));
            }
        }
    }
    return Err(());
}

fn parse_eye_color(h: &HashMap<&str, &str>) -> Result<EyeColor, ()> {
    if let Some(ecl_str) = h.get("ecl") {
        return match *ecl_str {
            "amb" => Some(EyeColor::Amb),
            "blu" => Some(EyeColor::Blu),
            "brn" => Some(EyeColor::Brn),
            "gry" => Some(EyeColor::Gry),
            "grn" => Some(EyeColor::Grn),
            "hzl" => Some(EyeColor::Hzl),
            "oth" => Some(EyeColor::Oth),
            _ => None,
        }
        .ok_or(());
    }
    return Err(());
}

fn parse_pid(h: &HashMap<&str, &str>) -> Result<String, ()> {
    if let Some(pid_str) = h.get("pid") {
        if pid_str.len() == 9 {
            if pid_str.chars().all(|x| x.is_ascii_digit()) {
                return Ok(String::from(*pid_str));
            }
        }
    }
    return Err(());
}

fn parse_passport(h: &HashMap<&str, &str>) -> Result<Passport, ()> {
    let byr: i32 = parse_map_value_nr_range(h, "byr", 1920, 2002)?;
    let iyr: i32 = parse_map_value_nr_range(h, "iyr", 2010, 2020)?;
    let eyr: i32 = parse_map_value_nr_range(h, "eyr", 2020, 2030)?;
    let (hgt, hgt_unit) = parse_height(h)?;
    let hcl: String = parse_hair_color(h)?;
    let ecl: EyeColor = parse_eye_color(h)?;
    let pid: String = parse_pid(h)?;
    let cid = String::from(*h.get("cid").unwrap_or(&""));

    let passport = Passport {
        byr,
        iyr,
        eyr,
        hgt,
        hgt_unit,
        hcl,
        ecl,
        pid,
        cid,
    };
    return Ok(passport);
}

fn create_passport_map(s: &str) -> HashMap<&str, &str> {
    return s
        .split(|c| c == ' ' || c == '\n')
        .map(|x| {
            let key_value: Vec<&str> = x.split(":").collect();
            return (key_value[0], key_value[1]);
        })
        .collect();
}

impl Passport {
    fn has_fields(s: &str) -> bool {
        let h: HashMap<&str, &str> = create_passport_map(s);
        let v = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        return v.iter().all(|x| h.contains_key(x));
    }

    fn parse(s: &str) -> Result<Passport, ()> {
        let h: HashMap<&str, &str> = create_passport_map(s);
        return parse_passport(&h);
    }
}

pub fn part1(v: &Vec<&str>) -> usize {
    let nr = v
        .iter()
        .map(|x| Passport::has_fields(x))
        .filter(|x| *x == true)
        .count();

    println!("Part1: {}", nr);
    return nr;
}

pub fn part2(v: &Vec<&str>) -> usize {
    let nr = v
        .iter()
        .map(|x| Passport::parse(x))
        .filter_map(Result::ok)
        .count();

    println!("Part2: {}", nr);
    return nr;
}

fn main() {
    let content = fs::read_to_string("../../inputs/day4_input.txt").expect("Cannot open file!");
    let v: Vec<&str> = content.trim().split("\n\n").collect();

    let _result_p1 = part1(&v);
    let _result_p2 = part2(&v);
}
