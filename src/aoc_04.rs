use std::collections::{HashMap, LinkedList};

#[derive(Debug, Clone)]
struct Bingo {
    cells: HashMap<usize, (usize, usize)>,
    drawn: HashMap<usize, bool>,
    row_count: [usize; 5],
    col_count: [usize; 5],
}

impl Bingo {
    fn from_file(filename: &str) -> (Vec<usize>, Vec<Bingo>) {
        let input = std::fs::read_to_string(filename).unwrap();
        let mut lines = input.trim().lines();

        let vals = lines.next().unwrap().trim().split(",").map(|x| x.parse::<usize>().unwrap()).collect();

        let mut boards = Vec::new();
        let mut curr_line = 0;
        let mut cells = HashMap::new();
        let mut drawn = HashMap::new();
        for line in lines {
            if line.is_empty() {
                continue;
            }

            line.trim().split_whitespace().map(|x| x.parse::<usize>().unwrap()).enumerate().for_each(|(i, v)| {
                cells.insert(v, (curr_line, i));
                drawn.insert(v, false);
            });

            curr_line += 1;
            if curr_line == 5 {
                boards.push(Bingo {
                    cells: cells.clone(),
                    drawn: drawn.clone(),
                    row_count: [0; 5],
                    col_count: [0; 5]
                });

                curr_line = 0;
                cells.clear();
                drawn.clear();
            }
        }

        (vals, boards)
    }

    fn win(&self) -> bool {
        self.row_count.iter().any(|x| *x == 5) || self.col_count.iter().any(|x| *x == 5)
    }

    fn draw(&mut self, val: usize) -> bool {
        if !self.drawn.contains_key(&val) || self.drawn[&val] {
            return false;
        }
        let (r, c) = self.cells[&val];
        self.row_count[r] += 1;
        self.col_count[c] += 1;
        *self.drawn.get_mut(&val).unwrap() = true;
        self.win()
    }

    fn score(&self) -> usize {
        self.drawn.iter().fold(0, |acc, (i, x)| if *x {acc} else {acc + i})
    }
}


fn play(boards: &mut Vec<Bingo>, vals: &Vec<usize>) -> Option<(Bingo, usize)> {
    for val in vals {
        for board in boards.iter_mut() {
            if board.draw(*val) {
                return Some((board.clone(), *val));
            }
        }
    }

    None
}

fn play_2(boards: &mut Vec<Bingo>, vals: &Vec<usize>) -> Option<(Bingo, usize)> {
    let mut boards_list: LinkedList<Bingo> = boards.clone().into_iter().collect();
    for val in vals {
        boards_list.iter_mut().for_each(|board| {board.draw(*val);});
        if boards_list.len() == 1  && boards_list.front().unwrap().win() {
            return Some((boards_list.front().unwrap().clone(), *val));
        }
        boards_list = boards_list.into_iter().filter(|board| !board.win()).collect();
    }

    None
}

fn drive(filename: &str) {
    let (vals, mut boards) = Bingo::from_file(filename);

    let mut acc = 0usize;
    if let Some((board, val)) = play(&mut boards, &vals) {
        println!("{} {:?}", val, board);
        acc = val * board.score();
    }

    println!("{}", acc);
}

fn drive_2(filename: &str) {
    let (vals, mut boards) = Bingo::from_file(filename);

    let mut acc = 0usize;
    if let Some((board, val)) = play_2(&mut boards, &vals) {
        println!("{} {:?}", val, board);
        acc = val * board.score();
    }

    println!("{}", acc);
}

#[test]
fn part_0() {
    drive("res/04/sample.txt");
    drive_2("res/04/sample.txt");
}

#[test]
fn part_1() {
    drive("res/04/input.txt");
}

#[test]
fn part_2() {
    drive_2("res/04/input.txt");
}
