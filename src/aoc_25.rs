
#[derive(Clone, Copy, Debug, PartialEq)]
enum Cuc {
    D,
    R
}

fn read_it(filename: &str) -> Vec<Vec<Option<Cuc>>> {
    let input = std::fs::read_to_string(filename).unwrap();
    input.lines().map(|line| {
        line.chars().map(|c| match c {
            '>' => Some(Cuc::R),
            'V' | 'v' => Some(Cuc::D),
            _ => None
        }).collect()
    }).collect()
}

fn step(input: &Vec<Vec<Option<Cuc>>>) -> Vec<Vec<Option<Cuc>>> {
    let mut out = vec![];
    let mut in_mod = input.clone();
    let rows = input.len();
    let cols = input[0].len();
    for _ in 0..rows {
        let out_line = vec![None; cols];
        out.push(out_line);
    }

    for i in 0..rows {
        for j in 0..cols {
            if let Some(Cuc::R) = input[i][j] {
                if in_mod[i][(j+1) % cols].is_none() {
                    out[i][(j+1) % cols] = Some(Cuc::R);
                } else {
                    out[i][j] = Some(Cuc::R);
                }
            }
        }
    }

    for i in 0..rows {
        for j in 0..cols {
            if let Some(Cuc::R) = input[i][j] {
                in_mod[i][j] = None;
            }
        }
    }

    for i in 0..rows {
        for j in 0..cols {
            if let Some(Cuc::R) = out[i][j] {
                in_mod[i][j] = Some(Cuc::R);
            }
        }
    }

    for i in 0..rows {
        for j in 0..cols {
            if let Some(Cuc::D) = input[i][j] {
                if in_mod[(i+1) % rows][j].is_none() {
                    out[(i+1) % rows][j] = Some(Cuc::D);
                } else {
                    out[i][j] = Some(Cuc::D);
                }
            }
        }
    }

    out
}

fn eq(a: &Vec<Vec<Option<Cuc>>>, b: &Vec<Vec<Option<Cuc>>>) -> bool {
    for i in 0..a.len() {
        for j in 0..a[i].len() {
            if a[i][j] != b[i][j] {
                return false;
            }
        }
    }
    true
}

fn drive(filename: &str) {
    let mut input = read_it(filename);

    let mut count = 0;
    loop {
        let next = step(&input);
        count += 1;
        if eq(&input, &next) {
            break;
        }
        input = next;
    }

    println!("{}", count);
}

#[test]
fn part_0() {
    drive("res/25/sample.txt");
}

#[test]
fn part_1() {
    drive("res/25/input.txt");
}
