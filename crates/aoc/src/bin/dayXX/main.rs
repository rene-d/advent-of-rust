use std::time::Duration;

fn main() {
    let args = aoc::parse_args();

    if args.is_verbose() {
        println!("{args:#?}");
    }

    args.run(|data| {
        std::thread::sleep(Duration::from_micros(2983));
        (format!("data length: {}", data.len()), "")
    });
}
