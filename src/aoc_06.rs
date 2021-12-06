// Represents an "old" fish
fn g(n: usize) -> usize {
    unsafe {
        static mut MEMO : [(bool, usize); 300] = [(false,0) ; 300];
        if n < 7 {
            return 1;
        }

        if let (true, res) = MEMO[n] {
            res
        } else {
            // Old fish plus a newly spawned fish
            let val = g(n - 7) + f(n - 7);
            MEMO[n] = (true, val);
            // println!("g({}) = {}", n, val);
            val
        }
    }
}

// Represents a "new" fish, takes 2 longer than an old fish
fn f(n: usize) -> usize {
    if n < 9 {
        return 1;
    }

    g(n - 2)
}

fn fish(n: usize, init_size: usize) -> usize {
    g(n + (6 - init_size))
}

fn read_it(filename: &str) -> [usize; 10] {
    let mut vals: [usize; 10] = [0; 10];
    std::fs::read_to_string(filename).unwrap()
        .trim()
        .split(",")
        .filter_map(|s| s.parse().ok())
        .map(|x: usize| vals[x] += 1).count();
    vals
}

fn drive(filename: &str, n: usize) {
    let mut acc = 0usize;
    let in_vals = read_it(filename);

    for i in 0..10 {
        if in_vals[i] == 0 {
            continue;
        }
        acc += in_vals[i] * fish(n, i);
    }
    println!("days: {} -- {}", n, acc);
}

#[test]
fn part_0() {
    for days in 0..20 {
        drive("res/06/sample.txt", days + 1);
    }
}

#[test]
fn part_1() {
    drive("res/06/input.txt", 80);
}

#[test]
fn part_2() {
    drive("res/06/input.txt", 256);
}
