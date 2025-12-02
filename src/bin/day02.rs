fn parse(input: &str) -> Vec<(i64, i64)> {
    input
        .trim()
        .split(",")
        .map(|slice| {
            let (from, to) = slice.split_once("-").unwrap();
            (from.parse::<i64>().unwrap(), to.parse::<i64>().unwrap())
        })
        .collect()
}

fn is_invalid_id_part1(id: i64) -> bool {
    let s = id.to_string();
    let (a, b) = s.split_at(s.len() / 2);
    a == b
}

fn is_invalid_id_part2(id: i64) -> bool {
    let s = id.to_string();
    let s = s.as_bytes();
    let len = s.len();

    for j in 1..=(s.len() / 2) {
        let sub = &s[0..j];

        if !len.is_multiple_of(j) {
            continue;
        }

        if len / j < 2 {
            continue;
        }

        if s.chunks(j).all(|c| c == sub) {
            return true;
        }
    }

    false
}

fn part1(ranges: &[(i64, i64)]) -> i64 {
    ranges.iter().fold(0, |acc, &(from, to)| {
        acc + (from..=to)
            .filter(|&val| is_invalid_id_part1(val))
            .sum::<i64>()
    })
}

fn part2(ranges: &[(i64, i64)]) -> i64 {
    ranges.iter().fold(0, |acc, &(from, to)| {
        acc + (from..=to)
            .filter(|&val| is_invalid_id_part2(val))
            .sum::<i64>()
    })
}

fn main() {
    let input = include_str!("../../input/day02.txt");
    let data = parse(input);

    println!("p1: {}", part1(&data));
    println!("p2: {}", part2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day02() {
        let data = parse(
            "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
        );

        assert_eq!(part1(&data), 1227775554);
        assert_eq!(part2(&data), 4174379265);
    }
}
