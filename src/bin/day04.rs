fn parse(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|l| {
            l.bytes()
                .map(|c| match c {
                    b'@' => true,
                    b'.' => false,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

const ADJACENT_DIRS: [(isize, isize); 8] = [
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
];

fn count_adjacent_rolls(pos: (usize, usize), grid: &[Vec<bool>]) -> u64 {
    fn in_grid(pos: (isize, isize), grid: &[Vec<bool>]) -> bool {
        pos.0 >= 0 && pos.0 < grid.len() as isize && pos.1 >= 0 && pos.1 < grid[0].len() as isize
    }

    let mut res = 0;

    for d in ADJACENT_DIRS {
        let new_pos = (pos.0 as isize + d.0, pos.1 as isize + d.1);
        if in_grid(new_pos, grid) && grid[new_pos.0 as usize][new_pos.1 as usize] {
            res += 1;
        }
    }
    res
}

fn part1(grid: &[Vec<bool>]) -> u64 {
    let mut res = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] && count_adjacent_rolls((i, j), grid) < 4 {
                res += 1;
            }
        }
    }
    res
}

fn part2(mut grid: Vec<Vec<bool>>) -> u64 {
    let mut res = 0;

    loop {
        let mut removed = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] && count_adjacent_rolls((i, j), &grid) < 4 {
                    removed += 1;
                    grid[i][j] = false;
                }
            }
        }
        if removed > 0 {
            res += removed;
        } else {
            break;
        }
    }

    res
}

fn main() {
    let input = include_str!("../../input/day04.txt");
    let data = parse(input);

    println!("p1: {}", part1(&data));
    println!("p2: {}", part2(data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day04() {
        let data = parse(
            "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.",
        );

        assert_eq!(part1(&data), 13);
        assert_eq!(part2(data), 43);
    }
}
