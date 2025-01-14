use args::Args;

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
pub mod util;

pub type Coord = coord::Coord;
pub type Direction = direction::Direction;
pub type Grid<T> = grid::Grid<T>;
pub type GridU<T> = gridu::GridU<T>;
pub type Square<T> = square::Square<T>;
pub type Counter<T> = counter::Counter<T>;

#[must_use]
pub fn parse_args() -> args::Args {
    Args::parse_args()
}

/// Read the puzzle input
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
