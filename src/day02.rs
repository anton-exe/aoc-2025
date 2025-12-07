use std::{cmp, collections::HashSet};

pub fn generator(input: &str) -> &str {
    input
}

pub fn part_1(input: &str) -> u64 {
    let mut total = 0;

    for (start, end) in input
        .split(',')
        .map(|range| {
            let split = range.split('-').collect::<Vec<&str>>();
            (
                split[0].parse().unwrap_or_default(),
                split[1].parse().unwrap_or_default(),
            )
        })
        .collect::<Vec<(u64, u64)>>()
    {
        for i in start.ilog10()..=end.ilog10() {
            if i % 2 == 1 {
                let modifier = 10_u64.pow(i.div_ceil(2)) + 1;
                for j in cmp::max(10_u64.pow(i / 2), start.div_ceil(modifier))
                    ..=cmp::min(10_u64.pow(i.div_ceil(2)) - 1, end.div_ceil(modifier))
                {
                    if (start..=end).contains(&(j * modifier)) {
                        total += j * modifier;
                    }
                }
            }
        }
    }

    total
}

pub fn part_2(input: &str) -> u64 {
    let mut invalid = HashSet::new();

    for (start, end) in input
        .split(',')
        .map(|range| {
            let split = range.split('-').collect::<Vec<&str>>();
            (
                split[0].parse().unwrap_or_default(),
                split[1].parse().unwrap_or_default(),
            )
        })
        .collect::<Vec<(u64, u64)>>()
    {
        for i in start.ilog10()..=end.ilog10() {
            for j in 2..=i + 1 {
                if i % j == j - 1 {
                    let mut modifier = 1;
                    for k in 1..j {
                        modifier += 10_u64.pow((i + 1) * k / j);
                    }
                    for k in cmp::max(10_u64.pow(i / j), start.div_ceil(modifier))
                        ..=cmp::min(10_u64.pow(i.div_ceil(j)) - 1, end.div_ceil(modifier))
                    {
                        if (start..=end).contains(&(k * modifier)) {
                            invalid.insert(k * modifier);
                        }
                    }
                }
            }
        }
    }

    invalid.iter().sum()
}
