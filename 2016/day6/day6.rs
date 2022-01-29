/*!
[Day 6: Signals and Noise](https://adventofcode.com/2016/day/6)

Something is jamming your communications with Santa. Fortunately, your
signal is only partially jammed, and protocol in situations like this
is to switch to a simple [repetition code](https://en.wikipedia.org/wiki/Repetition_code)
to get the message through.

In this model, the same message is sent repeatedly. You've recorded the
repeating message signal (your puzzle input), but the data seems quite
corrupted - almost too badly to recover. **Almost**.

All you need to do is figure out which character is most frequent for
each position. For example, suppose you had recorded the following
messages:

```text
eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar
```

The most common character in the first column is `e`; in the second, `a`;
in the third, `s`, and so on. Combining these characters returns the
error-corrected message, `easter`.

Given the recording in your puzzle input, **what is the error-corrected
version** of the message being sent?

--- Part Two ---

Of course, that **would be the message** - if you hadn't agreed to use
a **modified repetition code** instead.

In this modified code, the sender instead transmits what looks like
random data, but for each character, the character they actually want to
send is **slightly less likely** than the others. Even after signal-jamming
noise, you can look at the letter distributions in each column and choose
the **least common** letter to reconstruct the original message.

In the above example, the least common character in the first column is `a`;
in the second, `d`, and so on. Repeating this process for the remaining
characters produces the original message, `advent`.

Given the recording in your puzzle input and this new decoding methodology,
**what is the original message** that Santa is trying to send?
*/

fn main() {
    let data = std::fs::read_to_string("input.txt").unwrap();

    let (part1, part2) = solve(&data);
    println!("{}", part1);
    println!("{}", part2);
}

/// solve both parts
fn solve(data: &str) -> (String, String) {
    let mut freq = [[0; 26]; 8];

    for line in data.split('\n') {
        for (i, c) in line.chars().enumerate() {
            assert!(i < 8, "too many chars");
            if ('a'..='z').contains(&c) {
                freq[i][c as usize - 'a' as usize] += 1;
            }
        }
    }

    let mut message_most = String::new();
    let mut message_least = String::new();

    for freq_c in &freq {
        let mut max = usize::MIN;
        let mut max_char = ' ';

        let mut min = usize::MAX;
        let mut min_char = ' ';

        for (j, f) in freq_c.iter().enumerate().take(26) {
            if *f > max {
                max = *f;
                max_char = (u8::try_from(j).unwrap() + b'a') as char;
            }
            if *f != 0 && *f < min {
                min = *f;
                min_char = (u8::try_from(j).unwrap() + b'a') as char;
            }
        }

        message_most.push(max_char);
        message_least.push(min_char);
    }

    (message_most, message_least)
}

#[cfg(test)]
#[test]
fn test_part1() {
    let data = "eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar";

    let (part1, part2) = solve(&data);

    assert_eq!(part1, "easter  ".to_string());
    assert_eq!(part2, "advent  ".to_string());
}
