#[derive(Clone)]
enum Shell {
    Lit {
        val: usize,
    },
    Combo {
        left: Box<Shell>,
        right: Box<Shell>,
    }
}

impl Shell {
    fn str_helper(input: &mut Vec<char>) -> Self {
        let c = input.pop().unwrap();
        if c == '[' {
            let left = Box::new(Self::str_helper(input));
            let _comma = input.pop().unwrap();
            let right = Box::new(Self::str_helper(input));

            let _closing = input.pop().unwrap();
            Shell::Combo {
                left, right
            }
        } else {
            // It had better be a number
            Shell::Lit {
                val: c as usize - '0' as usize
            }
        }
    }

    pub fn from_str(input: &str) -> Self {
        let mut chars = input.chars().rev().collect();
        Self::str_helper(&mut chars)
    }

    fn add_left(&mut self, v: usize) {
        match self {
            Self::Lit {val} => {
                *val += v;
            },
            Self::Combo {left, ..} => {
                left.add_left(v);
            }
        }
    }

    fn add_right(&mut self, v: usize) {
        match self {
            Self::Lit {val} => {
                *val += v;
            },
            Self::Combo {right, ..} => {
                right.add_right(v);
            }
        }
    }

    fn explode(&mut self, depth: usize) -> Option<((usize, bool), (usize, bool))> {
        match self {
            Self::Lit {..} => {
                None
            },
            Self::Combo {left, right} => {
                if depth >= 4 {
                    if let Self::Lit{val} = left.as_ref() {
                        let left_val = *val;
                        if let Self::Lit{val} = right.as_ref() {
                            let right_val = *val;

                            // Explode
                            *self = Self::Lit{val: 0};
                            return Some(((left_val, false), (right_val, false)));
                        }
                    }
                }

                if let Some(((lv, lf), (rv, rf))) = left.explode(depth + 1) {
                    if !rf {
                        right.add_left(rv);
                        Some(((lv, lf), (rv, true)))
                    } else {
                        Some(((lv, lf), (rv, rf)))
                    }
                } else if let Some(((lv, lf), (rv, rf))) = right.explode(depth + 1) {
                    if !lf {
                        left.add_right(lv);
                        Some(((lv, true), (rv, rf)))
                    } else {
                        Some(((lv, lf), (rv, rf)))
                    }
                } else {
                    None
                }
            }
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Self::Lit {val} => {
                if *val >= 10 {
                    let left_val = *val / 2;
                    let right_val = *val - left_val;

                    *self = Self::Combo {
                        left: Box::new(Self::Lit{val: left_val}),
                        right: Box::new(Self::Lit{val: right_val}),
                    };

                    true
                } else {
                    false
                }
            },
            Self::Combo {left, right} => {
                left.split() || right.split()
            }
        }
    }

    pub fn reduce(&mut self) {
        // Look for things that need to be exploded
        loop {
            if self.explode(0).is_some() {
                continue;
            }
            if self.split() {
                continue;
            }
            break;
        }
    }

    pub fn add(self, other: Self) -> Self {
        let mut ret = Self::Combo {
            left: Box::new(self),
            right: Box::new(other),
        };

        ret.reduce();
        ret
    }

    pub fn magnitude(&self) -> usize {
        match self {
            Self::Lit{val} => {
                *val
            },
            Self::Combo{left, right} => {
                3 * left.magnitude() + 2 * right.magnitude()
            }
        }
    }

    fn fmt_h(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            &Shell::Lit{val} => {
                write!(f, "{}", val)?;
            },
            &Shell::Combo{left, right} => {
                write!(f, "[")?;
                left.fmt_h(f)?;
                write!(f, ",")?;
                right.fmt_h(f)?;
                write!(f, "]")?;
            }
        }
        Ok(())
    }
}

impl std::fmt::Display for Shell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}) ", self.magnitude())?;
        self.fmt_h(f)
    }
}

fn read_it(filename: &str) -> Vec<Shell> {
    let input = std::fs::read_to_string(filename).unwrap();
    input.lines().map(|line| Shell::from_str(line)).collect()
}

fn drive(filename: &str) {
    let shells = read_it(filename);

    let mut shells_iter = shells.into_iter();
    let mut shell = shells_iter.next().unwrap();
    for next in shells_iter {
        shell = shell.add(next);
    }

    println!("Res: {}", shell);
}

fn drive_2(filename: &str) {
    let shells = read_it(filename);

    let mut max = 0;
    for i in 0..shells.len() {
        for j in 0..shells.len() {
            let res = shells[i].clone().add(shells[j].clone()).magnitude();
            max = std::cmp::max(max, res);
        }
    }

    println!("{}", max);
}

#[test]
fn part_0() {
    drive("res/18/sample.txt");
    drive_2("res/18/sample.txt");
}

#[test]
fn part_1() {
    drive("res/18/input.txt");
}

#[test]
fn part_2() {
    drive_2("res/18/input.txt");
}
