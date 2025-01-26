use colored::Colorize;
use itertools::Itertools;
use one::{solutions, Solution};
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

fn main() {
    let args = aoc::parse_args_raw();

    // in raw/runall.py mode we need a file input path
    if args.has_option("-r") {
        runall_wrapper();
        return;
    }

    // print list of solutions, can be filtered
    if args.has_option("-l") {
        list_solutions();
        return;
    }

    // in a YEAR/dayDAY directory, we act as the standalone daily binary
    if run_day_directory() {
        return;
    }

    // get the year or year/day filter
    let mut year: Option<u16> = None;
    let mut day: Option<u8> = None;

    if !args.params().is_empty() {
        let re = regex::Regex::new(r"(\d+)").unwrap();

        let mut m = re.find_iter(&args.params()[0]);

        if let Some(y) = m.next() {
            year = y.as_str().parse().ok();
        }
        if let Some(d) = m.next() {
            day = d.as_str().parse().ok();
        }
    }

    let alt = if args.has_option("-a") {
        Some("*".to_string())
    } else {
        None
    };

    // and apply it to the solution inventory
    let sols = solutions(year, day, &alt);

    //  run all solutions
    run_all(&sols);
}

/// Test in we are in yearXXXX/dayYY directory.
/// IF yes, run the corresponding solution with
/// input in `input.txt` if found.
fn run_day_directory() -> bool {
    if let Ok(path) = std::env::current_dir() {
        if let Some(day) = path.file_name() {
            if let Some(day) = day.to_str().unwrap().strip_prefix("day") {
                //
                let (day, alt) = day
                    .split_once('_')
                    .map_or((day, None), |(day, alt)| (day, Some(alt.to_string())));

                if let Some(year) = path.parent() {
                    if let Some(year) = year
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .strip_prefix("year")
                    {
                        let year: Option<u16> = year.parse().ok();
                        let day: Option<u8> = day.parse().ok();

                        if let Some(sol) = solutions(year, day, &alt).first() {
                            // eprintln!("run solution {} {} {}", sol.year,sol.day, sol.alt.clone().unwrap_or("".to_string()));

                            let args = aoc::parse_args_raw();
                            if args.params().is_empty() {
                                run_day(sol, true);
                            } else {
                                (sol.main)();
                            }
                        }
                    }

                    return true;
                }
            }
        }
    }

    false
}

fn print_part_result(part: u8, answer: &str, ok: &str, day: u8) {
    if part == 2 && day == 25 {
        println!(
            "  {}  : {}",
            "Part 2".yellow(),
            "n/a".dimmed() // "Merry Christmas".bright_blue()
        );
    } else {
        print!("  {}  : ", format!("Part {part}").yellow());
        if ok.is_empty() {
            println!("{answer}");
        } else if answer.trim_ascii() == ok.trim_ascii() {
            println!("{}", answer.bright_green());
        } else {
            println!("{}", answer.bright_red());
        };
    }
}

fn run_all(sols: &[Solution]) {
    println!("💫 {} 🎄✨ 💫", "Advent of Code".green());

    let mut total_elapsed = Duration::ZERO;
    let mut puzzles = 0;
    let mut success = 0;
    let mut failed = 0;

    for sol in sols {
        // if sol.alt.is_some() && !alt {
        //     continue;
        // }

        // if sol.alt.is_none() {
        //     continue;
        // }

        if let Some((elapsed, ok, ko)) = run_day(sol, false) {
            puzzles += 1;
            total_elapsed += elapsed;
            success += i32::from(ok);
            failed += i32::from(ko);
        }
    }

    if puzzles > 1 {
        println!();
        println!("Elapsed: {total_elapsed:#?} for {puzzles} puzzle(s) - success: {success}, failed: {failed}");
    }
}

fn run_day(sol: &Solution, input_txt: bool) -> Option<(Duration, bool, bool)> {
    let (path_input, path_answer) = find_input_path(sol, input_txt);

    if let Some(alt) = &sol.alt {
        println!(
            "{} day {} ({}): {}",
            sol.year,
            sol.day,
            alt.magenta(),
            path_input.as_os_str().to_str().unwrap().dimmed()
        );
    } else {
        println!(
            "{} day {}: {}",
            sol.year,
            sol.day,
            path_input.as_os_str().to_str().unwrap().dimmed()
        );
    }

    if path_input.is_file() {
        if let Ok(data) = std::fs::read_to_string(&path_input) {
            // run the solution
            let instant = Instant::now();
            let (part1, part2) = (sol.solve)(&data);
            let elapsed = instant.elapsed();

            let mut success = false;
            let mut failed = false;

            let micros = Duration::new(elapsed.as_secs(), elapsed.subsec_micros() * 1000);

            if let Ok(ok) = std::fs::read_to_string(path_answer) {
                let (ok1, ok2) = ok.trim_ascii().split_once('\n').unwrap_or((&ok, ""));

                print_part_result(1, &part1, ok1, sol.day);
                print_part_result(2, &part2, ok2, sol.day);

                if ok1.trim_ascii() == part1 && ok2.trim_ascii() == part2 {
                    success = true;
                } else {
                    failed = true;
                }
            } else {
                print_part_result(1, &part1, "", sol.day);
                print_part_result(2, &part2, "", sol.day);
            }

            println!("{}", format!("  Elapsed : {micros:#?}").italic());
            println!();

            return Some((elapsed, success, failed));
        }
    }

    println!("  missing file: {}", path_input.to_str().unwrap().red());

    None
}

fn find_input_path(sol: &Solution, input_txt: bool) -> (PathBuf, PathBuf) {
    let hint = if input_txt {
        PathBuf::new().with_file_name("input.txt")
    } else {
        Path::new("input")
            .join(sol.year.to_string())
            .join(sol.day.to_string())
            .with_extension("in")
    };

    let mut path = hint.clone();

    if !path.is_file() {
        path = Path::new("input")
            .join(sol.year.to_string())
            .join(format!("day{}", sol.day))
            .with_extension("txt");
    }

    if !path.is_file() {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
            .ok()
            .unwrap_or_else(|| ".".to_string());

        let data_dir = Path::new(&manifest_dir).join("data");

        if let Ok(data_dir) = std::fs::read_dir(data_dir) {
            let user_sol = data_dir
                .filter_map(std::result::Result::ok)
                .filter(|f| f.path().is_dir())
                .filter(|f| f.file_name().as_bytes().iter().all(u8::is_ascii_digit))
                .sorted_by_key(std::fs::DirEntry::file_name)
                .next();

            if let Some(user_sol) = user_sol {
                path = user_sol
                    .path()
                    .join(sol.year.to_string())
                    .join(sol.day.to_string())
                    .with_extension("in");
            }
        }
    }

    if !path.is_file() {
        path = hint;
    }

    if path.file_name() == Path::new("input.txt").file_name() {
        (path.clone(), path.with_file_name("answer.txt"))
    } else {
        (path.clone(), path.with_extension("ok"))
    }
}

fn runall_wrapper() {
    let args = aoc::parse_args_raw();

    if args.params().len() != 2 {
        println!("-r requires a filter and a path");
        std::process::exit(1);
    }

    let filter = &args.params()[0];
    // let input_file = &args.params()[1];

    let re = regex::Regex::new(r"(\d+):(\d+)(?:[:_](\w+))?").unwrap();

    let mut year: Option<u16> = None;
    let mut day: Option<u8> = None;
    let mut alt: Option<String> = None;

    if let Some(caps) = re.captures(filter) {
        year = caps[1].parse().ok();
        day = caps[2].parse().ok();
        if let Some(s) = caps.get(3) {
            alt = Some(s.as_str().to_string());
        }
    }

    let sols = solutions(year, day, &alt);

    if sols.len() != 1 {
        println!("-r requires exactly one solution ({} found with year={year:?} day={day:?} alt={alt:?})", sols.len());
        std::process::exit(1);
    }

    let path = &args.params()[1];
    let data = aoc::load_input_data(path);
    let sol = &sols[0];

    args.run_data(sol.solve, &data);
}

fn list_solutions() {
    let args = aoc::parse_args_raw();

    // get the year or year/day filter
    let mut year: Option<u16> = None;
    let mut day: Option<u8> = None;
    let alt = Some("*".to_string());

    if !args.params().is_empty() {
        let re = regex::Regex::new(r"(\d+)").unwrap();

        let mut m = re.find_iter(&args.params()[0]);

        if let Some(y) = m.next() {
            year = y.as_str().parse().ok();
        }
        if let Some(d) = m.next() {
            day = d.as_str().parse().ok();
        }
    }

    for sol in &solutions(year, day, &alt) {
        let alt = sol
            .alt
            .clone()
            .map_or(String::new(), |alt| format!(" ({})", alt.magenta()));

        println!(
            "{} {} {} {:2}{}",
            "Year".green(),
            sol.year,
            "day".green(),
            sol.day,
            alt
        );
    }
}
