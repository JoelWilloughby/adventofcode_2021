use std::collections::BTreeSet;


fn d(x: isize) -> isize {
    (x.abs() * (x.abs()+1)) / 2
}

fn dumb((x0, x1): (isize, isize), (y0, y1): (isize, isize)) -> BTreeSet<(isize, isize)> {
    let mut acc = BTreeSet::new();

    for ox in 1..=x1 {
        let mut x = ox;
        // Find numbes of steps that will lead you to the right place.
        let mut xacc = 0;
        let mut steps = 0;
        // What is a good bound for this?
        while xacc <= x1  && steps < 1000 {
            if xacc >= x0 {
                // y <= 0
                for iy in 0.. {
                    let yval = -(d(iy + steps - 1) - d(iy) + iy);
                    if yval < y0 {
                        break;
                    }
                    if yval <= y1 {
                        acc.insert((ox, -iy));
                    }
                }
                // y > 0
                for y in 1.. {
                    let yval = d(y) - d(y - (steps - 1));
                    if yval > y1 {
                        break;
                    }
                    if yval >= y0 {
                        acc.insert((ox, y));
                    }
                }
            } else if x == 0 {
                break;
            }

            xacc += x;
            steps += 1;
            x -= x.signum();
        }
    }

    acc
}

#[test]
fn part_0() {
    let count = dumb((20, 30), (-10, -5));
    println!("{}", count.iter().fold(std::isize::MIN, |acc, (_, y)| std::cmp::max(acc, *y)));
    println!("{}", count.len());
    println!("{:?}", count);
}

#[test]
fn part_1() {
    let count = dumb((253, 280), (-73, -46));
    println!("{}", count.iter().fold(std::isize::MIN, |acc, (_, y)| std::cmp::max(acc, *y)));
    println!("{}", count.len());
}
