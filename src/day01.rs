pub fn generator(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|line| {
            line.replace('L', "-")
                .replace('R', "")
                .parse()
                .unwrap_or_default()
        })
        .collect()
}

pub fn part_1(input: &[i64]) -> i64 {
    let mut value = 50;
    let mut count = 0;

    for change in input {
        value = (value + change).rem_euclid(100);

        if value == 0 {
            count += 1;
        }
    }

    count
}

pub fn part_2(input: &[i64]) -> i64 {
    let mut value = 50;
    let mut count = 0;

    for change in input {
        let raw_value = value + change;
        value = raw_value.rem_euclid(100);

        match raw_value.cmp(&0) {
            std::cmp::Ordering::Less => {
                count += (raw_value / 100).abs();
                if raw_value != *change {
                    count += 1;
                }
            }
            std::cmp::Ordering::Equal => count += 1,
            std::cmp::Ordering::Greater => count += raw_value / 100,
        }
    }

    count
}
