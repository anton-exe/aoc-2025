mod day01;
mod day02;
mod day03;
mod day04;

aoc_main::main! {
    year 2025;
    day01 : generator => part_1, part_2;
    day02 : generator => part_1, part_2;
    day03 : generator => part_1, part_1_but_like_part_2, part_2;
    day04 : generator => part_1, part_2;
}
