//! [Day 23: LAN Party](https://adventofcode.com/2024/day/23)

// Node names are 2 lowercase letters → encode as u16 index 0..675
const N: usize = 26 * 26; // 676 possible node IDs
const W: usize = N.div_ceil(64); // 11 u64 words per bitset

type Bitset = [u64; W];
const ZERO: Bitset = [0u64; W];

const fn bs_set(bs: &mut Bitset, i: usize) {
    bs[i >> 6] |= 1u64 << (i & 63);
}

fn bs_and(a: &Bitset, b: &Bitset) -> Bitset {
    std::array::from_fn(|i| a[i] & b[i])
}

fn bs_count_and(a: &Bitset, b: &Bitset) -> u32 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x & y).count_ones())
        .sum()
}

fn bs_empty(bs: &Bitset) -> bool {
    bs.iter().all(|&w| w == 0)
}

/// Returns an iterator over the indices of set bits in `bs`.
fn bs_ones(bs: Bitset) -> impl Iterator<Item = usize> {
    let mut bits = bs;
    let mut wi = 0usize;
    std::iter::from_fn(move || {
        while wi < W {
            if bits[wi] != 0 {
                let bit = bits[wi].trailing_zeros() as usize;
                bits[wi] &= bits[wi] - 1; // clear lowest set bit
                return Some(wi * 64 + bit);
            }
            wi += 1;
        }
        None
    })
}

fn encode(s: &[u8]) -> usize {
    usize::from(s[0] - b'a') * 26 + usize::from(s[1] - b'a')
}

/// Returns `true` if the node starts with `'t'` (`'t' - 'a' == 19`).
const fn is_t_node(node: usize) -> bool {
    node / 26 == 19
}

fn node_name(i: usize) -> String {
    let hi = b'a' + u8::try_from(i / 26).expect("i/26 < 26");
    let lo = b'a' + u8::try_from(i % 26).expect("i%26 < 26");
    format!("{}{}", char::from(hi), char::from(lo))
}

struct Puzzle {
    adj: Vec<Bitset>,
    active: Bitset,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let mut adj = vec![ZERO; N];
        let mut active = ZERO;

        for line in data.lines() {
            if let Some((a, b)) = line.split_once('-') {
                let ai = encode(a.as_bytes());
                let bi = encode(b.as_bytes());
                bs_set(&mut adj[ai], bi);
                bs_set(&mut adj[bi], ai);
                bs_set(&mut active, ai);
                bs_set(&mut active, bi);
            }
        }

        Self { adj, active }
    }

    fn part1(&self) -> usize {
        let mut count = 0usize;
        for u in bs_ones(self.active) {
            for v in bs_ones(self.adj[u]) {
                if v <= u {
                    continue;
                }
                for w in bs_ones(bs_and(&self.adj[u], &self.adj[v])) {
                    if w > v && (is_t_node(u) || is_t_node(v) || is_t_node(w)) {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    fn part2(&self) -> String {
        let mut best: Vec<usize> = Vec::new();
        let mut r: Vec<usize> = Vec::new();
        let mut p = self.active;
        let mut x = ZERO;

        self.bron_kerbosch(&mut r, &mut p, &mut x, &mut best);

        // Sorting node indices gives lexicographic order on names (encode preserves lex order)
        best.sort_unstable();
        best.iter()
            .map(|&i| node_name(i))
            .collect::<Vec<_>>()
            .join(",")
    }

    fn bron_kerbosch(
        &self,
        r: &mut Vec<usize>,
        p: &mut Bitset,
        x: &mut Bitset,
        best: &mut Vec<usize>,
    ) {
        if bs_empty(p) {
            if bs_empty(x) && r.len() > best.len() {
                best.clone_from(r);
            }
            return;
        }

        // Pivot: choose u ∈ P ∪ X that maximises |P ∩ N(u)|
        let px: Bitset = std::array::from_fn(|i| p[i] | x[i]);
        let pivot = bs_ones(px)
            .max_by_key(|&u| bs_count_and(p, &self.adj[u]))
            .expect("P ∪ X non-empty (P is non-empty)");

        // Candidates: P \ N(pivot)
        let candidates: Bitset = std::array::from_fn(|i| p[i] & !self.adj[pivot][i]);

        for v in bs_ones(candidates) {
            r.push(v);

            let mut p_new = bs_and(p, &self.adj[v]);
            let mut x_new = bs_and(x, &self.adj[v]);
            self.bron_kerbosch(r, &mut p_new, &mut x_new, best);

            r.pop();
            p[v >> 6] &= !(1u64 << (v & 63));
            x[v >> 6] |= 1u64 << (v & 63);
        }
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (usize, String) {
    let puzzle = Puzzle::new(data);
    (puzzle.part1(), puzzle.part2())
}

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn test_part1() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), 7);
    }

    #[test]
    fn test_part2() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), "co,de,ka,ta");
    }
}
