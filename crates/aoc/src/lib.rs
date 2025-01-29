mod args;
mod coord;
mod counter;
mod direction;
mod grid;
mod gridu;
pub mod hexslice;
pub mod integer;
pub mod knot;
pub mod math;
pub mod ocr;
mod square;
mod unwraperror;
pub mod util;

pub use args::Args;
pub use unwraperror::DAMN;

pub type Coord = coord::Coord;
pub type Direction = direction::Direction;
pub type Grid<T> = grid::Grid<T>;
pub type GridU<T> = gridu::GridU<T>;
pub type Square<T> = square::Square<T>;
pub type Counter<T> = counter::Counter<T>;

pub struct Christmas(());

/// Christmay Day
pub const CHRISTMAS: Christmas = Christmas(());

impl std::fmt::Display for Christmas {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Ok(())
    }
}

/// Parse commandline arguments and load input file.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// fn solve(data: &str) -> (i32, i32) {
///     (1234, 5678)
/// }
///
/// let args = aoc::parse_args();
/// args.run(solve);
/// ```
///
/// Advanced usage:
///
/// ```
/// struct Puzzle {}
/// impl Puzzle {
///     fn new(data: &str) -> Self { Self { } }
///     fn part1(&self) -> &'static str { "road" }
///     fn part2(&self) -> u32 { 66 }
/// }
///
/// let args = aoc::parse_args();
/// args.run(|data| {
///     let puzzle = Puzzle::new(data);
///     (puzzle.part1(), puzzle.part2())
/// });
/// ```
#[must_use]
pub fn parse_args() -> Args {
    Args::parse_args()
}

#[must_use]
pub fn parse_args_raw() -> Args {
    Args::parse_args_raw()
}

/// Read the puzzle input.
/// # Panics
/// If the file cannot be found or read
#[must_use]
pub fn load_input_data(filename: &str) -> String {
    if filename == "-" {
        std::fs::read_to_string("/dev/stdin").unwrap()
    } else if std::path::Path::new(filename).is_file() {
        std::fs::read_to_string(filename).unwrap()
    } else {
        eprintln!("error: cannot read file {filename}");
        std::process::exit(1);
    }
}
