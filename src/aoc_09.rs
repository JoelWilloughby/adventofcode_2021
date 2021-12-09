use std::collections::BTreeSet;

fn read_it(filename: &str) -> Vec<Vec<usize>> {
    let input = std::fs::read_to_string(filename).unwrap();
    let mut out = vec![vec![]];
    for line in input.lines() {
        let mut l = vec![std::usize::MAX];
        line.trim().chars().map(|c| l.push(c as usize - '0' as usize)).count();
        l.push(std::usize::MAX);
        out.push(l);
    }
    out[0] = vec![std::usize::MAX; out[1].len()];
    out.push(vec![std::usize::MAX; out[1].len()]);
    out
}

fn drive(filename: &str) {
    let nums = read_it(filename);

    let mut acc = 0usize;
    for i in 1..(nums.len() - 1) {
        for j in 1..(nums[i].len() - 1) {
            if [(i-1, j), (i+1, j), (i, j-1), (i, j+1)].into_iter().all(|(x, y)| nums[i][j] < nums[x][y]) {
                acc += nums[i][j] + 1;
            }
        }
    }

    println!("{}", acc);
}

fn bfs(nums: &mut Vec<Vec<usize>>, i: usize, j: usize) -> usize {
    if nums[i][j] >= 9 {
        return 0;
    }

    let mut frontier : BTreeSet<(usize, usize)> = [(i, j)].into_iter().collect();
    let mut seen : BTreeSet<(usize, usize)> = BTreeSet::new();
    let mut acc = 0usize;
    while !frontier.is_empty() {
        let (i, j) = *frontier.iter().next().unwrap();
        frontier.remove(&(i, j));
        seen.insert((i, j));
        if nums[i][j] >= 9 {
            continue;
        }
        acc += 1;
        nums[i][j] = 9;
        [(i-1, j), (i+1, j), (i, j-1), (i, j+1)].into_iter().filter(|(x, y)| !seen.contains(&(*x, *y)))
            .map(|(x, y)| frontier.insert((x, y))).count();
    }
    acc
}

fn drive_2(filename: &str) {
    let mut nums = read_it(filename);

    let mut basins = Vec::new();
    for i in 1..(nums.len() - 1) {
        for j in 1..(nums[i].len() - 1) {
            let s = bfs(&mut nums, i, j);
            if s > 0 {
                basins.push(s);
            }
        }
    }

    basins.sort_by(|x, y| y.cmp(x));
    println!("{}", basins.iter().take(3).fold(1, |acc, x| acc * *x));
}

#[test]
fn part_0() {
    drive("res/09/sample.txt");
    drive_2("res/09/sample.txt");
}

#[test]
fn part_1() {
    drive("res/09/input.txt");
}

#[test]
fn part_2() {
    drive_2("res/09/input.txt");
}
