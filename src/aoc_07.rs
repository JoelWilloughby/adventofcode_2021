fn read_it(filename: &str) -> Vec<usize>{
    std::fs::read_to_string(filename).unwrap().trim().split(",").map(|x| x.parse().unwrap()).collect()
}

fn drive(filename: &str) {
    let mut nums = read_it(filename);
    nums.sort();

    // Just get the median?
    let target = nums[nums.len()/2];
    let val = nums.iter().fold(0, |acc, x| acc + (target as isize - *x as isize).abs());
    println!("{}, {}", target, val);
}

fn d(x: usize) -> usize {
    (x * (x + 1)) / 2
}

fn drive_2(filename: &str) {
    let mut nums = read_it(filename);
    nums.sort();

    // Brewt forrrceee
    let mut best = std::usize::MAX;
    let mut best_val = 0;
    for i in nums[0]..=nums[nums.len()-1] {
        let next = nums.iter().fold(0, |acc, x| acc + d((*x as isize - i as isize).abs() as usize));
        if next < best {
            best = next;
            best_val = i;
        } else {
            // Know its only downhill from here...
            break;
        }
    }

    println!("{}, {}", best_val, best);
}

#[test]
fn part_0() {
    drive("res/07/sample.txt");
    drive_2("res/07/sample.txt");
}

#[test]
fn part_1() {
    drive("res/07/input.txt");
}

#[test]
fn part_2() {
    drive_2("res/07/input.txt");
}
