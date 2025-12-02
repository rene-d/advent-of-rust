// pub mod rundb;

use itertools::Itertools;
use std::iter::empty;

/// Get the array of all available solutions.
#[must_use]
pub fn solutions(year: Option<u16>, day: Option<u8>, alt: &Option<String>) -> Vec<Solution> {
    let sols = empty()
        .chain(year2015())
        .chain(year2016())
        .chain(year2017())
        .chain(year2018())
        .chain(year2019())
        .chain(year2020())
        .chain(year2021())
        .chain(year2022())
        .chain(year2023())
        .chain(year2024())
        .chain(year2025());

    sols.filter(|sol| year.is_none_or(|x| x == sol.year))
        .filter(|sol| day.is_none_or(|x| x == sol.day))
        .filter(|sol| alt == &Some("*".to_string()) || alt == &sol.alt)
        .sorted_unstable_by_key(|sol| (sol.year, sol.day, sol.alt.is_some()))
        .collect()
}

/// A solution for given year and day.
/// Offer two callbacks:
///  - `solve` that takes the puzzle input and returns part one and two
///  - `main` that acts like a standalone program for the given day
#[derive(Clone)]
pub struct Solution {
    pub year: u16,
    pub day: u8,
    pub alt: Option<String>,
    pub solve: fn(&str) -> (String, String),
    pub main: fn() -> (),
}

macro_rules! make_year {
    ($year:tt $($day:tt),*) => {
        mod $year {
            $(pub mod $day { pub mod $day; })*
        }

        #[must_use]
        pub fn $year() -> Vec<Solution> {
            vec![$({
                let year = stringify!($year)[4..].parse().unwrap();
                let day = &stringify!($day)[3..];

                let (day, alt) = day
                    .split_once('_')
                    .map_or((day, None), |(day, alt)| (day, Some(alt.to_string())));
                let day = day.parse().unwrap();

                let solve = |data: &str| {
                    use crate::$year::$day::$day::solve;
                    let (part1, part2) = solve(data);
                    (part1.to_string(), part2.to_string())
                };

                let main = || {
                    use crate::$year::$day::$day::main;
                    main();
                };

                Solution { year, day, alt, solve, main }
            },)*]
        }
    }
}

make_year!(year2015
    day1,day2,day3,day4,day5,day6,day7,day8,day9,day10,day11,day12,day13,
    day14,day15,day16,day17,day18,day19,day20,day21,day22,day23,day24,day25
);

make_year!(year2016
    day1,day2,day3,day4,day5,day6,day7,day8,day9,day10,day11,day12,day13,
    day14,day15,day16,day17,day18,day19,day20,day21,day22,day23,day24,day25
);

make_year!(year2017
    day1,day2,day3,day4,day5,day6,day7,day8,day9,day10,day11,day12,day13,
    day14,day15,day16,day17,day18,day19,day20,day21,day22,day23,day24,day25
);

make_year!(year2018
    day1,day2,day3,day4,day5,day6,day7,day8,day9,day10,day11,day12,day13,
    day14,day15,day16,day17,day18,day19,day20,day21,day22,day23,day24,day25,
    day9_c,day23_z3
);

make_year!(year2019
    day1,day2,day3,day4,day5,day6,day7,day8,day9,day10,day11,day12,day13,
    day14,day15,day16,day17,day18,day19,day20,day21,day22,day23,day24,day25
);

make_year!(year2020
    day1,day2,day3,day4,day5,day6,day7,day8,day9,day10,day11,day12,day13,
    day14,day15,day16,day17,day18,day19,day20,day21,day22,day23,day24,day25
);

make_year!(year2021
    day1,day2,day3,day4,day5,day6,day7,day8,day9,day10,day11,day12,day13,
    day14,day15,day16,day17,day18,day19,day20,day21,day22,day23,day24,day25
);

make_year!(year2022
    day1,day2,day3,day4,day5,day6,day7,day8,day9,day10,day11,day12,day13,
    day14,day15,day16,day17,day18,day19,day20,day21,day22,day23,day24,day25,
    day13_pest
);

make_year!(year2023
    day1,day2,day3,day4,day5,day6,day7,day8,day9,day10,day11,day12,day13,
    day14,day15,day16,day17,day18,day19,day20,day21,day22,day23,day24,day25
);

make_year!(year2024
    day1,day2,day3,day4,day5,day6,day7,day8,day9,day10,day11,day12,day13,
    day14,day15,day16,day17,day18,day19,day20,day21,day22,day23,day24,day25,
    day13_z3
);

make_year!(year2025
    day1,day2
);
