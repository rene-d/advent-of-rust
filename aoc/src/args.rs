
#[derive(Parser)]
pub struct Args {
    /// Puzzle input
    #[arg(default_value = "input.txt")]
    path: String,
}


pub fn parse() -> Args {
    let args = Args::parse();
    args
}