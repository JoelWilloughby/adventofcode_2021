use std::collections::BTreeSet;

#[derive(PartialEq, Eq, Ord, PartialOrd, Clone, Copy, Debug)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

struct Scans {
    vals: Vec<Point>,
    dists: Vec<Vec<Point>>,
    pairwise: Vec<Vec<usize>>,
    oriented: Vec<Vec<Point>>,
    orientation: usize,
    offset: Point
}

impl Point {
    fn get(&self, orientation: usize) -> Point {
        let (x_index, y_index, z_index, x_dir, y_dir, z_dir) = [
            (0, 1, 2,  1,  1,  1),
            (0, 2, 1,  1,  1, -1),
            (0, 1, 2,  1, -1, -1),
            (0, 2, 1,  1, -1,  1),

            (0, 1, 2, -1,  1, -1),
            (0, 2, 1, -1, -1, -1),
            (0, 1, 2, -1, -1,  1),
            (0, 2, 1, -1,  1,  1),

            (1, 0, 2,  1,  1, -1),
            (1, 2, 0,  1,  1,  1),
            (1, 0, 2, -1,  1,  1),
            (1, 2, 0, -1,  1, -1),

            (1, 0, 2,  1, -1,  1),
            (1, 2, 0, -1, -1,  1),
            (1, 0, 2, -1, -1, -1),
            (1, 2, 0,  1, -1, -1),

            (2, 0, 1,  1,  1,  1),
            (2, 1, 0, -1,  1,  1),
            (2, 0, 1, -1, -1,  1),
            (2, 1, 0,  1, -1,  1),

            (2, 0, 1,  1, -1, -1),
            (2, 1, 0,  1,  1, -1),
            (2, 0, 1, -1,  1, -1),
            (2, 1, 0, -1, -1, -1)
            ][orientation];

        let vals = [self.x, self.y, self.z];

        Self {
            x: vals[x_index] * x_dir,
            y: vals[y_index] * y_dir,
            z: vals[z_index] * z_dir,
        }
    }

    fn sq_dist(&self) -> usize {
        self.x.abs() as usize + self.y.abs() as usize + self.z.abs() as usize
        // self.x as usize * self.x as usize +
        // self.y as usize * self.y as usize +
        // self.z as usize * self.z as usize
    }

    fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Scans {
    pub fn new(vals: Vec<Vec<isize>>) -> Self {
        let dists : Vec<Vec<Point>> = (0..vals.len()).into_iter().map(|i| {
            (0..vals.len()).into_iter().map(|j| {
                let left = &vals[i];
                let right = &vals[j];

                Point{
                    x: left[0] - right[0], 
                    y: left[1] - right[1], 
                    z: left[2] - right[2]
                }
            }).collect()
        }).collect();

        let vals = vals.into_iter().map(|vs| Point {
            x: vs[0], y: vs[1], z: vs[2]
        }).collect();

        Self {
            vals,
            oriented: dists.clone(),
            pairwise: dists.iter().map(|ps| ps.iter().map(|p| p.sq_dist()).collect()).collect(),
            dists,
            orientation: 0,
            offset: Point{x:0, y:0, z:0}
        }
    }

    pub fn orient_to(&mut self, other: &Self) -> bool {
        for self_row in self.pairwise.iter() {
            for other_row in other.pairwise.iter() {
                let mut count = 0;
                for self_dist in self_row {
                    for other_dist in other_row {
                        if self_dist == other_dist {
                            count += 1;
                        }
                    }
                }
                if count >= 12 {
                    return self.orient_to_long(other);
                }
            }
        } 
        false
    }

    pub fn orient_to_long(&mut self, other: &Self) -> bool {
        for orientation in 0..24 {
            let mut pairs = vec![];
            for (si, self_row) in self.dists.iter().enumerate() {
                for (oi, other_row) in other.oriented.iter().enumerate() {
                    let mut counter = 0;
                    for self_pair in self_row {
                        for other_pair in other_row {
                            // if orientation == 0 &&  si == 0 && oi == 9 {
                            //     println!("{} {:?} {:?}", orientation, self_pair, other_pair);
                            // }
                            if self_pair.get(orientation) == *other_pair {
                                counter += 1;
                                if counter >= 12 {
                                    break;
                                }
                            }
                        }
                        if counter >= 12 {
                            pairs.push((self.vals[si].get(orientation), other.vals[oi].get(other.orientation).add(&other.offset)));
                        }
                    }
                }
            }

            // println!("{} {}", orientation, pairs.len());
            if pairs.len() >= 12 {
                // Found 12 similar points...
                let mut diff = Point{x:0, y:0, z:0};
                for (self_point, other_point) in pairs {
                    diff = Point{
                        x: other_point.x - self_point.x,
                        y: other_point.y - self_point.y,
                        z: other_point.z - self_point.z,
                    };
                    // println!("{:?}", diff);
                }

                self.orientation = orientation;
                self.offset = diff;
                self.oriented = self.dists.iter().map(|ds| ds.iter().map(|d| d.get(orientation)).collect()).collect();
                return true;
            }
        }

        false
    }
}

fn read_it(filename: &str) -> Vec<Vec<Vec<isize>>> {
    let input = std::fs::read_to_string(filename).unwrap();

    let mut lines = input.lines();

    let mut ret = vec![];
    while let Some(line) = lines.next() {
        if line.is_empty() {
            continue;
        }

        let mut vals = vec![];
        while let Some(val_line) = lines.next() {
            if val_line.is_empty() {
                break;
            }
            let val_vec : Vec<isize> = val_line.split(",").map(|x| x.parse().unwrap()).collect();
            vals.push(val_vec);
        }

        if !vals.is_empty() {
            ret.push(vals);
        }
    }

    ret
}

fn drive(filename: &str) {
    let vals = read_it(filename);
    for (i, vs) in vals.iter().enumerate() {
        println!("Scanner {}", i);
        println!("{:?}", vs);
    }

    let mut scanners: Vec<Scans> = vals.into_iter().rev().map(|vs| Scans::new(vs)).collect();
    let mut oriented = vec![];
    let mut skipped = vec![];
    while !scanners.is_empty() {
        let mut found = oriented.is_empty();
        let mut scanner = scanners.pop().unwrap();
        for o in oriented.iter() {
            if scanner.orient_to(o) {
                found = true;
                println!("Oriented: {}", scanner.orientation);
                break;
            }
        }
        if found {
            oriented.push(scanner);
        } else {
            println!("Skipping {:?}", scanner.vals);
            skipped.push(scanner);
        }

        if scanners.is_empty() {
            scanners = skipped.drain(0..).collect();
            skipped = vec![];
        }
    }

    let mut points = BTreeSet::new();
    for o in oriented.iter() {
        for p in &o.vals {
            points.insert(p.get(o.orientation).add(&o.offset));
        }
    }
    println!("{}", points.len());

    let mut max = 0;
    for i in 0..oriented.len() {
        println!("{:?}", oriented[i].offset);
        for j in 0..oriented.len() {
            if i == j {
                continue;
            }

            let pi = oriented[i].offset;
            let pj = oriented[j].offset;
            let dist = (pi.x - pj.x).abs() + (pi.y - pj.y).abs() + (pi.z - pj.z).abs();
            max = std::cmp::max(max, dist);
        }
    }

    println!("{}", max);
    // let mut s0 = Scans::new(vals[4].clone());
    // s0.orientation = 13;
    // let mut s2 = Scans::new(vals[2].clone());

    // s2.orient_to(&s0);
    // let s0_set: BTreeSet<Point> = s0.vals.iter().map(|p| p.get(s0.orientation).add(&s0.offset)).collect();
    // let s2_set: BTreeSet<Point> = s2.vals.iter().map(|p| p.get(s2.orientation).add(&s2.offset)).collect();

    // let intersect: BTreeSet<&Point> = s0_set.intersection(&s2_set).collect();

    // println!("{:?}", intersect);
}

#[test]
fn part_0() {
    drive("res/19/sample.txt");
}

#[test]
fn part_1() {
    drive("res/19/input.txt");
}