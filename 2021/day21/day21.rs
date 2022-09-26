// Day 21: Dirac Dice
// https://adventofcode.com/2021/day/21

use std::collections::HashMap;
use std::env;

fn main() {
    let args: Vec<u8> = env::args()
        .skip(1)
        .map(|s| s.parse::<u8>().unwrap())
        .collect();

    let player: u8;
    let computer: u8;

    if args.len() == 2 {
        player = args[0];
        computer = args[1];
    } else {
        player = 5;
        computer = 9;
    }

    println!("{}", part1(player, computer));
    println!("{}", part2(player, computer));
}

fn part2(player: u8, computer: u8) -> u64 {
    fn solve(
        cache: &mut HashMap<(u8, u8, u64, u64), (u64, u64)>,
        position1: u8,
        position2: u8,
        score1: u64,
        score2: u64,
    ) -> (u64, u64) {
        if score1 >= 21 {
            return (1, 0);
        }
        if score2 >= 21 {
            return (0, 1);
        }

        // memoization
        let key = (position1, position2, score1, score2);
        if let Some(v) = cache.get(&key) {
            return *v;
        }

        // explore the Dirac dices universes
        let mut total = (0, 0);
        for die1 in 1..=3 {
            for die2 in 1..=3 {
                for die3 in 1..=3 {
                    // player1's turn
                    let new_position1 = (position1 - 1 + die1 + die2 + die3) % 10 + 1;
                    let new_score1 = score1 + u64::from(new_position1);

                    // next turns
                    let next_turns = solve(cache, position2, new_position1, score2, new_score1);

                    total.0 += next_turns.1;
                    total.1 += next_turns.0;
                }
            }
        }

        cache.insert(key, total);
        total
    }

    let mut cache: HashMap<(u8, u8, u64, u64), (u64, u64)> = HashMap::new();

    let res = solve(&mut cache, player, computer, 0, 0);

    std::cmp::max(res.0, res.1)
}

fn part1(player: u8, computer: u8) -> u32 {
    let mut current_player = 0;
    let mut scores = vec![0, 0];
    let mut positions = vec![player, computer];

    let mut die: u32 = 0;
    let mut roll = || -> u32 {
        die += 1;
        (die - 1) % 100 + 1
    };

    for _ in 0..1000 {
        let d1 = roll();
        let d2 = roll();
        let d3 = roll();

        let new_place =
            u8::try_from((u32::from(positions[current_player]) + d1 + d2 + d3 - 1) % 10 + 1)
                .unwrap();
        let new_score = scores[current_player] + u32::from(new_place);

        scores[current_player] = new_score;
        positions[current_player] = new_place;

        #[cfg(debug_assertions)]
        println!(
            "Player {} rolls {}+{}+{} and moves to space {} for a total score of {}.",
            current_player + 1,
            d1,
            d2,
            d3,
            new_place,
            new_score
        );

        if new_score >= 1000 {
            return scores[1 - current_player] * die;
        }

        current_player = 1 - current_player;
    }

    0
}

#[cfg(test)]
#[test]
fn test_part1() {
    assert_eq!(part1(4, 8), 739785);
}

#[test]
fn test_part2() {
    assert_eq!(part2(4, 8), 444356092776315);
}
