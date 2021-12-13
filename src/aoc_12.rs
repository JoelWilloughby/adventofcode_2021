use std::collections::{HashMap, BTreeSet};

fn read_it(filename: &str) -> HashMap<String, BTreeSet<String>> {
    let input = std::fs::read_to_string(filename).unwrap();
    let mut ret = HashMap::new();
    for line in input.lines() {
        let items : Vec<_> = line.split("-").collect();
        let a = items[0];
        let b = items[1];

        ret.entry(a.to_owned()).or_insert(BTreeSet::new()).insert(b.to_owned());
        ret.entry(b.to_owned()).or_insert(BTreeSet::new()).insert(a.to_owned());
    }

    ret
}

fn s<'a>(conns: &'a HashMap<String, BTreeSet<String>>, path: &mut BTreeSet<&'a str>, next: &'a str) -> usize {
    if next == "end" {
        // println!("found: {:?}", path);
        return 1;
    }

    if next.to_lowercase() == next {
        if path.contains(next) {
            return 0;
        }

        path.insert(next);
    }

    let mut acc = 0;
    for neighbor in conns[next].iter() {
        acc += s(conns, path, &neighbor);
    }

    path.remove(next);

    acc
}

fn t<'a>(conns: &'a HashMap<String, BTreeSet<String>>, path: &mut BTreeSet<&'a str>, extra: &mut Option<&'a str>, next: &'a str) -> usize {
    if next == "end" {
        // println!("found: {:?}", path);
        return 1;
    }

    let mut used_extra = false;
    if next.to_lowercase() == next {
        if path.contains(next) {
            if next == "start" {
                return 0;
            }
            if let Some(_) = extra {
                return 0;
            }

            extra.replace(next);
            used_extra = true;
        } else {
            path.insert(next);
        }
    }

    let mut acc = 0;
    for neighbor in conns[next].iter() {
        acc += t(conns, path, extra, &neighbor);
    }

    if used_extra {
        extra.take();
    } else {
        path.remove(next);
    }

    acc
}

fn search_it(conns: &HashMap<String, BTreeSet<String>>) -> usize {
    let mut path = BTreeSet::new();
    s(conns, &mut path, "start")
}

fn search_it_2(conns: &HashMap<String, BTreeSet<String>>) -> usize {
    let mut path = BTreeSet::new();
    t(conns, &mut path, &mut None, "start")
}

fn drive(filename: &str) {
    let graph = read_it(filename);
    let val = search_it(&graph);

    println!("{}", val);
}

fn drive_2(filename: &str) {
    let graph = read_it(filename);
    let val = search_it_2(&graph);

    println!("{}", val);
}

#[test]
fn part_0() {
    drive("res/12/sample.txt");
    drive_2("res/12/sample.txt");
}

#[test]
fn part_1() {
    drive("res/12/input.txt");
}

#[test]
fn part_2() {
    drive_2("res/12/input.txt");
}
