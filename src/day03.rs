pub fn generator(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|number| u64::from(number - 48))
                .collect()
        })
        .collect()
}

pub fn part_1(input: &[Vec<u64>]) -> u64 {
    input
        .iter()
        .map(|line| {
            line[..line.len() - 1].iter().max().unwrap() * 10
                + line[line
                    .iter()
                    .position(|value| value == line[..line.len() - 1].iter().max().unwrap())
                    .unwrap()
                    + 1..]
                    .iter()
                    .max()
                    .unwrap()
        })
        .sum()
}

pub fn part_1_but_like_part_2(input: &[Vec<u64>]) -> u64 {
    const DIGITS: usize = 2;
    input
        .iter()
        .map(|line| {
            let mut nums = line.clone();

            (0..DIGITS)
                .map(|i| {
                    let chosen = nums[..nums.len() - (DIGITS - 1 - i)]
                        .iter()
                        .enumerate()
                        .rev()
                        .max_by_key(|(_idx, val)| *val)
                        .unwrap();
                    nums = nums[chosen.0..].to_vec();
                    nums.remove(0) * 10u64.pow((DIGITS - 1 - i).try_into().unwrap())
                })
                .sum::<u64>()
        })
        .sum()
}

pub fn part_2(input: &[Vec<u64>]) -> u64 {
    const DIGITS: usize = 12;
    input
        .iter()
        .map(|line| {
            let mut nums = line.clone();

            (0..DIGITS)
                .map(|i| {
                    let chosen = nums[..nums.len() - (DIGITS - 1 - i)]
                        .iter()
                        .enumerate()
                        .rev()
                        .max_by_key(|(_idx, val)| *val)
                        .unwrap();
                    nums = nums[chosen.0..].to_vec();
                    nums.remove(0) * 10u64.pow((DIGITS - 1 - i).try_into().unwrap())
                })
                .sum::<u64>()
        })
        .sum()
}
