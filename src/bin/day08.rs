use std::collections::HashMap;

type JunctionBox = (i64, i64, i64);

fn parse(input: &str) -> Vec<JunctionBox> {
    input
        .lines()
        .map(|line| {
            let coords: Vec<i64> = line.split(',').map(|v| v.parse().unwrap()).collect();
            let [x, y, z]: [i64; 3] = coords.try_into().unwrap();
            (x, y, z)
        })
        .collect()
}

fn get_junction_box_pairs(boxes: &[JunctionBox]) -> Vec<(JunctionBox, JunctionBox)> {
    fn distance(a: JunctionBox, b: JunctionBox) -> i64 {
        let dx = b.0 - a.0;
        let dy = b.1 - a.1;
        let dz = b.2 - a.2;

        (dx * dx + dy * dy + dz * dz).isqrt()
    }

    let pairs: Vec<_> = (0..boxes.len())
        .flat_map(|i| (i + 1..boxes.len()).map(move |j| (&boxes[i], &boxes[j])))
        .collect();

    let mut distances = vec![];
    for pair in pairs {
        let (&a, &b) = pair;
        distances.push((distance(a, b), (a, b)));
    }
    distances.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    distances.iter().map(|dist| dist.1).collect()
}

fn create_circuits(
    junction_box_pairs: &[(JunctionBox, JunctionBox)],
    num_boxes: usize,
    pairs_to_consider: Option<usize>,
) -> (Vec<Vec<JunctionBox>>, (JunctionBox, JunctionBox)) {
    let mut circuits: Vec<Vec<JunctionBox>> = vec![];
    let mut connected: HashMap<JunctionBox, usize> = HashMap::new();
    let mut last_processed = ((0, 0, 0), (0, 0, 0));

    for (a, b) in junction_box_pairs
        .iter()
        .take(pairs_to_consider.unwrap_or(junction_box_pairs.len()))
    {
        if connected.contains_key(a) && connected.contains_key(b) {
            // Both A and B are already connected...

            if connected[a] != connected[b] {
                // ...but in differnent circuits -> combine both circuits
                let idx_a = connected[a];
                let idx_b = connected[b];
                let circuit_b = circuits[idx_b].clone();
                for b in circuit_b {
                    circuits[idx_a].push(b);
                    connected.insert(b, idx_a);
                    circuits[idx_b].clear();
                }
            }
        } else if connected.contains_key(a) {
            // Connect box B with circuit that contains box A
            let idx_a = connected[a];
            circuits[idx_a].push(*b);
            connected.insert(*b, idx_a);
        } else if connected.contains_key(b) {
            // Connect box A with circuit that contains box B
            let idx_b = connected[b];
            circuits[idx_b].push(*a);
            connected.insert(*a, idx_b);
        } else {
            // Both A and B are not connected -> connect them in new circuit
            let v = vec![*a, *b];
            let idx = circuits.len();
            circuits.push(v);
            connected.insert(*a, idx);
            connected.insert(*b, idx);
        }

        last_processed = (*a, *b);

        if connected.len() == num_boxes {
            assert_eq!(circuits.iter().filter(|c| !c.is_empty()).count(), 1);
            break;
        }
    }

    (circuits, last_processed)
}

fn part1(boxes: &[JunctionBox], pairs_to_consider: usize) -> i64 {
    let (circuits, _) = create_circuits(
        &get_junction_box_pairs(boxes),
        boxes.len(),
        Some(pairs_to_consider),
    );

    let mut circuit_sizes: Vec<i64> = circuits
        .iter()
        .filter(|set| !set.is_empty())
        .map(|c| c.len() as i64)
        .collect();

    circuit_sizes.sort_unstable_by(|a, b| b.cmp(a));
    circuit_sizes.into_iter().take(3).product::<i64>()
}

fn part2(boxes: &[JunctionBox]) -> i64 {
    let (_, last_processed) = create_circuits(&get_junction_box_pairs(boxes), boxes.len(), None);

    last_processed.0.0 * last_processed.1.0
}

fn main() {
    let input = include_str!("../../input/day08.txt");
    let data = parse(input);

    println!("p1: {}", part1(&data, 1000));
    println!("p2: {}", part2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day08() {
        let data = parse(
            "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689",
        );

        assert_eq!(part1(&data, 10), 40);
        assert_eq!(part2(&data), 25272);
    }
}
