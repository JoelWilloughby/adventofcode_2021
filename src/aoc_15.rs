use std::{cmp::Ordering, collections::{BinaryHeap, HashMap}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SearchCell {
    index: (usize, usize),
    parent: (usize, usize),
    distance: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SeenCell {
    parent: (usize, usize),
    distance: usize,
}

impl Ord for SearchCell {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for SearchCell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn read_it(filename: &str) -> Vec<Vec<usize>> {
    let input = std::fs::read_to_string(filename).unwrap();
    input.lines().map(|line| line.chars().map(|c| c as usize - '0' as usize).collect()).collect()
}

fn search(nums: &Vec<Vec<usize>>, start: (usize, usize), target: (usize, usize)) -> Vec<(usize, usize)> {
    let mut frontier : BinaryHeap<SearchCell> = BinaryHeap::new();
    let mut visited : HashMap<(usize, usize), SeenCell> = HashMap::new();
    frontier.push(SearchCell{index: start, parent: start, distance: 0});

    let rows = nums.len();
    let cols = nums[0].len();
    while !frontier.is_empty() {
        let next = frontier.pop().unwrap();
        if visited.contains_key(&next.index) {
            continue;
        }

        let (i, j) = next.index;
        let actual_distance = visited.get(&next.parent).unwrap_or(&SeenCell{distance: 0, parent: (0, 0)}).distance + nums[i][j];
        visited.insert(next.index, SeenCell {
            parent: next.parent,
            distance: actual_distance
        });

        if next.index == target {
            break;
        }

        for (r, c) in [(i, j+1), (i+2, j+1), (i+1, j), (i+1, j+2)] {
            if r < 1 || c < 1 || r >= rows+1 || c >= cols+1 {
                continue;
            }

            frontier.push(SearchCell{
                parent: next.index,
                index: (r-1 , c-1),
                distance: actual_distance + nums[r-1][c-1] + ((target.0 as isize - (r-1) as isize).abs() + (target.1 as isize - (c-1) as isize).abs()) as usize
            });
        }
    }

    let mut path_iter = target;
    let mut path = vec![];
    while path_iter != start {
        path.push(path_iter);
        path_iter = visited[&path_iter].parent;
    }

    path.push(start);
    path.reverse();

    path
}

fn drive(filename: &str) {
    let nums = read_it(filename);
    let path = search(&nums, (0, 0), (nums.len() - 1, nums[0].len() - 1));

    let val = path.iter().skip(1).fold(0, |acc, (i, j)| acc + nums[*i][*j]);

    println!("{}", val);
}

fn dist(nums: &Vec<Vec<usize>>, i: usize, j: usize) -> usize{
    let i_mul = i/nums.len();
    let j_mul = j/nums[0].len();
    let i_mod = i%nums.len();
    let j_mod = j%nums[0].len();

    1 + ((i_mul + j_mul + nums[i_mod][j_mod] - 1) % 9)
}

fn search_2(nums: &Vec<Vec<usize>>, start: (usize, usize), target: (usize, usize)) -> Vec<(usize, usize)> {
    let mut frontier : BinaryHeap<SearchCell> = BinaryHeap::new();
    let mut visited : HashMap<(usize, usize), SeenCell> = HashMap::new();
    frontier.push(SearchCell{index: start, parent: start, distance: 0});

    let rows = 5 * nums.len();
    let cols = 5 * nums[0].len();
    while !frontier.is_empty() {
        let next = frontier.pop().unwrap();
        if visited.contains_key(&next.index) {
            continue;
        }

        let (i, j) = next.index;

        let actual_distance = visited.get(&next.parent).unwrap_or(&SeenCell{distance: 0, parent: (0, 0)}).distance + dist(nums, i, j);
        visited.insert(next.index, SeenCell {
            parent: next.parent,
            distance: actual_distance
        });

        if next.index == target {
            break;
        }

        for (r, c) in [(i, j+1), (i+2, j+1), (i+1, j), (i+1, j+2)] {
            if r < 1 || c < 1 || r >= rows+1 || c >= cols+1 {
                continue;
            }

            frontier.push(SearchCell{
                parent: next.index,
                index: (r-1 , c-1),
                distance: actual_distance + dist(nums, r-1, c-1) + ((target.0 as isize - (r-1) as isize).abs() + (target.1 as isize - (c-1) as isize).abs()) as usize
            });
        }
    }

    let mut path_iter = target;
    let mut path = vec![];
    while path_iter != start {
        path.push(path_iter);
        path_iter = visited[&path_iter].parent;
    }

    path.push(start);
    path.reverse();

    path
}

fn drive_2(filename: &str) {
    let nums = read_it(filename);
    let path = search_2(&nums, (0, 0), (5 * nums.len() - 1, 5 * nums[0].len() - 1));

    let val = path.iter().skip(1).fold(0, |acc, (i, j)| acc + dist(&nums, *i, *j));

    println!("{}", val);
}

#[test]
fn part_0() {
    drive("res/15/sample.txt");
    drive_2("res/15/sample.txt");
}

#[test]
fn part_1() {
    drive("res/15/input.txt");
}

#[test]
fn part_2() {
    drive_2("res/15/input.txt");
}
