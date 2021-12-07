#[test]
fn part_1() {
    let input = std::fs::read_to_string("res/01/input.txt").unwrap();
    let mut nums = Vec::<usize>::new();

    for line in input.lines() {
        nums.push(line.parse().unwrap());
    }

    let mut count = 0usize;
    let mut last_num = std::usize::MAX;
    for num in nums {
        if num > last_num {
            count += 1;
        }
        last_num = num;
    }

    println!("{}", count);
}

#[test]
fn part_2() {
    let input = std::fs::read_to_string("res/01/input.txt").unwrap();
    let mut nums = Vec::<usize>::new();

    for line in input.lines() {
        nums.push(line.parse().unwrap());
    }

    let mut count = 0usize;
    let mut last_sum = nums[0] + nums[1] + nums[2];
    for i in 3..nums.len() {
        let current_sum = last_sum - nums[i-3] + nums[i];

        if current_sum > last_sum {
            count += 1;
        }

        last_sum = current_sum;
    }

    println!("{}", count);
}
