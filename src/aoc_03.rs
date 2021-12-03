#[derive(Clone)]
struct Nums {
    nums: Vec<usize>,
    length: usize,
}

fn read_it(filename: &str) -> Nums {
    let input = std::fs::read_to_string(filename).unwrap();
    let mut nums = Vec::<usize>::new();
    let mut length = 0usize;
    for line in input.lines() {
        length = line.len();
        nums.push(usize::from_str_radix(line, 2).unwrap());
    }

    Nums { nums, length }
}

fn count_it(nums: &Vec<usize>, pos: usize) -> (usize, usize) {
    let count = nums.iter().fold(0, |acc, &x| if x & (1 << pos) != 0 {acc + 1} else {acc});

    if 2 * count >= nums.len() {
        (0, 1 << pos)
    } else {
        (1 << pos, 0)
    }
}

fn epsilon_delta(nums: &Nums) -> (usize, usize) {
    let mut eps = 0usize;
    let mut mnums = nums.nums.clone();
    while mnums.iter().any(|&x| x != 0) {
        eps <<= 1;
        let mut count = 0;
        for num in mnums.iter_mut() {
            if *num & 1 == 1 {
                count += 1;
            }
            *num = *num >> 1;
        }

        if 2 * count >= nums.nums.len() {
            eps += 1;
        }
    }

    let mut rev_eps = eps;
    eps = 0;
    for _ in 0..nums.length {
        eps <<= 1;
        eps += rev_eps & 1;
        rev_eps >>= 1;
    }

    (eps, (!eps & ((1 << nums.length) - 1)))
}

fn oxy_co2(nums: &Nums) -> (usize, usize) {
    let mut mnums = nums.clone();
    while mnums.nums.len() > 1 {
        mnums.length -= 1;
        let (_, target) = count_it(&mnums.nums, mnums.length);
        mnums.nums = mnums.nums.into_iter().filter(|x| x & (1 << mnums.length) == target).collect();
    }

    let oxy = mnums.nums[0];

    mnums = nums.clone();
    while mnums.nums.len() > 1 {
        mnums.length -= 1;
        let (target, _) = count_it(&mnums.nums, mnums.length);
        mnums.nums = mnums.nums.into_iter().filter(|x| x & (1 << mnums.length) == target).collect();
    }

    (oxy, mnums.nums[0])
}

#[test]
fn part_1() {
    let nums = read_it("res/03/input.txt");
    let (eps, del) = epsilon_delta(&nums);
    println!("e: {:b}, d: {:b}, {}", eps, del, eps * del);
}

#[test]
fn part_2() {
    let nums = read_it("res/03/input.txt");
    let (oxy, co2) = oxy_co2(&nums);
    println!("oxy: {}, co2: {}, {}", oxy, co2, oxy * co2);
}
