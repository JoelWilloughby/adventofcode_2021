enum Command {
    H(isize),
    D(isize)
}

fn read_it(filename: &str) -> Vec::<Command> {
    let input = std::fs::read_to_string(filename).unwrap();
    let mut acc = Vec::<Command>::new();
    for line in input.lines() {
        let parsed : Vec<&str> = line.split(" ").collect();
        let num = parsed[1].parse::<isize>().unwrap();
        match parsed[0] {
            "forward" => acc.push(Command::H(num)),
            "down" => acc.push(Command::D(num)),
            "up" => acc.push(Command::D(num * -1)),
            _ => {}
        }
    }

    acc
}

#[test]
fn part_1() {
    let commands = read_it("res/02/input.txt");
    let mut horizontal_pos = 0isize;
    let mut depth = 0isize;

    for command in commands {
        match command {
            Command::H(val) => horizontal_pos += val,
            Command::D(val) => depth += val,
        }
    }

    println!("D: {}, H: {}, Total: {}", depth, horizontal_pos, depth * horizontal_pos);
}

#[test]
fn part_2() {
    let commands = read_it("res/02/input.txt");
    let mut horizontal_pos = 0isize;
    let mut depth = 0isize;
    let mut aim = 0isize;

    for command in commands {
        match command {
            Command::H(val) => {
                horizontal_pos += val;
                depth += aim * val;
            },
            Command::D(val) => aim += val,
        }
    }

    println!("D: {}, H: {}, Total: {}", depth, horizontal_pos, depth * horizontal_pos);
}
