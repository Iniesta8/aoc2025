use std::cmp::max;
use std::ops::RangeInclusive;

fn parse(input: &str) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    let mut fresh_ingredients = vec![];
    let mut available_ingredients = vec![];

    let (fresh, available) = input.trim().split_once("\n\n").unwrap();

    for range in fresh.lines() {
        let (start, end) = range.split_once('-').unwrap();
        let start = start.parse::<u64>().unwrap();
        let end = end.parse::<u64>().unwrap();

        fresh_ingredients.push(start..=end);
    }

    for id in available.lines() {
        available_ingredients.push(id.parse::<u64>().unwrap());
    }

    (fresh_ingredients, available_ingredients)
}

fn part1(data: &(Vec<RangeInclusive<u64>>, Vec<u64>)) -> u64 {
    let (fresh, available) = data;
    let mut res = 0;

    for id in available {
        for range in fresh {
            if range.contains(id) {
                res += 1;
                break;
            }
        }
    }

    res
}

fn part2(data: Vec<RangeInclusive<u64>>) -> u64 {
    let mut ranges = data;
    ranges.sort_by_key(|r| *r.start());

    let mut merged_ranges: Vec<RangeInclusive<u64>> = vec![];

    for next_range in ranges {
        let (range_start, range_end) = next_range.into_inner();
        if let Some(last) = merged_ranges.last_mut() {
            let (last_start, last_end) = last.clone().into_inner();

            if range_start <= last_end {
                let new_end = max(last_end, range_end);
                *last = last_start..=new_end;
                continue;
            }
        }
        merged_ranges.push(range_start..=range_end);
    }

    merged_ranges
        .iter()
        .map(|r| r.end() - r.start() + 1)
        .sum::<u64>()
}

fn main() {
    let input = include_str!("../../input/day05.txt");
    let data = parse(input);

    println!("p1: {}", part1(&data));
    println!("p2: {}", part2(data.0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day05() {
        let data = parse(
            "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32",
        );

        assert_eq!(part1(&data), 3);
        assert_eq!(part2(data.0), 14);
    }
}
