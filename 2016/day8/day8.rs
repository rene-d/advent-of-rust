//! [Day 8: Two-Factor Authentication](https://adventofcode.com/2016/day/8)

#![allow(clippy::manual_memcpy)]
#![allow(clippy::needless_range_loop)] // assumed. code is much clearer

use phf::phf_map;
use regex::Regex;

/// ``main`` reads the puzzle input then solves part 1 and part 2
fn main() {
    let data = aoc::load_input_data(8);

    solve(&data);
}

fn solve(data: &str) {
    const WIDTH: usize = 50;
    const HEIGHT: usize = 6;

    let re_rect = Regex::new(r"rect (\d+)x(\d+)").unwrap();
    let re_row = Regex::new(r"rotate row y=(\d+) by (\d+)").unwrap();
    let re_col = Regex::new(r"rotate column x=(\d+) by (\d+)").unwrap();

    let mut grid = vec![vec![false; WIDTH]; HEIGHT];

    for line in data.split('\n') {
        if let Some(caps) = re_rect.captures(line) {
            let width = caps[1].parse::<usize>().unwrap();
            let height = caps[2].parse::<usize>().unwrap();

            // println!("rect {}x{}", width, height);

            for x in 0..width {
                for y in 0..height {
                    grid[y][x] = true;
                }
            }
        } else if let Some(caps) = re_row.captures(line) {
            let y = caps[1].parse::<usize>().unwrap();
            let by = caps[2].parse::<usize>().unwrap();

            // println!("row {} by {}", y, by);

            let mut new_row = vec![false; WIDTH];
            for x in 0..WIDTH {
                new_row[(x + by) % WIDTH] = grid[y][x];
            }
            for x in 0..WIDTH {
                grid[y][x] = new_row[x];
            }
        } else if let Some(caps) = re_col.captures(line) {
            let x = caps[1].parse::<usize>().unwrap();
            let by = caps[2].parse::<usize>().unwrap();

            // println!("col {} by {}", x, by);

            let mut new_col = vec![false; HEIGHT];
            for y in 0..HEIGHT {
                new_col[(y + by) % HEIGHT] = grid[y][x];
            }
            for y in 0..HEIGHT {
                grid[y][x] = new_col[y];
            }
        } else {
            // panic!("bad line: {}", line);
        }
    }

    let mut lit = 0;
    let mut crt = String::new();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if grid[y][x] {
                lit += 1;
                crt.push('#');
            } else {
                crt.push('.');
            }
        }
        crt.push('\n');
    }

    println!("{}", lit); // part 1
    println!("{}", ocr(&crt)); // part 2
}

// UPOJFLBCEZ

static CHARSET_5X6: phf::Map<&'static str, char> = phf_map! {
    ".##.. #..#. #..#. ####. #..#. #..#." => 'A',
    "###.. #..#. ###.. #..#. #..#. ###.." => 'B',
    ".##.. #..#. #.... #.... #..#. .##.." =>'C',
    "####. #.... ###.. #.... #.... ####." =>'E',
    "####. #.... ###.. #.... #.... #...." => 'F',
    ".##.. #..#. #.... #.##. #..#. .###." => 'G',
    "#..#. #..#. ####. #..#. #..#. #..#." => 'H',
    ".###. ..#.. ..#.. ..#.. ..#.. .###." => 'I',
    "..##. ...#. ...#. ...#. #..#. .##.." => 'J',
    "#..#. #.#.. ##... #.#.. #.#.. #..#." => 'K',
    "#.... #.... #.... #.... #.... ####." => 'L',
    ".##.. #..#. #..#. #..#. #..#. .##.." => 'O',
    "###.. #..#. #..#. ###.. #.... #...." => 'P',
    "###.. #..#. #..#. ###.. #.#.. #..#." => 'R',
    ".###. #.... #.... .##.. ...#. ###.." => 'S',
    "#..#. #..#. #..#. #..#. #..#. .##.." => 'U',
    "#...# #...# .#.#. ..#.. ..#.. ..#.." => 'Y',
    "####. ...#. ..#.. .#... #.... ####." => 'Z',
};

fn ocr(text: &str) -> String {
    let lines = text.lines().collect::<Vec<&str>>();

    let width = lines.iter().map(|x| x.len()).min().unwrap();

    let mut x = 0;
    let mut result = String::new();

    while x < width - 5 + 1 {
        let key = (0..6).map(|y| &lines[y][x..(x + 5)]).collect::<Vec<&str>>().join(" ");

        if let Some(letter) = CHARSET_5X6.get(&key) {
            result.push(*letter);
            x += 5;
        } else {
            x += 1;
        }
    }

    result
}
