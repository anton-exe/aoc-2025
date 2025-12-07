pub fn generator(input: &str) -> &str {
    input
}

const NEIGHBOURS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub fn part_1(input: &str) -> u64 {
    let grid = input
        .lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|byte| {
                    if byte == &b'@' {
                        return true;
                    }
                    false
                })
                .collect()
        })
        .collect::<Vec<Vec<bool>>>();

    grid.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, cell)| {
                    if *cell
                        && NEIGHBOURS
                            .iter()
                            .map(|(dy, dx)| {
                                if ((y == 0) && (*dy < 0))
                                    || ((x == 0) && (*dx < 0))
                                    || (!grid
                                        .get(
                                            usize::try_from(
                                                i32::try_from(y).unwrap_or_default() + dy,
                                            )
                                            .unwrap_or_default(),
                                        )
                                        .unwrap_or(&Vec::<bool>::new())
                                        .get(
                                            usize::try_from(
                                                i32::try_from(x).unwrap_or_default() + dx,
                                            )
                                            .unwrap_or_default(),
                                        )
                                        .unwrap_or(&false))
                                {
                                    return 0;
                                }
                                1
                            })
                            .sum::<i64>()
                            < 4
                    {
                        return 1;
                    }
                    0
                })
                .sum::<u64>()
        })
        .sum()
}

pub fn part_2(input: &str) -> u64 {
    let mut total = 0;

    let mut grid = input
        .lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|byte| {
                    if byte == &b'@' {
                        return true;
                    }
                    false
                })
                .collect()
        })
        .collect::<Vec<Vec<bool>>>();

    loop {
        let og_grid = grid.clone();
        let sum = og_grid
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, cell)| {
                        if *cell
                            && NEIGHBOURS
                                .iter()
                                .map(|(dy, dx)| {
                                    if ((y == 0) && (*dy < 0))
                                        || ((x == 0) && (*dx < 0))
                                        || (!og_grid
                                            .get(
                                                usize::try_from(
                                                    i32::try_from(y).unwrap_or_default() + dy,
                                                )
                                                .unwrap_or_default(),
                                            )
                                            .unwrap_or(&Vec::<bool>::new())
                                            .get(
                                                usize::try_from(
                                                    i32::try_from(x).unwrap_or_default() + dx,
                                                )
                                                .unwrap_or_default(),
                                            )
                                            .unwrap_or(&false))
                                    {
                                        return 0;
                                    }
                                    1
                                })
                                .sum::<i64>()
                                < 4
                        {
                            grid[y][x] = false;
                            return 1;
                        }
                        0
                    })
                    .sum::<u64>()
            })
            .sum::<u64>();
        if sum == 0 {
            break;
        }
        total += sum;
    }
    total
}
