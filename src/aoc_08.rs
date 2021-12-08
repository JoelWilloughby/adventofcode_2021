use std::collections::{HashMap, HashSet};
use lazy_static::lazy_static;

struct Solver {
    decoder: HashMap<String, usize>,
}

//  aa
// b  c
//  dd
// e  f
//  gg

lazy_static! {
    static ref ONE : HashSet<char>    = [          'c',           'f'     ].iter().cloned().collect();

    static ref SEVEN : HashSet<char>  = ['a',      'c',           'f'     ].iter().cloned().collect();

    static ref FOUR : HashSet<char>   = [     'b', 'c', 'd',      'f'     ].iter().cloned().collect();

    static ref TWO : HashSet<char>    = ['a',      'c', 'd', 'e',      'g'].iter().cloned().collect();
    static ref THREE : HashSet<char>  = ['a',      'c', 'd',      'f', 'g'].iter().cloned().collect();
    static ref FIVE : HashSet<char>   = ['a', 'b',      'd',      'f', 'g'].iter().cloned().collect();

    static ref TWO_X : HashSet<char>  = ['a',      'c', 'd', 'e', 'f', 'g'].iter().cloned().collect();
    static ref FIVE_X : HashSet<char> = ['a', 'b', 'c', 'd',      'f', 'g'].iter().cloned().collect();
    static ref INT_5 : HashSet<char>  = ['a',           'd',           'g'].iter().cloned().collect();

    static ref SIX : HashSet<char>    = ['a', 'b',      'd', 'e', 'f', 'g'].iter().cloned().collect();
    static ref NINE : HashSet<char>   = ['a', 'b', 'c', 'd',      'f', 'g'].iter().cloned().collect();
    static ref ZERO : HashSet<char>   = ['a', 'b', 'c',      'e', 'f', 'g'].iter().cloned().collect();
    static ref INT_6 : HashSet<char>  = ['a', 'b',                'f', 'g'].iter().cloned().collect();

    static ref EIGHT : HashSet<char>  = ['a', 'b', 'c', 'd', 'e', 'f', 'g'].iter().cloned().collect();
}

impl Solver {
    fn from_vec(input: &Vec<String>) -> Self {
        let mut candidates = HashMap::new();
        let char_set : HashSet<char> = ['a', 'b', 'c', 'd', 'e', 'f', 'g'].iter().cloned().collect();
        ['a', 'b', 'c', 'd', 'e', 'f', 'g'].iter().cloned().map(|c| candidates.insert(c, char_set.clone())).count();

        // Filter on known things
        for s in input.iter() {
            if s.len() > 4 {continue;}
            let filter_set = match s.len() {
                2 => ONE.clone(),
                3 => SEVEN.clone(),
                4 => FOUR.clone(),
                _ => EIGHT.clone()
            };

            s.chars().map(|c| {
                let i = candidates[&c].intersection(&filter_set).cloned().collect(); candidates.insert(c, i)
            }).count();
            EIGHT.difference(&s.chars().collect()).map(|c| {
                let i = candidates[&c].difference(&filter_set).cloned().collect(); candidates.insert(*c, i)
            }).count();
        }

        for s in input.iter() {
            match s.len() {
                5 => {
                    // Gather keys for each necessary thing
                    ['d', 'g'].iter().map(|filter_c| {
                        let keys : HashSet<char> = s.chars().filter(|c| candidates[c].contains(&filter_c)).collect();
                        EIGHT.difference(&keys).map(|c| {
                            candidates.get_mut(&c).unwrap().remove(&filter_c);
                        }).count();
                    }).count();
                },
                _ => {}
            }
        }

        let definite_filter : Vec<(char, char)> = candidates.iter().filter(|(_, cc)| cc.len() == 1).map(|(c, cc)| (*c, *cc.iter().next().unwrap())).collect();
        definite_filter.iter().map(|(c, cc)| {
            EIGHT.difference(&[*c].iter().cloned().collect()).map(|a| {
                candidates.get_mut(&a).unwrap().remove(cc);
            }).count();
        }).count();

        for s in input.iter() {
            match s.len() {
                5 => {
                    let possible = s.chars().fold(HashSet::new(), |s, c| s.union(&candidates[&c]).cloned().collect());
                    s.chars().map(|c| {
                        candidates.get_mut(&c).unwrap().remove(&if possible == *FIVE_X {'c'} else if possible == *TWO_X {'f'} else {'k'});
                    }).count();
                },
                _ => {}
            }
        }

        let rev_map : HashMap<char, char> = candidates.iter().map(|(key, val)| (*val.iter().next().unwrap(), *key)).collect();
        let mut decoder = HashMap::new();
        [ZERO.clone(), ONE.clone(), TWO.clone(), THREE.clone(), FOUR.clone(), FIVE.clone(), SIX.clone(), SEVEN.clone(), EIGHT.clone(), NINE.clone()].iter().enumerate().map(|(i, s)| {
            let mut st : Vec<char> = s.iter().map(|x| rev_map[x]).collect();
            st.sort();
            decoder.insert(st.into_iter().collect(), i);
        }).count();

        Self {
            decoder
        }
    }

    fn eval(&self, s: &str) -> usize {
        let mut sorted : Vec<char> = s.chars().collect();
        sorted.sort();

        let sorted: String = sorted.into_iter().collect();

        self.decoder[&sorted]
    }
}

fn read_it(filename: &str) -> Vec<(Vec<String>, Vec<String>)> {
    let input = std::fs::read_to_string(filename).unwrap();
    input.lines().map(|line| {
        let mut groups = line.split("|");
        (groups.next().unwrap().split_whitespace().map(|s| s.to_owned()).collect(), groups.next().unwrap().split_whitespace().map(|s| s.to_owned()).collect())
    }).collect()
}

fn drive(filename: &str) {
    let stuff = read_it(filename);
    println!("{:?}", stuff);

    let val = stuff.iter().fold(0, |acc, (_, ts)| acc + ts.iter().filter(|t| [2, 3, 4, 7].contains(&t.len())).count());
    println!("{}", val);
}

fn drive_2(filename: &str) {
    let stuff = read_it(filename);
    let mut big_acc = 0;
    for (ss, targets) in stuff {
        let solver = Solver::from_vec(&ss);

        let acc = targets.iter().fold(0, |acc, t| 10 * acc + solver.eval(t));
        println!("{}", acc);
        big_acc += acc;
    }

    println!("{}", big_acc);
}

#[test]
fn part_0() {
    drive("res/08/sample.txt");
    drive_2("res/08/sample.txt");
}

#[test]
fn part_1() {
    drive("res/08/input.txt");
}

#[test]
fn part_2() {
    drive_2("res/08/input.txt");
}
