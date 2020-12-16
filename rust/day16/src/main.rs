//! Solutions to 2020: Advent of Code day 16
//! By Peter Fornwall

use std::collections::HashSet;
use std::fs;

struct Range {
    min: i32,
    max: i32,
}

struct Field {
    name: String,
    ranges: Vec<Range>,
}

struct Data {
    fields: Vec<Field>,
    my_ticket: Vec<i32>,
    nearby_tickets: Vec<Vec<i32>>,
}

fn is_value_in_field_ranges(field: &Field, value: i32) -> bool {
    for range in &field.ranges {
        if value >= range.min && value <= range.max {
            return true;
        }
    }
    false
}

fn is_value_in_any_valid_ranges(data: &Data, value: i32) -> bool {
    for field in &data.fields {
        if is_value_in_field_ranges(field, value) {
            return true;
        }
    }
    false
}

fn get_valid_tickets_and_checksum(data: &Data) -> (Vec<Vec<i32>>, i32) {
    let mut valid_nearby_tickets: Vec<Vec<i32>> = Vec::new();
    let mut checksum = 0;

    for nearby in &data.nearby_tickets {
        let mut all_valid = true;
        for &nearby_value in nearby {
            if !is_value_in_any_valid_ranges(&data, nearby_value) {
                all_valid = false;
                checksum += nearby_value;
                continue;
            }
        }
        if all_valid {
            valid_nearby_tickets.push(nearby.clone());
        }
    }
    (valid_nearby_tickets, checksum)
}

fn part1(data: &Data) -> i32 {
    let (_, checksum) = get_valid_tickets_and_checksum(&data);
    println!("Part1: {}", checksum);
    checksum
}

fn part2(data: &Data) -> (i64, Vec<String>) {
    let (valid_nearby_tickets, _) = get_valid_tickets_and_checksum(&data);

    let mut ticket_product = 1i64;
    let mut return_fields = Vec::new();
    let mut seen_field_ids = HashSet::new();
    let nr_fields = data.fields.len();
    while seen_field_ids.len() != nr_fields {
        for ticket_field_index in 0..nr_fields {
            let mut fields_ok_count = 0;
            let mut could_be_field_index = 0;
            for check_field_index in 0..nr_fields {
                if seen_field_ids.contains(&check_field_index) {
                    continue;
                }
                let all_valid = valid_nearby_tickets.iter().all(|tickets| {
                    is_value_in_field_ranges(
                        &data.fields[check_field_index],
                        tickets[ticket_field_index],
                    )
                });
                if all_valid {
                    fields_ok_count += 1;
                    if fields_ok_count > 1 {
                        break;
                    }
                    could_be_field_index = check_field_index;
                }
            }
            if fields_ok_count == 1 {
                let field_name = &data.fields[could_be_field_index].name;
                if field_name.starts_with("departure") {
                    let value = data.my_ticket[ticket_field_index] as i64;
                    ticket_product *= value;
                }
                return_fields.push(field_name.clone());
                seen_field_ids.insert(could_be_field_index);
            }
        }
    }
    println!("Part2: {}", ticket_product);
    (ticket_product, return_fields)
}

fn parse(content: &str) -> Data {
    let sections: Vec<&str> = content.trim().split("\n\n").collect();
    let mut fields: Vec<Field> = Vec::new();
    for line in sections[0].trim().split('\n') {
        // departure location: 32-69 or 86-968
        let parts: Vec<&str> = line.trim().split(':').collect();
        let name = String::from(parts[0].trim());
        let ranges_str: Vec<&str> = parts[1].trim().split("or").collect();
        let mut ranges: Vec<Range> = Vec::new();
        for range_str in ranges_str {
            let range_vals: Vec<i32> = range_str
                .trim()
                .split('-')
                .map(|x| x.parse().unwrap())
                .collect();
            let range = Range {
                min: range_vals[0],
                max: range_vals[1],
            };
            ranges.push(range);
        }
        let field = Field { name, ranges };
        fields.push(field);
    }

    fn parse_ticket(s: &str) -> Vec<i32> {
        s.trim().split(',').map(|x| x.parse().unwrap()).collect()
    }
    let my_ticket: Vec<i32> = parse_ticket(sections[1].trim().split('\n').nth(1).unwrap());
    let nearby_tickets: Vec<Vec<i32>> = sections[2]
        .trim()
        .split('\n')
        .skip(1)
        .map(|x| parse_ticket(x))
        .collect();

    Data {
        fields,
        my_ticket,
        nearby_tickets,
    }
}

fn main() {
    let content = fs::read_to_string("../../inputs/day16_input.txt").expect("Cannot open file!");
    let data: Data = parse(&content);

    let result_p1 = part1(&data);
    assert_eq!(result_p1, 23036);

    let (result_p2, _) = part2(&data);
    assert_eq!(result_p2, 1909224687553);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "class: 1-3 or 5-7
        row: 6-11 or 33-44
        seat: 13-40 or 45-50

        your ticket:
        7,1,14

        nearby tickets:
        7,3,47
        40,4,50
        55,2,20
        38,6,12";
        let data = parse(&input);
        let result = part1(&data);
        assert_eq!(result, 71);
    }

    #[test]
    fn test_part2() {
        let input = "class: 0-1 or 4-19
        row: 0-5 or 8-19
        seat: 0-13 or 16-19

        your ticket:
        11,12,13

        nearby tickets:
        3,9,18
        15,1,5
        5,14,9";
        let data = parse(&input);
        let (result, fields) = part2(&data);
        assert_eq!(result, 1);
        assert_eq!(
            fields,
            vec!["row".to_string(), "class".to_string(), "seat".to_string()]
        );
    }
}
