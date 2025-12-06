fn part1(input: &str) -> u64 {
    let worksheet: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();

    let operations = worksheet.last().unwrap();
    let mut result: Vec<u64> = worksheet[0].iter().map(|v| v.parse().unwrap()).collect();

    for line in worksheet.iter().skip(1).take(worksheet.len() - 2) {
        for (j, &val) in line.iter().enumerate() {
            let val: u64 = val.parse().unwrap();

            result[j] = match operations[j] {
                "+" => result[j] + val,
                "*" => result[j] * val,
                _ => unimplemented!("operation not supported"),
            }
        }
    }

    result.iter().sum()
}

fn part2(input: &str) -> u64 {
    let worksheet: Vec<Vec<_>> = input.lines().map(|line| line.bytes().collect()).collect();

    let operations: Vec<u8> = worksheet
        .last()
        .unwrap()
        .iter()
        .copied()
        .filter(|&b| !b.is_ascii_whitespace())
        .collect();

    let mut problems: Vec<Vec<String>> = vec![];
    let mut problem: Vec<String> = vec![];

    for j in 0..worksheet[0].len() {
        let val: String = (0..worksheet.len() - 1)
            .filter_map(|i| {
                let ch = worksheet[i][j] as char;
                (ch != ' ').then_some(ch)
            })
            .collect();
        if val.is_empty() {
            problems.push(std::mem::take(&mut problem));
        } else {
            problem.push(val);
        }
    }
    problems.push(problem);

    assert_eq!(problems.len(), operations.len());

    operations
        .iter()
        .zip(problems.iter())
        .map(|(op, problem)| {
            let nums: Vec<u64> = problem.iter().map(|s| s.parse().unwrap()).collect();

            match op {
                b'+' => nums.iter().sum::<u64>(),
                b'*' => nums.iter().product::<u64>(),
                _ => unimplemented!("operation not supported"),
            }
        })
        .sum()
}

fn main() {
    let input = include_str!("../../input/day06.txt");

    println!("p1: {}", part1(input));
    println!("p2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day06() {
        let input = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

        assert_eq!(part1(&input), 4277556);
        assert_eq!(part2(&input), 3263827);
    }
}
