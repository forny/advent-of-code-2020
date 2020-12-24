//! Solutions to 2020: Advent of Code day 23
//! By Peter Fornwall

fn part(input: &str, size: usize, nr_rounds: usize) -> String {
    let circle_org: Vec<u32> = input.chars().map(|x| x as u32 - '0' as u32).collect();

    let mut circle: Vec<u32> = vec![0; size + 1];
    for index in 0..size {
        let cur_value = *circle_org.get(index).unwrap_or(&(index as u32 + 1));
        let mut next_value = *circle_org.get(index + 1).unwrap_or(&(index as u32 + 2));
        if next_value > size as u32 {
            next_value = circle_org[0];
        }
        circle[cur_value as usize] = next_value;
    }

    let mut cur_value = circle_org[0];
    for _ in 1..=nr_rounds {
        let val1 = circle[cur_value as usize];
        let val2 = circle[val1 as usize];
        let val3 = circle[val2 as usize];

        let mut destination_cup = cur_value;
        loop {
            destination_cup -= 1;
            if destination_cup == 0 {
                destination_cup = size as u32;
            }
            if destination_cup != val1 && destination_cup != val2 && destination_cup != val3 {
                break;
            }
        }
        // Remove from current list
        circle[cur_value as usize] = circle[val3 as usize];
        // Insert after destination cup
        circle[val3 as usize] = circle[destination_cup as usize];
        circle[destination_cup as usize] = val1;

        cur_value = circle[cur_value as usize];
    }

    let mut s: String;
    if size == input.len() {
        s = String::new();
        let mut current = 1;
        for _ in 1..size {
            current = circle[current as usize];
            s += &current.to_string();
        }
    } else {
        let val1 = circle[1];
        let val2 = circle[val1 as usize];
        let product = val1 as u64 * val2 as u64;
        s = product.to_string();
    }
    s
}

fn main() {
    let input = "952438716";

    let result_p1 = part(input, 9, 100);
    println!("Part1: {}", result_p1);
    assert_eq!(result_p1, "97342568");

    let result_p2 = part(input, 1_000_000, 10_000_000);
    println!("Part2: {}", result_p2);
    assert_eq!(result_p2, "902208073192");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "389125467";
        let result = part(input, 9, 10);
        assert_eq!(result, "92658374");

        let result = part(input, 9, 100);
        assert_eq!(result, "67384529");
    }

    #[test]
    fn test_part2() {
        let input = "389125467";
        let result = part(input, 1_000_000, 10_000_000);
        assert_eq!(result, "149245887792");
    }
}
