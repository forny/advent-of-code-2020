//! Solutions to 2020: Advent of Code day 23
//! By Peter Fornwall

fn part(input: &str, size: usize, nr_rounds: usize) -> String {
    let circle_org: Vec<usize> = input.chars().map(|x| x as usize - '0' as usize).collect();

    let mut circle: Vec<(usize, usize)> = vec![(0, 0); size + 1];
    let mut prev_value = if size > input.len() {
        size
    } else {
        *circle_org.last().unwrap()
    };

    for index in 0..size {
        let cur_value = *circle_org.get(index).unwrap_or(&(index + 1));
        let mut next_value = *circle_org.get(index + 1).unwrap_or(&(index + 2));
        if next_value > size {
            next_value = circle_org[0];
        }
        circle[cur_value] = (prev_value, next_value);
        prev_value = cur_value;
    }

    let mut cur_value = circle_org[0];
    for _ in 1..=nr_rounds {
        let val1 = circle[cur_value].1;
        let val2 = circle[val1].1;
        let val3 = circle[val2].1;

        let mut destination_cup = cur_value;
        loop {
            destination_cup -= 1;
            if destination_cup == 0 {
                destination_cup = size;
            }
            if destination_cup != val1 && destination_cup != val2 && destination_cup != val3 {
                break;
            }
        }
        // Remove from current list
        let before_val1 = circle[val1].0;
        let after_val_3 = circle[val3].1;
        circle[before_val1].1 = after_val_3;
        circle[after_val_3].0 = before_val1;
        // Insert after destination cup
        let after_destination = circle[destination_cup].1;
        circle[destination_cup].1 = val1;
        circle[after_destination].0 = val3;
        circle[val1].0 = destination_cup;
        circle[val3].1 = after_destination;

        cur_value = circle[cur_value].1;
    }

    let mut s: String;
    if size == input.len() {
        s = String::new();
        let mut current = 1;
        for _ in 1..size {
            current = circle[current].1;
            s += &current.to_string();
        }
    } else {
        let val1 = circle[1].1;
        let val2 = circle[val1].1;
        let product = val1 * val2;
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
