use std::collections::{HashMap, HashSet};

pub fn generator(input: &str) -> &str {
    input
}

pub fn part_1(input: &str) -> u64 {
    let mut current_rays = HashSet::new();

    let mut splits = 0;

    current_rays.insert(
        input
            .lines()
            .next()
            .expect("invalid input! (no 1st line)")
            .chars()
            .enumerate()
            .find(|&(_idx, c)| c == 'S')
            .expect("invalid input! (no S)")
            .0,
    );

    for line in input.lines() {
        if !line.contains('^') {
            continue;
        }
        for (idx, c) in line.chars().enumerate() {
            if c != '^' {
                continue;
            }
            if current_rays.remove(&idx) {
                current_rays.insert(idx - 1);
                current_rays.insert(idx + 1);
                splits += 1;
            }
        }
    }

    splits
}

pub fn part_2(input: &str) -> u64 {
    let mut current_rays = HashMap::new();

    current_rays.insert(
        input
            .lines()
            .next()
            .expect("invalid input! (no 1st line)")
            .chars()
            .enumerate()
            .find(|&(_idx, c)| c == 'S')
            .expect("invalid input! (no S)")
            .0,
        1u64,
    );

    for line in input.lines() {
        if !line.contains('^') {
            continue;
        }
        for (idx, c) in line.chars().enumerate() {
            if c != '^' {
                continue;
            }
            if current_rays.contains_key(&idx) {
                let parent_tls = current_rays.remove(&idx).expect("incalid .contains_key!");
                current_rays.insert(
                    idx - 1,
                    parent_tls + current_rays.get(&(idx - 1)).unwrap_or(&0),
                );
                current_rays.insert(
                    idx + 1,
                    parent_tls + current_rays.get(&(idx + 1)).unwrap_or(&0),
                );
            }
        }
    }

    current_rays.values().sum()
}
