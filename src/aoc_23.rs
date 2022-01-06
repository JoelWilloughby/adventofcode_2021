
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

    // will be sorted based on cost to move (lowest first)
    things: Vec<(Amphipod, Index)>,
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
        writeln!(f, "#############")?;
        write!(f, "#")?;
        for space in self.hallway.iter() {
            if let Some(a) = space {
                write!(f, "{}", a)?;
            } else {
                write!(f, ".")?;
            }
        }
        writeln!(f, "#")?;

        write!(f, "###")?;
        for room in self.rooms.iter() {
            if room.len() > 1 {
                write!(f, "{}", room[1])?;
            } else {
                write!(f, ".")?;
            }
            write!(f, "#")?;
        }
        writeln!(f, "##")?;

        write!(f, "###")?;
        for room in self.rooms.iter() {
            if room.len() > 0 {
                write!(f, "{}", room[0])?;
            } else {
                write!(f, ".")?;
            }
            write!(f, "#")?;
        }
        writeln!(f, "##")?;

        writeln!(f, "  #########  ")?;

        Ok(())
    }
}

impl Game {
    fn new(rooms: [Vec<Amphipod>; 4]) -> Self {
        let mut things = vec![];
        for (i, room) in rooms.iter().enumerate() {
            for (j, amphi) in room.iter().enumerate() {
                things.push((*amphi, Index::R(i, j)));
            }
        }

        things.sort_by(|(a, _), (b, _)| {
            a.cost().cmp(&b.cost())
        });

        println!("{:?}", things);

        Self {
            hallway: [None; 11],
            rooms,
            things,
        }
    }

    fn possibilities_from_hallway(&self, hallway: usize, c: usize) -> Vec<(Index, usize)> {
        let mut ret = vec![];
        // look left
        let mut steps = 1;
        while hallway >= steps {
            let temp = hallway - steps;
            if self.hallway[temp].is_some() {
                break;
            }

            ret.push((Index::H(temp), steps + c));
            if temp > 2 {
                let next_room = (temp - 2) / 2;
                if rooms[next_room].len() < 2 {
                    ret.push(Index::R(next_room), c + steps + 1 + (2 - rooms[next_room].len()))
                }

                steps += 2;
            } else {
                steps += 1;
            }
        }

        // look right
        let mut steps = 1;
        while self.hallway.len() - hallway > steps {
            let temp = hallway + steps;
            if self.hallway[temp].is_some() {
                break;
            }

            ret.push((Index::H(temp), steps + c));
            if temp < self.hallway.len() - 3 {
                let next_room = (temp - 2) / 2
                if rooms[next_room].len() < 2 {
                    ret.push(Index::R(next_room), c + steps + 1 + (2 - rooms[next_room].len()))
                }

                steps += 2;
            } else {
                steps += 1;
            }
        }

        ret
    }

    fn possibilities(&self, index: Index) -> Vec<Index> {
        static ROOM_LOOKUP = [2, 4, 6, 8];
        match index {
            Index::R(r, i) => {
                if i == 0 && self.rooms.len() > 1 {
                    return vec![];
                }

                self.possibilities_from_hallway(ROOM_LOOKUP[r], 1)
            },
            Index::H(h) => {
                self.possibilities_from_hallway(ROOM_LOOKUP[r], 0)
            }
        }
    }

    fn solve(&mut self) -> usize {
        for thing in self.things.iter() {

        }
    }
}

fn drive(rooms: [Vec<Amphipod>; 4]) {
    let game = Game::new(rooms);

    println!("{}", game);
}

#[test]
fn part_0() {
    drive([
        vec![Amphipod::A, Amphipod::B],
        vec![Amphipod::D, Amphipod::C],
        vec![Amphipod::C, Amphipod::B],
        vec![Amphipod::A, Amphipod::D],
        ]);
}
