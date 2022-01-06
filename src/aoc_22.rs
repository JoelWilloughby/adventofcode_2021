use std::collections::LinkedList;

use regex::Regex;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Cube {
    xs: isize,
    xe: isize,
    ys: isize,
    ye: isize,
    zs: isize,
    ze: isize,
}

impl Cube {
    fn well_formed(&self) -> bool {
        self.xs <= self.xe && self.ys <= self.ye && self.zs <= self.ze
    }

    fn intersection(&self, other: &Self) -> Option<Self> {
        let i = Self {
            xs: std::cmp::max(self.xs, other.xs),
            xe: std::cmp::min(self.xe, other.xe),
            ys: std::cmp::max(self.ys, other.ys),
            ye: std::cmp::min(self.ye, other.ye),
            zs: std::cmp::max(self.zs, other.zs),
            ze: std::cmp::min(self.ze, other.ze),
        };

        if i.well_formed() {
            Some(i)
        } else {
            None
        }
    }

    // Subtracts the other cube from this cube. Returns a list of cubes that
    // make up the current cube minus the new one. This is possibly empty.
    pub fn subtract(&self, other: &Self) -> Vec<Self> {
        // Look from each direction
        let inter = self.intersection(other);
        if inter.is_none() {
            return vec![*self];
        }

        let i_cube = inter.unwrap();
        if i_cube == *self {
            return vec![];
        }

        let mut cubes = vec![];
        if self.xs < i_cube.xs {
            cubes.push(Self {
                xs: self.xs, xe: i_cube.xs - 1,
                ..*self
            });
        }
        if self.xe > i_cube.xe {
            cubes.push(Self {
                xs: i_cube.xe + 1, xe: self.xe,
                ..*self
            });
        }

        if self.ys < i_cube.ys {
            cubes.push(Self {
                ys: self.ys, ye: i_cube.ys - 1,
                xs: i_cube.xs, xe: i_cube.xe,
                ..*self
            });
        }
        if self.ye > i_cube.ye {
            cubes.push(Self {
                ys: i_cube.ye + 1, ye: self.ye,
                xs: i_cube.xs, xe: i_cube.xe,
                ..*self
            });
        }

        if self.zs < i_cube.zs {
            cubes.push(Self {
                zs: self.zs, ze: i_cube.zs - 1,
                ..i_cube
            });
        }
        if self.ze > i_cube.ze {
            cubes.push(Self {
                zs: i_cube.ze + 1, ze: self.ze,
                ..i_cube
            });
        }

        cubes.into_iter().filter(|c| c.well_formed()).collect()
    }

    pub fn volume(&self) -> usize {
        ((self.xe - self.xs).abs() as usize + 1) *
        ((self.ye - self.ys).abs() as usize + 1) *
        ((self.ze - self.zs).abs() as usize + 1)
    }
}

fn read_it(filename: &str) -> Vec<(bool, Cube)> {
    let input = std::fs::read_to_string(filename).unwrap();
    let interval_regex = Regex::new(r"^[xyz]=([0-9\-]+)\.\.([0-9\-]+)$").unwrap();
    let mut ret = vec![];
    for line in input.lines() {
        let mut words = line.split(" ");
        let on = words.next().unwrap() == "on";

        let mut intervals = words.next().unwrap().split(",");
        let x_caps = interval_regex.captures(intervals.next().unwrap()).unwrap();
        let (xs, xe): (isize, isize) = (x_caps.get(1).unwrap().as_str().parse().unwrap(), x_caps.get(2).unwrap().as_str().parse().unwrap());
        let y_caps = interval_regex.captures(intervals.next().unwrap()).unwrap();
        let (ys, ye): (isize, isize) = (y_caps.get(1).unwrap().as_str().parse().unwrap(), y_caps.get(2).unwrap().as_str().parse().unwrap());
        let z_caps = interval_regex.captures(intervals.next().unwrap()).unwrap();
        let (zs, ze): (isize, isize) = (z_caps.get(1).unwrap().as_str().parse().unwrap(), z_caps.get(2).unwrap().as_str().parse().unwrap());

        ret.push((on, Cube {xs, xe, ys, ye, zs, ze}));
    }

    ret
}

fn drive(filename: &str, reboot: bool) {
    let input = read_it(filename);
    let mut on_cubes = LinkedList::new();

    for (on, cube) in input.iter() {
        if reboot && (cube.xs.abs() > 50 || cube.xe.abs() > 50 || cube.ys.abs() > 50 || cube.ye.abs() > 50 || cube.zs.abs() > 50 || cube.ze.abs() > 50) {
            continue;
        }

        let intersecting_cubes: Vec<Cube> = on_cubes.drain_filter(|c: &mut Cube| c.intersection(cube).is_some()).collect::<Vec<_>>();
        for int_cube in intersecting_cubes {
            let extras = int_cube.subtract(cube);
            for e in extras {
                on_cubes.push_back(e);
            }
        }

        if *on {
            on_cubes.push_back(*cube);
        }
    }

    let val = on_cubes.iter().fold(0, |acc, c| acc + c.volume());
    println!("{}", val);
}

#[test]
fn part_0() {
    drive("res/22/sample.txt", true);
    drive("res/22/sample.txt", false);
}

#[test]
fn part_1() {
    drive("res/22/input.txt", true);
}

#[test]
fn part_2() {
    drive("res/22/input.txt", false);
}
