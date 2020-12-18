//! Solutions to 2020: Advent of Code day 18
//! By Peter Fornwall

use regex::Regex;
use std::fs;

#[derive(Debug, PartialEq, Clone, Copy)]
enum OperationKind {
    Add,
    Mul,
    //None,
}
#[derive(Debug, Copy, Clone)]
enum Node {
    Number(i64),
    Operation(OperationKind),
}

impl Node {
    fn get_expected_number(&self) -> i64 {
        match self {
            Node::Number(nr) => *nr,
            _ => panic!(),
        }
    }

    fn get_expected_op_kind(&self) -> OperationKind {
        match self {
            Node::Operation(op_kind) => *op_kind,
            _ => panic!(),
        }
    }
}

fn calc_expressions_and_sum(
    expressions: &[&str],
    calculate_inside_parenthesis: fn(&[Node]) -> i64,
) -> i64 {
    let mut sum = 0;
    let re = Regex::new(r"\d+|\(|\)|\+|\*").unwrap();
    for &line in expressions {
        let mut input_items: Vec<&str> = re.find_iter(line).map(|x| x.as_str()).collect();
        input_items.insert(0, "(");
        input_items.push(")");

        let mut parenthesis_stack: Vec<Vec<Node>> = Vec::new();
        parenthesis_stack.push(Vec::new());
        for item in input_items {
            if item == "(" {
                parenthesis_stack.push(Vec::new());
            } else if item == ")" {
                let nodes = parenthesis_stack.last().unwrap();
                let result = calculate_inside_parenthesis(nodes);
                parenthesis_stack.pop();
                let parent_nodes = parenthesis_stack.last_mut().unwrap();
                parent_nodes.push(Node::Number(result));
            } else if item == "+" || item == "*" {
                let op_kind = if item == "+" {
                    OperationKind::Add
                } else {
                    OperationKind::Mul
                };
                let nodes = parenthesis_stack.last_mut().unwrap();
                nodes.push(Node::Operation(op_kind));
            } else {
                let nr = item.parse::<i64>().unwrap();
                let nodes = parenthesis_stack.last_mut().unwrap();
                nodes.push(Node::Number(nr));
            }
        }
        sum += parenthesis_stack[0][0].get_expected_number();
    }
    sum
}

// if do_op_kind == None, then perform both addition/multiplication in order.
fn calc_loop(nodes: &mut Vec<Node>, do_op_kind: Option<OperationKind>) {
    let mut i = 0;
    while (i + 2) < nodes.len() {
        let op_kind = nodes[i + 1].get_expected_op_kind();
        if do_op_kind == None || do_op_kind == Some(op_kind) {
            let nr1 = nodes[i].get_expected_number();
            let nr2 = nodes[i + 2].get_expected_number();
            let result: i64;
            if op_kind == OperationKind::Add {
                result = nr1 + nr2;
            } else {
                result = nr1 * nr2;
            }
            nodes[i] = Node::Number(result);
            nodes.remove(i + 1);
            nodes.remove(i + 1);
        } else {
            i += 2;
        }
    }
}

fn part1(expressions: &[&str]) -> i64 {
    fn calc(nodes: &[Node]) -> i64 {
        let mut nodes = nodes.iter().copied().collect();
        // Addition/Multiplication have the same precedence. Do both in order.
        calc_loop(&mut nodes, None);
        nodes[0].get_expected_number()
    }
    calc_expressions_and_sum(expressions, calc)
}

fn part2(expressions: &[&str]) -> i64 {
    fn calc(nodes: &[Node]) -> i64 {
        let mut nodes = nodes.iter().copied().collect();
        // Addition is evaluated before multiplication.
        calc_loop(&mut nodes, Some(OperationKind::Add));
        calc_loop(&mut nodes, Some(OperationKind::Mul));
        nodes[0].get_expected_number()
    }
    calc_expressions_and_sum(expressions, calc)
}

fn parse(content: &str) -> Vec<&str> {
    content.trim().split('\n').map(|x| x.trim()).collect()
}

fn main() {
    let content = fs::read_to_string("../../inputs/day18_input.txt").expect("Cannot open file!");
    let expressions = parse(&content);

    let result_p1 = part1(&expressions);
    println!("Part1: {}", result_p1);
    assert_eq!(result_p1, 464478013511);
    let result_p2 = part2(&expressions);
    println!("Part2: {}", result_p2);
    assert_eq!(result_p2, 85660197232452);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "1 + 2 * 3 + 4 * 5 + 6";
        let v = parse(input);
        let result = part1(&v);
        assert_eq!(result, 71);

        let input = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        let v = parse(input);
        let result = part1(&v);
        assert_eq!(result, 13632);
    }

    #[test]
    fn test_part2() {
        let input = "1 + 2 * 3 + 4 * 5 + 6";
        let v = parse(input);
        let result = part2(&v);
        assert_eq!(result, 231);

        let input = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        let v = parse(input);
        let result = part2(&v);
        assert_eq!(result, 23340);
    }
}
