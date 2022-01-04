
fn read_it(filename: &str) -> (Vec<bool>, Vec<Vec<bool>>) {
    let input = std::fs::read_to_string(filename).unwrap();
    let mut lines = input.lines();

    let key_line = lines.next().unwrap();
    let mut state = vec![];
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }

        state.push( line.chars().map(|c| c == '#').collect() )
    }

    let key = key_line.chars().map(|c| c == '#').collect();

    (key, state)
}


fn get(state: &Vec<Vec<bool>>, i: isize, j: isize, default: bool) -> bool {
    if i < 0 || j < 0 || i >= state.len() as isize || j >= state[0].len() as isize {
        default
    } else {
        state[i as usize][j as usize]
    }
}

fn get_it(state: &Vec<Vec<bool>>, i: isize, j: isize, default: bool) -> usize {
    let mut acc = 0;
    for x in -1..=1 {
        for y in -1..=1 {
            acc <<= 1;
            if get(state, i + x, j + y, default) {
                acc += 1;
            }
        }
    }

    acc
}

fn step(key: &Vec<bool>, state: Vec<Vec<bool>>, default: bool) -> (Vec<Vec<bool>>, bool) {
    let nrows = state.len();
    let ncols = state[0].len();

    (
        (-2..=(nrows as isize + 2)).into_iter().map(|x| {
            (-2..=(ncols as isize + 2)).into_iter().map(|y| {
                let key_val = get_it(&state, x, y, default);
                key[key_val]
            }).collect()
        }).collect(),
        if !default && key[0] {true} else if default && !key[511] {false} else {default}
    )
}

fn drive(filename: &str, n: usize) {
    let (key, mut state) = read_it(filename);

    let mut default = false;
    for _ in 0..n {
        let (s, d) = step(&key, state, default);
        default = d;
        state = s;
    }

    let val = state.iter().fold(0, |acc, x| acc + x.iter().fold(0, |ac, y| if *y {ac + 1} else {ac}));
    println!("{}", val);
}

#[test]
fn part_0() {
    drive("res/20/sample.txt", 2);
    drive("res/20/sample.txt", 50);
}

#[test]
fn part_1() {
    drive("res/20/input.txt", 2);
}

#[test]
fn part_2() {
    drive("res/20/input.txt", 50);
}
