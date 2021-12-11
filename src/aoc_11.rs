
fn read_it(filename:&str) -> Vec<Vec<usize>> {
    let input = std::fs::read_to_string(filename).unwrap();
    input.lines().map(|line| {
        line.trim().chars().map(|x| x as usize - '0' as usize).collect()
    }).collect()
}

fn explode(mat: &mut Vec<Vec<usize>>) -> bool {
    let rows = mat.len() as isize;
    let cols = mat[0].len() as isize;
    for i in 0..rows {
        for j in 0..cols {
            if mat[i as usize][j as usize] == 10 {
                let neighbors = [(i-1,j-1),(i-1,j),(i-1,j+1),
                                 (i,j-1),(i,j+1),
                                 (i+1,j-1),(i+1,j),(i+1,j+1)];
                for (x, y) in neighbors {
                     if x < 0 || y < 0 || x >= rows || y >= cols {
                         continue;
                     }

                     if mat[x as usize][y as usize] < 10 { 
                        mat[x as usize][y as usize] += 1; 
                     }
                }

                mat[i as usize][j as usize] = 11;
            }
        }
    }

    let mut acc = 0usize;
    for i in 0..rows {
        for j in 0..cols {
            if mat[i as usize][j as usize] == 11 {
                acc += 1;
                mat[i as usize][j as usize] = 12;
            }
        }
    }

    acc > 0
}

fn cycle(mat: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut next : Vec<Vec<usize>> = mat.iter().map(|line| line.iter().map(|x| *x + 1).collect()).collect();

    while explode(&mut next) { }

    next.iter().map(|line| line.iter().map(|x| if *x > 9 { 0 } else { *x }).collect()).collect()
}

fn drive(filename: &str) {
    let mut state = read_it(filename);

    let mut acc = 0;
    for _ in 0..100 {
        state = cycle(state);

        acc = state.iter().fold(acc, |ac, line| ac + line.iter().fold(0, |a, x| if *x == 0 {a + 1} else {a}));
    }
    state.iter().for_each(|line| {println!("{:?}", line);});
    println!("");
    println!("{}", acc);
}

fn drive_2(filename: &str) {
    let mut state = read_it(filename);

    let mut i = 0;
    loop {
        state = cycle(state);
        i += 1;

        let acc = state.iter().fold(0, |ac, line| ac + line.iter().fold(0, |a, x| if *x == 0 {a + 1} else {a}));
        if acc == state.len() * state[0].len() {
            break;
        }
        // state.iter().for_each(|line| {println!("{:?}", line);});
        // println!("");
    }
    state.iter().for_each(|line| {println!("{:?}", line);});
    println!("");
    println!("{}", i);
}

#[test]
fn part_0() {
    drive("res/11/sample.txt");
    drive_2("res/11/sample.txt");
}

#[test]
fn part_1() {
    drive("res/11/input.txt");
}

#[test]
fn part_2() {
    drive_2("res/11/input.txt");
}
