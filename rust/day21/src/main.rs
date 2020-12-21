//! Solutions to 2020: Advent of Code day 21
//! By Peter Fornwall

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn parts(foods: &[(HashSet<&str>, HashSet<&str>)]) -> (usize, String) {
    let mut m: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut all_allergens: HashSet<&str> = HashSet::new();
    foods.iter().for_each(|x| {
        x.0.iter().for_each(|i| {
            m.insert(*i, HashSet::new());
        });
        all_allergens.extend(x.1.iter());
    });
    let all_ingredients: Vec<&str> = m.keys().copied().clone().collect();
    // Map ingredient to set of allergens it cannot contain
    for food in foods {
        for one_ing in &all_ingredients {
            if !food.0.contains(one_ing) {
                m.get_mut(one_ing).unwrap().extend(food.1.clone());
            }
        }
    }

    let safe_ingredients: HashSet<_> = all_ingredients
        .iter()
        .filter(|&ingredient| m[ingredient].len() == all_allergens.len())
        .collect();
    let mut count_safe_used = 0;
    for (ings, _) in foods {
        for &clean in &safe_ingredients {
            if ings.contains(clean) {
                count_safe_used += 1;
            }
        }
    }
    println!("Part 1: {}", count_safe_used);

    let mut ing_to_aller_vec: Vec<(&str, &str)> = Vec::new();
    loop {
        let mut did_find = false;
        for ing in &all_ingredients {
            if m[ing].len() == (all_allergens.len() - 1) {
                // Cannot contain all but one. Then has that one.
                for test_aller in &all_allergens {
                    if !m[ing].contains(test_aller) {
                        // ing has test_aller
                        did_find = true;
                        ing_to_aller_vec.push((ing, test_aller));
                        for mark in &all_ingredients {
                            m.get_mut(mark).unwrap().insert(test_aller);
                        }
                    }
                }
            }
        }
        if !did_find {
            break;
        }
    }
    ing_to_aller_vec.sort_by(|a, b| a.1.cmp(b.1));
    let sorted_ings: Vec<&str> = ing_to_aller_vec.iter().map(|x| x.0).collect();
    let result_p2 = sorted_ings.join(",");

    println!("p2: {}", result_p2);
    (count_safe_used, result_p2)
}

fn parse(content: &str) -> Vec<(HashSet<&str>, HashSet<&str>)> {
    let mut foods: Vec<(HashSet<&str>, HashSet<&str>)> = Vec::new();
    for line in content.lines() {
        let mut parts_it = line.split(" (contains ");
        let ingredients: HashSet<&str> = parts_it.next().unwrap().split(' ').collect();
        let allergens: HashSet<&str> = parts_it
            .next()
            .unwrap()
            .trim_end_matches(')')
            .split(", ")
            .collect();
        foods.push((ingredients, allergens));
    }
    foods
}

fn main() {
    let content = fs::read_to_string("../../inputs/day21_input.txt").expect("Cannot open file!");
    let v = parse(&content);

    let (result_p1, result_p2) = parts(&v);
    assert_eq!(result_p1, 2734);
    assert_eq!(result_p2, "kbmlt,mrccxm,lpzgzmk,ppj,stj,jvgnc,gxnr,plrlg");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";
        let v = parse(input);
        let (result_p1, result_p2) = parts(&v);
        assert_eq!(result_p1, 5);
        assert_eq!(result_p2, "mxmxvkd,sqjhc,fvjkl");
    }
}
