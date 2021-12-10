use std::collections::BTreeMap;
use lazy_static::lazy_static;

lazy_static!{
    static ref PAIR_MAP: BTreeMap<char, char> = [('}', '{'), (']', '['), (')', '('), ('>', '<')].into_iter().collect();
    static ref SCORE_MAP: BTreeMap<char, usize> = [('}', 1197), (']', 57), (')', 3), ('>', 25137)].into_iter().collect();
    static ref SCALAR_MAP: BTreeMap<char, usize> = [('{', 3), ('[', 2), ('(', 1), ('<', 4)].into_iter().collect();
}

fn eval(line: &str) -> Result<Vec<char>, char> {
    let mut stack = vec![];
    for c in line.chars() {
        match c {
            '{' | '(' | '[' | '<' => {
                stack.push(c);
            },
            '}' | ')' | ']' | '>' => {
                if PAIR_MAP[&c] != *stack.last().unwrap_or(&'x') {
                    return Err(c);
                } else {
                    stack.pop();
                }
            },
            _ => {}
        }
    }

    Ok(stack)
}

fn drive(filename: &str) {
    let input = std::fs::read_to_string(filename).unwrap();
    let val: usize = input.lines()
        .filter_map(|line| eval(line).err())
        .map(|x| SCORE_MAP[&x])
        .sum();

    println!("{}", val);
}

fn drive_2(filename: &str) {
    let input = std::fs::read_to_string(filename).unwrap();
    let mut stuff: Vec<usize> = input.lines()
        .filter_map(|line| eval(line).ok())
        .map(|remaining| remaining.iter().rev().fold(0, |acc,  x| 5 * acc + SCALAR_MAP[x]))
        .collect();

    stuff.sort();
    println!("{}", stuff[stuff.len()/2]);
}

#[test]
fn part_0() {
    drive("res/10/sample.txt");
    drive_2("res/10/sample.txt");
}

#[test]
fn part_1() {
    drive("res/10/input.txt");
}

#[test]
fn part_2() {
    drive_2("res/10/input.txt");
}
