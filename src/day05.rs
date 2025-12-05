use std::{
    cmp::{max, min},
    collections::HashSet,
    ops::RangeInclusive,
};

pub fn generator(input: &str) -> &str {
    input
}

pub fn part_1(input: &str) -> u64 {
    let (ranges, ingredients) = input.split_once("\n\n").unwrap_or_default();

    ingredients
        .lines()
        .map(|ingredient| {
            for (start, end) in ranges
                .lines()
                .map(|line| line.split_once('-').unwrap_or_default())
            {
                if (start.parse::<u64>().unwrap_or_default()
                    ..=end.parse::<u64>().unwrap_or_default())
                    .contains(&ingredient.parse::<u64>().unwrap_or_default())
                {
                    return 1;
                }
            }
            0
        })
        .sum()
}

pub fn part_2(input: &str) -> u64 {
    let mut ranges = input
        .split_once("\n\n")
        .unwrap_or_default()
        .0
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap_or_default();
            start.parse::<u64>().unwrap_or_default()..=end.parse::<u64>().unwrap_or_default()
        })
        .collect::<HashSet<RangeInclusive<u64>>>();

    loop {
        let mut compressed_ranges = HashSet::<RangeInclusive<u64>>::new();

        let start_size = ranges.len();

        for range in &ranges {
            let mut working_range = range.clone();

            for range2 in &ranges {
                if range2.contains(working_range.start()) || range2.contains(working_range.end()) {
                    working_range =
                        min(*range.start(), *range2.start())..=max(*range.end(), *range2.end());
                }
            }

            compressed_ranges.insert(working_range);
        }

        ranges = compressed_ranges;

        if start_size == ranges.len() {
            break;
        }
    }

    ranges
        .iter()
        .map(|range| range.end() - range.start() + 1)
        .sum()
}
