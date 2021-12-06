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

// Represent a "new" fish
fn f(n: usize) -> usize {
    unsafe {
        static mut MEMO : [(bool, usize); 300] = [(false,0) ; 300];
        if n < 9 {
            return 1;
        }

        if let (true, res) = MEMO[n] {
            res
        } else {
            let val = g(n - 2);
            MEMO[n] = (true, val);
            val
        }
    }
}

fn fish(n: usize, init_size: usize) -> usize {
    g(n + (6 - init_size))
}

#[test]
fn part_0() {
    for days in 0..20 {
        let mut acc = 0usize;
        for val in [3, 4, 3, 1, 2] {
            acc += fish(days + 1, val);
        }
        println!("days: {} -- {}", days + 1, acc);
    }
}

fn drive(n: usize) {
    let mut acc = 0usize;
    // for val in [3, 4, 3, 1, 2] {
    for val in [4,3,4,5,2,1,1,5,5,3,3,1,5,1,4,2,2,3,1,5,1,4,1,2,3,4,1,4,1,5,2,1,1,3,3,5,1,1,1,1,4,5,1,2,1,2,1,1,1,5,3,3,1,1,1,1,2,4,2,1,2,3,2,5,3,5,3,1,5,4,5,4,4,4,1,1,2,1,3,1,1,4,2,1,2,1,2,5,4,2,4,2,2,4,2,2,5,1,2,1,2,1,4,4,4,3,2,1,2,4,3,5,1,1,3,4,2,3,3,5,3,1,4,1,1,1,1,2,3,2,1,1,5,5,1,5,2,1,4,4,4,3,2,2,1,2,1,5,1,4,4,1,1,4,1,4,2,4,3,1,4,1,4,2,1,5,1,1,1,3,2,4,1,1,4,1,4,3,1,5,3,3,3,4,1,1,3,1,3,4,1,4,5,1,4,1,2,2,1,3,3,5,3,2,5,1,1,5,1,5,1,4,4,3,1,5,5,2,2,4,1,1,2,1,2,1,4,3,5,5,2,3,4,1,4,2,4,4,1,4,1,1,4,2,4,1,2,1,1,1,1,1,1,3,1,3,3,1,1,1,1,3,2,3,5,4,2,4,3,1,5,3,1,1,1,2,1,4,4,5,1,5,1,1,1,2,2,4,1,4,5,2,4,5,2,2,2,5,4,4] {
        acc += fish(n, val);
    }
    println!("days: {} -- {}", n, acc);
}

#[test]
fn part_1() {
    drive(80);
}

#[test]
fn part_2() {
    drive(256);
}
