//! [Day 22: Crab Combat](https://adventofcode.com/2020/day/22)

use rustc_hash::FxHashSet;

/// Maximum length a deck.
/// 64 is faster than the minimum required value (50).
const SIZE: usize = 64;

#[derive(Clone)]
struct Deck {
    cards: [u8; SIZE],
    length: usize,
    bottom: usize,
}

impl Deck {
    #[inline]
    const fn new() -> Self {
        Self {
            cards: [0u8; SIZE],
            length: 0,
            bottom: 0,
        }
    }

    fn parse(cards: &str) -> Self {
        let mut deck = Self::new();
        for c in cards.lines().skip(1).map(|s| s.parse().unwrap()) {
            deck.push(c);
        }
        deck
    }

    #[inline]
    const fn is_empty(&self) -> bool {
        self.length == 0
    }

    #[inline]
    fn push(&mut self, card: u8) {
        self.cards[(self.bottom + self.length) % SIZE] = card;
        self.length += 1;
    }

    #[inline]
    fn pop(&mut self) -> u8 {
        let card = self.cards[self.bottom];
        self.bottom = (self.bottom + 1) % SIZE;
        self.length -= 1;
        card
    }

    #[inline]
    fn extend(&mut self, other: &Self) {
        for i in 0..other.length {
            self.cards[(self.bottom + self.length) % SIZE] = other.cards[(other.bottom + i) % SIZE];
            self.length += 1;
        }
    }

    fn score(&self) -> u32 {
        let mut idx = (self.bottom + self.length) % SIZE;
        let mut rank = 0;
        let mut s = 0;
        for _ in 0..self.length {
            idx = (idx + SIZE - 1) % SIZE;
            rank += 1;
            s += rank * u32::from(self.cards[idx]);
        }
        s
    }

    #[inline]
    fn take(&self, n: usize) -> Self {
        let mut new_deck = self.clone();
        new_deck.length = n;
        new_deck
    }
}

/// Make a hash with the two decks.
/// The hash is made with cards of the first deck, then 0, then the cards of the second deck.
fn make_hash(deck1: &Deck, deck2: &Deck) -> [u8; 64] {
    let mut hash = [0u8; 64];

    #[allow(clippy::needless_range_loop)]
    for i in 0..deck1.length {
        hash[i] = deck1.cards[(deck1.bottom + i) % SIZE];
    }

    for i in 0..deck2.length {
        hash[i + deck1.length + 1] = deck2.cards[(deck2.bottom + i) % SIZE];
    }

    hash
}

fn recursive_combat(deck1: &Deck, deck2: &Deck, recursive: bool) -> (Deck, Deck) {
    let mut seen = FxHashSet::default();

    let mut deck1 = deck1.clone();
    let mut deck2 = deck2.clone();

    while !deck1.is_empty() && !deck2.is_empty() {
        if recursive && !seen.insert(make_hash(&deck1, &deck2)) {
            deck1.extend(&deck2);
            return (deck1, Deck::new());
        }

        let card1 = deck1.pop();
        let card2 = deck2.pop();

        let deck1_win = if recursive
            && deck1.length >= usize::from(card1)
            && deck2.length >= usize::from(card2)
        {
            recursive_combat(
                &deck1.take(usize::from(card1)),
                &deck2.take(usize::from(card2)),
                true,
            )
            .1 //  deck2
            .is_empty()
        } else {
            card1 > card2
        };

        if deck1_win {
            deck1.push(card1);
            deck1.push(card2);
        } else {
            deck2.push(card2);
            deck2.push(card1);
        }
    }

    (deck1, deck2)
}

struct Puzzle {
    deck1: Deck,
    deck2: Deck,
}

impl Puzzle {
    /// Initialize from the puzzle input.
    fn new(data: &str) -> Self {
        let (cards1, cards2) = data.split_once("\n\n").unwrap();

        Self {
            deck1: Deck::parse(cards1),
            deck2: Deck::parse(cards2),
        }
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        let (win1, win2) = recursive_combat(&self.deck1, &self.deck2, false);
        win1.score() + win2.score()
    }

    /// Solve part two.
    fn part2(&self) -> u32 {
        let (win1, win2) = recursive_combat(&self.deck1, &self.deck2, true);
        win1.score() + win2.score()
    }
}

fn main() {
    let args = aoc::parse_args();
    let puzzle = Puzzle::new(&args.input);
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let data = aoc::load_input_data("test.txt");
        let puzzle = Puzzle::new(&data);
        assert_eq!(puzzle.part1(), 306);
    }

    #[test]
    fn part2() {
        let data = aoc::load_input_data("test.txt");
        let puzzle = Puzzle::new(&data);
        assert_eq!(puzzle.part2(), 291);
    }
}
