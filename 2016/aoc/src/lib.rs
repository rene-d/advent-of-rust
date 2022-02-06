//! Utility functions

/// Read the puzzle input
/// # Panics
/// If the file cannot be found or read
#[must_use]
pub fn load_input_data(day: u8) -> String {
    let filename = if let Some(arg) = std::env::args().nth(1) {
        if arg == "-" {
            "/dev/stdin".to_string()
        } else {
            arg
        }
    } else {
        let filename = "input.txt";
        if std::path::Path::new(filename).is_file() {
            filename.to_owned()
        } else {
            let txt = format!("day{}/input.txt", day);
            if std::path::Path::new(&txt).is_file() {
                txt
            } else {
                panic!("input file not found");
            }
        }
    };

    std::fs::read_to_string(filename).unwrap()
}
