fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| l.bytes().map(|c| c - b'0').collect())
        .collect()
}

fn get_largest_joltage(bank: &[u8], digits: usize) -> u64 {
    let mut stack: Vec<u8> = Vec::with_capacity(digits);
    let mut skip = bank.len() - digits;

    for &d in bank {
        while skip > 0 && !stack.is_empty() && stack.last().unwrap() < &d {
            stack.pop();
            skip -= 1;
        }
        stack.push(d);
    }

    stack[0..digits]
        .iter()
        .fold(0, |acc, &d| 10 * acc + d as u64)
}

fn part1(battery: &[Vec<u8>]) -> u64 {
    battery
        .iter()
        .map(|bank| get_largest_joltage(bank, 2))
        .sum()
}

fn part2(battery: &[Vec<u8>]) -> u64 {
    battery
        .iter()
        .map(|bank| get_largest_joltage(bank, 12))
        .sum()
}

fn main() {
    let input = include_str!("../../input/day03.txt");
    let data = parse(input);

    println!("p1: {}", part1(&data));
    println!("p2: {}", part2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day03() {
        let data = parse(
            "\
987654321111111
811111111111119
234234234234278
818181911112111",
        );

        assert_eq!(part1(&data), 357);
        assert_eq!(part2(&data), 3121910778619);
    }
}
