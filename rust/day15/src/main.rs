//! Solutions to 2020: Advent of Code day 15
//! By Peter Fornwall

fn get_number(start_seq: &[usize], nr_iterations: usize) -> usize {
    let mut last_seen = Vec::with_capacity(nr_iterations);
    last_seen.resize(nr_iterations, usize::MAX);
    for (index, &value) in start_seq[..(start_seq.len() - 1)].iter().enumerate() {
        last_seen[value] = index;
    }
    let mut last_number = *start_seq.last().unwrap();

    for i in (start_seq.len() - 1)..(nr_iterations - 1) {
        let last_index = last_seen[last_number];
        last_seen[last_number] = i;
        if last_index != usize::MAX {
            last_number = i - last_index;
        } else {
            last_number = 0;
        }
    }
    last_number
}

fn part1(v: &[usize]) -> usize {
    let last_number = get_number(v, 2020);
    println!("Part1: {}", last_number);
    last_number
}

fn part2(v: &[usize]) -> usize {
    let last_number = get_number(v, 30_000_000);
    println!("Part2: {}", last_number);
    last_number
}

fn main() {
    let v = vec![0, 13, 16, 17, 1, 10, 6];

    let result_p1 = part1(&v);
    assert_eq!(result_p1, 276);
    let result_p2 = part2(&v);
    assert_eq!(result_p2, 31916);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let v = vec![1, 3, 2];
        let result = part1(&v);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_part2() {
        let v = vec![0, 3, 6];
        let result = part2(&v);
        assert_eq!(result, 175594);
    }
}
