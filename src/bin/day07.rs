use std::collections::HashMap;

fn parse(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.bytes().collect()).collect()
}

fn print_line(line: &[u8]) {
    for ch in line.iter().map(|&ch| ch as char) {
        print!("{ch}");
    }
    println!();
}

// fn step(manifold: &[Vec<u8>], cur_line: usize, cur_idx: usize) -> Vec<usize> {
//     let mut result = vec![];

//     if cur_line + 1 == manifold.len() - 1 {
//         // reached end
//         return vec![cur_idx];
//     }

//     if manifold[cur_line + 1][cur_idx] == b'.' {
//         // continue
//         return vec![cur_idx];
//     }

//     if manifold[cur_line + 1][cur_idx] == b'^' {
//         // split
//         result.push(cur_idx - 1);
//         result.push(cur_idx + 1);
//     }

//     result
// }

fn find_splits(manifold: &[Vec<u8>]) -> Vec<Vec<usize>> {
    let mut result = vec![];

    for i in 0..manifold.len() {
        if i % 2 != 0 {
            result.push(vec![]);
            continue;
        }
        let mut splits_in_line = vec![];
        for j in 0..manifold[0].len() {
            if manifold[i][j] == b'^' {
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
    cache: &mut HashMap<(usize, usize), Vec<usize>>,
) -> Vec<usize> {
    if line + 1 == manifold.len() - 1 {
        return vec![pos];
    }

    if cache.contains_key(&(line, pos)) {
        // println!("result already in cache");
        let res = cache.get(&(line, pos)).unwrap().to_vec();
        // println!("result already in cache {:?}", res);
        return res;
    }

    let mut res = vec![];

    if splits[line + 1].contains(&pos) {
        // timelines.push(pos + 1);
        res.append(&mut dp(manifold, splits, line + 1, pos + 1, cache));
        // timelines.push(pos - 1);
        res.append(&mut dp(manifold, splits, line + 1, pos - 1, cache));
    } else {
        // timelines.push(pos);
        res.append(&mut dp(manifold, splits, line + 1, pos, cache));
    }

    cache.insert((line, pos), res.clone());

    res
}

fn part2(manifold: &[Vec<u8>]) -> u64 {
    let start = manifold[0].iter().position(|&ch| ch == b'S').unwrap();
    let splits = find_splits(&manifold);

    // let mut timelines = vec![start];

    // for i in 1..manifold.len() {
    //     let mut new_timelines: Vec<usize> = vec![];

    //     if splits[i].is_empty() {
    //         continue;
    //     }

    //     for tl in &timelines {
    //         if splits[i].contains(&tl) {
    //             new_timelines.push(tl - 1);
    //             new_timelines.push(tl + 1);
    //         } else {
    //             new_timelines.push(*tl);
    //         }
    //     }
    //     timelines = new_timelines;
    // }

    // let mut timelines = vec![start];

    let mut cache: HashMap<(usize, usize), Vec<usize>> = HashMap::new();

    let result = dp(manifold, &splits, 0, start, &mut cache);

    result.len() as u64
}

// fn part2(manifold: &[Vec<u8>]) -> u64 {
//     let start = manifold[0].iter().position(|&ch| ch == b'S').unwrap();

//     let mut timelines = vec![start];

//     for (i, line) in manifold.iter().skip(1).enumerate() {
//         println!("{i}");
//         let mut new_timeslines = vec![];

//         if line.iter().all(|&b| b == b'.') {
//             continue;
//         }

//         for pos in timelines {
//             let mut res = step(&manifold, i, pos);
//             new_timeslines.append(&mut res);
//         }

//         timelines = new_timeslines;
//     }

//     timelines.len() as u64
// }

fn run(manifold: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let mut result: Vec<Vec<u8>> = vec![];
    result.push(manifold.iter().next().unwrap().to_vec());

    for (i, line) in manifold.iter().skip(1).enumerate() {
        let mut new_line = line.clone();

        for j in 0..line.len() {
            if line[j] == b'|' {
                new_line[j] = b'|';
            } else if line[j] == b'^' {
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

fn part1(manifold: &Vec<Vec<u8>>) -> u64 {
    let manifold = run(&manifold);

    let mut res = 0;
    for w in manifold.windows(2) {
        for j in 0..w[0].len() {
            if w[0][j] == b'|' && w[1][j] == b'^' {
                res += 1;
            }
        }
    }
    res
}

fn main() {
    let input = include_str!("../../input/day07.txt");
    let data = parse(&input);

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
