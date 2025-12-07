use std::collections::HashMap;

fn parse(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.bytes().collect()).collect()
}

fn _print_manifold(manifold: &[Vec<u8>]) {
    fn print_line(line: &[u8]) {
        for ch in line.iter().map(|&ch| ch as char) {
            print!("{ch}");
        }
        println!();
    }
    for line in manifold {
        print_line(line);
    }
}

fn run(manifold: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let mut result: Vec<Vec<u8>> = vec![];
    result.push(manifold.iter().next().unwrap().to_vec());

    for (i, line) in manifold.iter().skip(1).enumerate() {
        let mut new_line = line.clone();

        for j in 0..line.len() {
            if line[j] == b'^' {
                new_line[j - 1] = b'|';
                new_line[j + 1] = b'|';
            } else if result[(i + 1) - 1][j] == b'|' || result[(i + 1) - 1][j] == b'S' {
                new_line[j] = b'|';
            }
        }
        result.push(new_line);
    }
    result
}

fn find_splits(manifold: &[Vec<u8>]) -> Vec<Vec<usize>> {
    let mut result = vec![];

    for (i, line) in manifold.iter().enumerate() {
        if i % 2 != 0 {
            result.push(vec![]);
            continue;
        }
        let mut splits_in_line = vec![];
        for (j, &b) in line.iter().enumerate() {
            if b == b'^' {
                splits_in_line.push(j);
            }
        }
        result.push(splits_in_line);
    }
    result
}

fn dp(
    manifold: &[Vec<u8>],
    splits: &[Vec<usize>],
    line: usize,
    pos: usize,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if line + 1 == manifold.len() - 1 {
        return 1;
    }

    if let Some(res) = cache.get(&(line, pos)) {
        return *res;
    }

    let res = if splits[line + 1].contains(&pos) {
        dp(manifold, splits, line + 1, pos + 1, cache)
            + dp(manifold, splits, line + 1, pos - 1, cache)
    } else {
        dp(manifold, splits, line + 1, pos, cache)
    };

    cache.insert((line, pos), res);

    res
}

fn part1(manifold: &[Vec<u8>]) -> u64 {
    let manifold = run(manifold);

    if cfg!(test) {
        _print_manifold(&manifold);
    }

    let mut res = 0;
    for w in manifold.windows(2) {
        let [top, bottom] = w else { continue };
        for (&tb, &bb) in top.iter().zip(bottom) {
            if tb == b'|' && bb == b'^' {
                res += 1;
            }
        }
    }
    res
}

fn part2(manifold: &[Vec<u8>]) -> u64 {
    let start = manifold[0].iter().position(|&b| b == b'S').unwrap();
    let splits = find_splits(manifold);
    let mut cache = HashMap::new();

    dp(manifold, &splits, 0, start, &mut cache) as u64
}

fn main() {
    let input = include_str!("../../input/day07.txt");
    let data = parse(input);

    println!("p1: {}", part1(&data));
    println!("p2: {}", part2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day07() {
        let data = parse(
            "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............",
        );

        assert_eq!(part1(&data), 21);
        assert_eq!(part2(&data), 40);
    }
}
