use std::collections::HashMap;

pub fn generator(input: &str) -> &str {
    input
}

fn calc_pairs(input: &str) -> HashMap<(usize, usize), f64> {
    let mut pairs = HashMap::new();

    for (idx1, line1) in input.lines().enumerate() {
        for (idx2, line2) in input.lines().enumerate().skip(idx1 + 1) {
            let mut coords1 = line1.split(',');
            let x1 = coords1
                .next()
                .expect("mangled input!")
                .parse::<f64>()
                .expect("mangled input!");
            let y1 = coords1
                .next()
                .expect("mangled input!")
                .parse::<f64>()
                .expect("mangled input!");
            let z1 = coords1
                .next()
                .expect("mangled input!")
                .parse::<f64>()
                .expect("mangled input!");
            let mut coords2 = line2.split(',');
            let x2 = coords2
                .next()
                .expect("mangled input!")
                .parse::<f64>()
                .expect("mangled input!");
            let y2 = coords2
                .next()
                .expect("mangled input!")
                .parse::<f64>()
                .expect("mangled input!");
            let z2 = coords2
                .next()
                .expect("mangled input!")
                .parse::<f64>()
                .expect("mangled input!");
            pairs.insert(
                (idx1, idx2),
                ((x1 - x2).powf(2.0) + (y1 - y2).powf(2.0) + (z1 - z2).powf(2.0)).sqrt(),
            );
        }
    }

    pairs
}

pub fn part_1(input: &str) -> u64 {
    let mut boxes = vec![None; input.lines().count()];
    let mut circuits: HashMap<usize, u64> = HashMap::new();

    let mut pairs = calc_pairs(input)
        .into_iter()
        .collect::<Vec<((usize, usize), f64)>>();
    pairs.sort_by(|(_k1, a), (_k2, b)| a.total_cmp(b));

    for i in 0..1000usize {
        if let Some(circuit) = boxes[pairs[i].0.0] {
            if let Some(circuit2) = boxes[pairs[i].0.1] {
                if circuit != circuit2 {
                    for (b, _c) in boxes
                        .clone()
                        .iter()
                        .enumerate()
                        .filter(|(_b, c)| **c == Some(circuit2))
                    {
                        boxes[b] = Some(circuit);
                    }
                    circuits.insert(
                        circuit,
                        *circuits.get(&circuit).expect("mangled var!")
                            + *circuits.get(&circuit2).expect("mangled var!"),
                    );
                    circuits.remove(&circuit2);
                }
            } else {
                boxes[pairs[i].0.1] = Some(circuit);
                circuits.insert(circuit, *circuits.get(&circuit).expect("mangled var!") + 1);
            }
        } else if let Some(circuit) = boxes[pairs[i].0.1] {
            boxes[pairs[i].0.0] = Some(circuit);
            circuits.insert(circuit, *circuits.get(&circuit).expect("mangled var!") + 1);
        } else {
            boxes[pairs[i].0.0] = Some(i);
            boxes[pairs[i].0.1] = Some(i);
            circuits.insert(i, 2);
        }
    }

    let mut circuits = circuits.iter().collect::<Vec<(&usize, &u64)>>();
    circuits.sort_by(|(_k1, a), (_k2, b)| b.cmp(a));

    circuits[0].1 * circuits[1].1 * circuits[2].1
}

pub fn part_2(input: &str) -> u64 {
    let mut boxes = vec![None; input.lines().count()];
    let mut circuits: HashMap<usize, u64> = HashMap::new();

    let mut pairs = calc_pairs(input)
        .into_iter()
        .collect::<Vec<((usize, usize), f64)>>();
    pairs.sort_by(|(_k1, a), (_k2, b)| a.total_cmp(b));

    for i in 0..pairs.len() {
        if let Some(circuit) = boxes[pairs[i].0.0] {
            if let Some(circuit2) = boxes[pairs[i].0.1] {
                if circuit != circuit2 {
                    for (b, _c) in boxes
                        .clone()
                        .iter()
                        .enumerate()
                        .filter(|(_b, c)| **c == Some(circuit2))
                    {
                        boxes[b] = Some(circuit);
                    }
                    circuits.insert(
                        circuit,
                        *circuits.get(&circuit).expect("mangled var!")
                            + *circuits.get(&circuit2).expect("mangled var!"),
                    );
                    circuits.remove(&circuit2);
                }
            } else {
                boxes[pairs[i].0.1] = Some(circuit);
                circuits.insert(circuit, *circuits.get(&circuit).expect("mangled var!") + 1);
            }
        } else if let Some(circuit) = boxes[pairs[i].0.1] {
            boxes[pairs[i].0.0] = Some(circuit);
            circuits.insert(circuit, *circuits.get(&circuit).expect("mangled var!") + 1);
        } else {
            boxes[pairs[i].0.0] = Some(i);
            boxes[pairs[i].0.1] = Some(i);
            circuits.insert(i, 2);
        }

        if circuits.len() == 1
            && circuits.clone().into_values().next().unwrap_or_default() == boxes.len() as u64
        {
            return input
                .lines()
                .nth(pairs[i].0.0)
                .expect("mangled input!")
                .split_once(',')
                .unwrap_or_default()
                .0
                .parse::<u64>()
                .unwrap_or_default()
                * input
                    .lines()
                    .nth(pairs[i].0.1)
                    .expect("mangled input!")
                    .split_once(',')
                    .unwrap_or_default()
                    .0
                    .parse::<u64>()
                    .unwrap_or_default();
        }
    }

    panic!("circuit never connected!")
}
