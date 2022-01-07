
#[derive(Copy, Clone, Debug)]
enum Amphipod {
    A,
    B,
    C,
    D,
}

#[derive(Clone, Debug)]
enum Index {
    R(usize, usize),
    H(usize)
}

#[derive(Clone)]
struct Game {
    hallway: [Option<Amphipod>; 11],
    rooms: [Vec<Amphipod>; 4],
    room_size: usize,
}

impl Amphipod {
    pub fn cost(&self) -> usize {
        match self {
            Self::A => 1,
            Self::B => 10,
            Self::C => 100,
            Self::D => 1000,
        }
    }

    pub fn home(&self) -> usize {
        match self {
            Self::A => 0,
            Self::B => 1,
            Self::C => 2,
            Self::D => 3,
        }
    }
}

impl std::fmt::Display for Amphipod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::A => {"A"},
            Self::B => {"B"},
            Self::C => {"C"},
            Self::D => {"D"},
        })?;

        Ok(())
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "#")?;
        for space in self.hallway.iter() {
            if let Some(a) = space {
                write!(f, "{}", a)?;
            } else {
                write!(f, ".")?;
            }
        }
        writeln!(f, "#")?;

        for depth in 0..self.room_size {
            write!(f, "###")?;
            let curr_room = self.room_size - depth - 1;
            for room in self.rooms.iter() {
                if room.len() > curr_room {
                    write!(f, "{}", room[curr_room])?;
                } else {
                    write!(f, ".")?;
                }
                write!(f, "#")?;
            }
            writeln!(f, "##")?;
        }

        Ok(())
    }
}

impl Game {
    fn new(rooms: [Vec<Amphipod>; 4]) -> Self {
        Self {
            hallway: [None; 11],
            room_size: rooms[0].len(),
            rooms,
        }
    }

    fn can_move_into(&self, amphi: Amphipod, r: usize) -> bool {
        amphi.home() == r && self.rooms[r].len() < self.room_size && self.room_partial_done(r)
    }

    fn possibilities_from_room(&self, amphi: Amphipod, room: usize, index: usize) -> Vec<(Index, usize)> {
        let mut ret = vec![];
        let hallway = 2 * room + 2;
        let c = self.room_size - index;

        // look left
        let mut steps = 1;
        while hallway >= steps {
            let temp: usize = hallway - steps;
            if self.hallway[temp].is_some() {
                break;
            }

            ret.push((Index::H(temp), steps + c));

            if temp > 2 {
                let next_room = (temp - 3) / 2;
                if self.can_move_into(amphi, next_room) {
                    let home_steps = self.room_size - self.rooms[next_room].len();
                    ret.push((Index::R(next_room, self.room_size - home_steps), c + steps + 1 + (home_steps)))
                }

                steps += 2;
            } else {
                steps += 1;
            }
        }

        // look right
        let mut steps = 1;
        while hallway + steps < self.hallway.len() {
            let temp = hallway + steps;
            if self.hallway[temp].is_some() {
                break;
            }

            ret.push((Index::H(temp), steps + c));

            if temp < self.hallway.len() - 3 {
                let next_room = (temp - 1) / 2;
                if self.can_move_into(amphi, next_room) {
                    let home_steps = self.room_size - self.rooms[next_room].len();
                    ret.push((Index::R(next_room, self.room_size - home_steps), c + steps + 1 + (home_steps)))
                }

                steps += 2;
            } else {
                steps += 1;
            }
        }

        ret
    }

    fn possibilities_from_hallway(&self, amphi: Amphipod, hallway: usize) -> Vec<(Index, usize)>{
        let home = amphi.home();
        let target_hallway = 2 * home + 2;
        let start = std::cmp::min(target_hallway, hallway);
        let end = std::cmp::max(target_hallway, hallway);

        for h in start..=end {
            if self.hallway[h].is_some() {
                return vec![];
            }
        }

        if self.can_move_into(amphi, home) {
            let home_steps = self.room_size - self.rooms[home].len();
            return vec![(Index::R(home, self.room_size - home_steps), end - start + home_steps)]
        }

        vec![]
    }

    fn possibilities(&self, amphi: Amphipod, index: Index) -> Vec<(Index, usize)> {
        match index {
            Index::R(r, i) => {
                self.possibilities_from_room(amphi, r, i)
            },
            Index::H(h) => {
                self.possibilities_from_hallway(amphi, h)
            }
        }
    }

    fn room_partial_done(&self, r: usize) -> bool {
        self.rooms[r].iter().all(|a| r == a.home())
    }

    fn room_done(&self, r: usize) -> bool {
        self.rooms[r].len() == self.room_size && self.room_partial_done(r)
    }

    fn solve(&mut self, cost: usize, memo: &mut std::collections::HashMap<String, Option<usize>>) -> Option<usize> {
        if let Some(res) = memo.get(&format!("{}", self)).clone() {
            if let Some(cost_from_here) = *res {
                return Some(cost + cost_from_here);
            } else {
                return None;
            }
        }

        let mut allgood = true;
        let mut min_cost = None;
        for r in 0..self.rooms.len() {
            if self.room_done(r) {
                continue;
            }

            allgood = false;

            if self.rooms[r].is_empty() {
                continue;
            }

            if self.room_partial_done(r) {
                continue;
            }

            let amphi = self.rooms[r].pop().unwrap();
            let apossibilities = self.possibilities(amphi, Index::R(r, self.rooms[r].len()));

            for (index, steps) in apossibilities {
                match index {
                    Index::R(r, i) => {
                        assert!(self.rooms[r].len() == i);
                        self.rooms[r].push(amphi);
                        if let Some(c) = self.solve(cost + steps * amphi.cost(), memo) {
                            min_cost = Some(std::cmp::min(min_cost.unwrap_or(std::usize::MAX), c));
                        }
                        self.rooms[r].pop();
                    },
                    Index::H(h) => {
                        assert!(self.hallway[h].is_none());
                        self.hallway[h] = Some(amphi);
                        if let Some(c) = self.solve(cost + steps * amphi.cost(), memo) {
                            min_cost = Some(std::cmp::min(min_cost.unwrap_or(std::usize::MAX), c));
                        }
                        self.hallway[h] = None;
                    }
                }
            }

            self.rooms[r].push(amphi);
        }

        if allgood {
            memo.insert(format!("{}", self), Some(0));
            return Some(cost);
        }

        for h in 0..self.hallway.len() {
            if let Some(amphi) = self.hallway[h].clone() {
                self.hallway[h] = None;
                let apossibilities = self.possibilities(amphi, Index::H(h));
                for (index, steps) in apossibilities {
                    match index {
                        Index::R(r, i) => {
                            assert!(self.rooms[r].len() == i);
                            self.rooms[r].push(amphi);
                            if let Some(c) = self.solve(cost + steps * amphi.cost(), memo) {
                                min_cost = Some(std::cmp::min(min_cost.unwrap_or(std::usize::MAX), c));
                            }
                            self.rooms[r].pop();
                        },
                        _ => {panic!("Got a hallway move from a hallway");}
                    }
                }

                self.hallway[h] = Some(amphi);
            }
        }

        memo.insert(format!("{}", self), min_cost.map(|final_c| final_c - cost));
        min_cost
    }

    fn do_it(&mut self) -> usize {
        self.solve(0, &mut std::collections::HashMap::new()).unwrap()
    }
}

fn drive(rooms: [Vec<Amphipod>; 4]) {
    let mut game = Game::new(rooms);
    println!("{}", game);

    let val = game.do_it();
    println!("{}", val);
}

#[test]
fn part_0() {
    drive([
        vec![Amphipod::A, Amphipod::D, Amphipod::D, Amphipod::B],
        vec![Amphipod::D, Amphipod::B, Amphipod::C, Amphipod::C],
        vec![Amphipod::C, Amphipod::A, Amphipod::B, Amphipod::B],
        vec![Amphipod::A, Amphipod::C, Amphipod::A, Amphipod::D],
        ]);

    drive([
        vec![Amphipod::A, Amphipod::B],
        vec![Amphipod::D, Amphipod::C],
        vec![Amphipod::C, Amphipod::B],
        vec![Amphipod::A, Amphipod::D],
        ]);
}

#[test]
fn part_1() {
    drive([
        vec![Amphipod::D, Amphipod::C],
        vec![Amphipod::A, Amphipod::C],
        vec![Amphipod::B, Amphipod::B],
        vec![Amphipod::A, Amphipod::D],
        ]);
}

#[test]
fn part_2() {
    drive([
        vec![Amphipod::D, Amphipod::D, Amphipod::D, Amphipod::C],
        vec![Amphipod::A, Amphipod::B, Amphipod::C, Amphipod::C],
        vec![Amphipod::B, Amphipod::A, Amphipod::B, Amphipod::B],
        vec![Amphipod::A, Amphipod::C, Amphipod::A, Amphipod::D],
        ]);
}
