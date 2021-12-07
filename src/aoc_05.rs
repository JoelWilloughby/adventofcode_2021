use regex::Regex;
use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

fn read_it(filename: &str) -> Vec<(Point, Point)> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([0-9]+),([0-9]+) -> ([0-9]+),([0-9]+)").unwrap();
    }

    let input = std::fs::read_to_string(filename).unwrap();
    input.lines().map(|line| {
            let caps = RE.captures(line).unwrap();
            (Point{x: caps[1].parse().unwrap(), y: caps[2].parse().unwrap()}, Point{x: caps[3].parse().unwrap(), y: caps[4].parse().unwrap()})
        }).collect()
}

fn do_it(segments: &Vec<(Point, Point)>) -> usize {
    let mut grid : HashMap<Point, usize> = HashMap::new();

    for (p, q) in segments {
        let x_dir = (q.x - p.x).signum();
        let y_dir = (q.y - p.y).signum();
        let d = std::cmp::max((p.x - q.x).abs(), (p.y - q.y).abs());
        for i in 0..=d {
            *grid.entry(Point{x: p.x + i * x_dir, y: p.y + i * y_dir}).or_insert(0) += 1;
        }
    }

    grid.into_iter().filter(|(_, x)| *x > 1).count()
}

fn drive(filename: &str) {
    let segments = read_it(filename).into_iter().filter(|(p, q)| p.x == q.x || p.y == q.y).collect();
    let val = do_it(&segments);

    println!("{}", val);
}

fn drive_2(filename: &str) {
    let segments = read_it(filename);
    let val = do_it(&segments);

    println!("{}", val);
}

#[test]
fn part_0() {
    drive("res/05/sample.txt");
    drive_2("res/05/sample.txt");
}

#[test]
fn part_1() {
    drive("res/05/input.txt");
}

#[test]
fn part_2() {
    drive_2("res/05/input.txt");
}
