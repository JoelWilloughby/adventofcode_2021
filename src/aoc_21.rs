
struct Player {
    score: usize,
    state: usize,
}

struct Game {
    dice: usize,
    p1: Player,
    p2: Player,
    turn: bool,
}

impl Game {
    fn new(p1: usize, p2: usize) -> Self {
        Self {
            dice: 0,
            p1: Player{score: 0, state: p1},
            p2: Player{score: 0, state: p2},
            turn: true
        }
    }

    fn modulo(n: usize) -> usize {
        let val = n % 10;
        if val == 0 {
            10
        } else {
            val
        }
    }

    fn step(&mut self) -> bool {
        let player = if self.turn {&mut self.p1} else {&mut self.p2};

        let travel: usize = 3 * self.dice + 6;
        self.dice += 3;
        self.dice %= 100;

        player.state += travel;
        player.state = Self::modulo(player.state);
        player.score += player.state;

        self.turn = !self.turn;

        player.score >= 1000
    }
}

fn f((s1, c1): (usize, usize), (s2, c2): (usize, usize), turn: usize) -> (usize, usize) {
    // Might be memoizable, but this works for the size inputs we are given (up to 21)
    static POSSIBILITIES: [(usize, usize); 7] = [(1, 3), (3, 4), (6, 5), (7, 6), (6, 7), (3, 8), (1, 9)];

    if turn == 0 && s2 >= 21 {
        // Player 2 just won
        return (0, 1);
    } else if turn == 1 && s1 >= 21 {
        // Player 1 just won
        return (1, 0);
    }

    // Try all possible 3 dice combinations
    POSSIBILITIES.iter().fold((0, 0), |(lacc, racc), (c, v)| {
        if turn == 0 {
            let next_val = Game::modulo(c1 + *v);
            let (l, r) = f((s1 + next_val, next_val), (s2, c2), 1);
            (lacc + *c * l, racc + *c * r)
        } else {
            let next_val = Game::modulo(c2 + v);
            let (l, r) = f((s1, c1), (s2 + next_val, next_val), 0);
            (lacc + *c * l, racc + *c * r)
        }
    })
}

fn drive(p1: usize, p2: usize) {
    let mut game = Game::new(p1, p2);

    let mut count = 0;
    loop {
        count += 3;
        if game.step() {
            break;
        }
    }

    let min_score = std::cmp::min(game.p1.score, game.p2.score);
    println!("{} {} {}", count, game.p1.score, game.p2.score);
    println!("{}", count * min_score);
}

fn drive_2(p1: usize, p2: usize) {
    let (l, r) = f((0, p1), (0, p2), 0);
    println!("{} {}", l, r);
}

#[test]
fn part_0() {
    drive(4, 8);
    drive_2(4, 8);
}

#[test]
fn part_1() {
    drive(2, 1);
}

#[test]
fn part_2() {
    drive_2(2, 1);
}
