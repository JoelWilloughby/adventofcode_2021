use std::collections::BTreeSet;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone, Copy)]
enum Fold {
    X(usize),
    Y(usize)
}

fn read_it(filename: &str) -> (BTreeSet<(usize, usize)>, Vec<Fold>) {
    lazy_static!{
        static ref P_RE: Regex = Regex::new(r"([0-9]+),([0-9]+)").unwrap();
        static ref X_RE: Regex = Regex::new(r"fold along x=([0-9]+)").unwrap();
        static ref Y_RE: Regex = Regex::new(r"fold along y=([0-9]+)").unwrap();
    }
    let input = std::fs::read_to_string(filename).unwrap();
    let mut ret = BTreeSet::new();
    let mut folds = Vec::new();
    for line in input.lines() {
        if let Some(caps) = P_RE.captures(line) {
            let x = caps[1].parse().unwrap();
            let y = caps[2].parse().unwrap();
            ret.insert((x, y));
        } else if let Some(caps) = X_RE.captures(line) {
            let x = caps[1].parse().unwrap();
            folds.push(Fold::X(x));
        } else if let Some(caps) = Y_RE.captures(line) {
            let y = caps[1].parse().unwrap();
            folds.push(Fold::Y(y));
        }
    }

    (ret, folds)
}

fn fold(paper: &BTreeSet<(usize, usize)>, f: Fold) -> BTreeSet<(usize, usize)> {
    paper.iter().map(|(x, y)| {
        (
            if let Fold::X(fold_x) = f {
                if *x > fold_x {
                    fold_x - (*x - fold_x)
                } else {
                    *x
                }
            } else {
                *x
            },
            if let Fold::Y(fold_y) = f {
                if *y > fold_y {
                    fold_y - (*y - fold_y)
                } else {
                    *y
                }
            } else {
                *y
            }
        )
    }).collect()
}


fn print(paper: &BTreeSet<(usize, usize)>) {
    let min_x = paper.iter().fold(std::usize::MAX, |m, (x, _)| std::cmp::min(*x, m));
    let min_y = paper.iter().fold(std::usize::MAX, |m, (_, y)| std::cmp::min(*y, m));
    let max_x = paper.iter().fold(0, |m, (x, _)| std::cmp::max(*x, m));
    let max_y = paper.iter().fold(0, |m, (_, y)| std::cmp::max(*y, m));

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if paper.contains(&(x, y)) {
                print!("$");
            } else {
                print!(" ");
            }
        }

        println!("");
    }
}

fn drive(filename: &str) {
    let (mut paper, folds) = read_it(filename);
    paper = fold(&paper, folds[0]);
    println!("{}", paper.len());
}

fn drive_2(filename: &str) {
    let (mut paper, folds) = read_it(filename);
    for f in folds {
        paper = fold(&paper, f);
    }
    print(&paper);
}

#[test]
fn part_0() {
    drive("res/13/sample.txt");
    drive_2("res/13/sample.txt");
}

#[test]
fn part_1() {
    drive("res/13/input.txt");
}

#[test]
fn part_2() {
    drive_2("res/13/input.txt");
}
