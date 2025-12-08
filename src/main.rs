mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

aoc_main::main! {
    year 2025;
    day01 : generator => part_1, part_2;
    day02 : generator => part_1, part_2;
    day03 : generator => part_1, part_1_but_like_part_2, part_2;
    day04 : generator => part_1, part_2;
    day05 : generator => part_1, part_2;
    day06 : generator => part_1, part_2;
    day07 : generator => part_1, part_2;
    day08 : generator => part_1, part_2;
}
