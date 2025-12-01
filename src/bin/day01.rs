const START: i32 = 50;
const DIAL_TICKS: i32 = 100;

fn parse(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            let dir = chars.next().unwrap();
            let val = chars.as_str().parse::<i32>().unwrap();

            match dir {
                'R' => val,
                'L' => -val,
                _ => unreachable!(),
            }
        })
        .collect()
}

fn part1(rotations: &[i32]) -> u32 {
    let mut res = 0;
    let mut cur_pos = START;
    for &r in rotations {
        cur_pos = (cur_pos + r).rem_euclid(DIAL_TICKS);
        if cur_pos == 0 {
            res += 1;
        }
    }
    res
}

fn part2(rotations: &[i32]) -> i32 {
    let mut res = 0;
    let mut cur_pos = START;
    for &r in rotations {
        let new_pos = cur_pos + r;

        if new_pos == 0 && cur_pos != 0 {
            res += 1;
        } else if new_pos != 0 && cur_pos == 0 {
            res += new_pos.abs().div_euclid(DIAL_TICKS);
        } else if new_pos != 0 && cur_pos != 0 && new_pos.signum() != cur_pos.signum() {
            res += 1;
            res += new_pos.abs().div_euclid(DIAL_TICKS);
        } else if new_pos.signum() == cur_pos.signum() {
            res += new_pos.abs().div_euclid(DIAL_TICKS);
        }

        cur_pos = new_pos.rem_euclid(DIAL_TICKS);
    }
    res
}

fn main() {
    let input = include_str!("../../input/day01.txt");
    let data = parse(input);

    println!("p1: {}", part1(&data));
    println!("p2: {}", part2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day01() {
        let data = parse(
            "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82",
        );

        assert_eq!(part1(&data), 3);
        assert_eq!(part2(&data), 6);
    }
}
