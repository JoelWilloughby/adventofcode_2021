use std::collections::{HashMap};

fn read_it(filename: &str) -> (String, HashMap<String, char>) {
    let mut char_map = HashMap::new();
    let input = std::fs::read_to_string(filename).unwrap();
    let mut lines = input.lines();
    let start = lines.next().unwrap().to_owned();
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }

        let mut entries = line.split(" -> ");
        let key = entries.next().unwrap().to_owned();
        let val = entries.next().unwrap().chars().next().unwrap();

        char_map.insert(key, val);
    }

    (start, char_map)
}

fn cycle(start: &str, char_map: &HashMap<String, char>) -> String {
    let mut char_list : Vec<char> = vec![];
    let chars : Vec<char> = start.chars().collect();

    char_list.push(chars[0]);
    for i in 1..chars.len() {
        if let Some(c) = char_map.get(&start[(i-1)..=i]) {
            char_list.push(*c);
        }
        char_list.push(chars[i]);
    }

    char_list.into_iter().collect()
}

fn drive(filename: &str) {
    let (mut start, char_map) = read_it(filename);

    for _ in 0..10 {
        start = cycle(&start, &char_map);
    }

    let mut counters = [0usize; 255];
    for c in start.chars() {
        counters[c as usize] += 1;
    }

    let max = counters.iter().fold(0, |acc, x| std::cmp::max(acc, *x));
    let min = counters.iter().fold(std::usize::MAX, |acc, x| if *x > 0 {std::cmp::min(acc, *x)} else {acc});

    println!("{}", max - min);
}

fn do_it(start: &str, n: usize, char_map: &HashMap<String, char>) -> HashMap<char, usize> {
    let mut memo : HashMap<(char, char), usize>  = HashMap::new();
    let mut pair_map : HashMap<(char, char), ((char, char), (char, char))> = HashMap::new();
    let chars : Vec<char> = start.chars().collect();

    for (s, c) in char_map.iter() {
        let mut chars = s.chars();
        let s0 = chars.next().unwrap();
        let s1 = chars.next().unwrap();
        pair_map.insert((s0, s1), ((s0, *c), (*c, s1)));
    }

    let mut ret : HashMap<char, usize> = HashMap::new();
    *ret.entry(chars[0]).or_insert(0) += 1;
    for i in 1..chars.len() {
        *memo.entry((chars[i-1], chars[i])).or_insert(0) += 1;
        *ret.entry(chars[i]).or_insert(0) += 1;
    }

    for _ in 0..n {
        for (p, count) in memo.clone() {
            if let Some((p1, p2)) = pair_map.get(&p) {
                *memo.entry(p).or_insert(0) -= count;
                *memo.entry(*p1).or_insert(0) += count;
                *memo.entry(*p2).or_insert(0) += count;
                *ret.entry(p1.1).or_insert(0) += count;
            }
        }
    }

    ret
}

fn drive_2(filename: &str) {
    let (start, char_map) = read_it(filename);
    let counters = do_it(&start, 40, &char_map);
    let max = counters.values().fold(0, |acc, x| std::cmp::max(acc, *x));
    let min = counters.values().fold(std::usize::MAX, |acc, x| std::cmp::min(acc, *x));

    println!("{}", max - min);
}

#[test]
fn part_0() {
    drive("res/14/sample.txt");
    drive_2("res/14/sample.txt");
}

#[test]
fn part_1() {
    drive("res/14/input.txt");
}

#[test]
fn part_2() {
    drive_2("res/14/input.txt");
}
