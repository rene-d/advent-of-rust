use std::fmt;

struct Password {
    pwd: Vec<char>,
    loops: usize,
}

impl Password {
    fn new(pwd: &str) -> Self {
        Self {
            pwd: pwd.chars().collect(),
            loops: 0,
        }
    }

    fn next(&mut self) {
        for current in self.pwd.iter_mut().rev() {
            if *current == 'z' {
                *current = 'a';
            } else {
                *current = (*current as u8 + 1) as char;
                break;
            }
        }
        self.loops += 1;
    }

    fn is_valid(&self) -> bool {
        // Passwords must include one increasing straight of at least three
        // letters, like abc, bcd, cde, and so on, up to xyz. They cannot skip
        // letters; abd doesn't count.
        let mut straigh = false;
        for i in 0..self.pwd.len() - 2 {
            if self.pwd[i] as u8 + 1 == self.pwd[i + 1] as u8
                && self.pwd[i + 1] as u8 + 1 == self.pwd[i + 2] as u8
            {
                straigh = true;
            }
        }
        if !straigh {
            return false;
        }

        // Passwords may not contain the letters i, o, or l, as these letters
        // can be mistaken for other characters and are therefore confusing.
        for c in &self.pwd {
            match c {
                'i' | 'o' | 'l' => return false,
                _ => (),
            }
        }

        // Passwords must contain at least two different, non-overlapping pairs
        // of letters, like aa, bb, or zz.
        for i in 0..self.pwd.len() - 1 {
            // find a pair
            if self.pwd[i] == self.pwd[i + 1] {
                for j in i + 2..self.pwd.len() - 1 {
                    // then a second pair
                    if self.pwd[j] == self.pwd[j + 1] {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn next_valid(&mut self) {
        self.next();
        while !self.is_valid() {
            self.next();
        }
    }
}

impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} [loops: {}]",
            self.pwd.iter().collect::<String>(),
            self.loops
        )
    }
}

/// main function
fn main() {
    let mut pwd: Password = Password::new("hepxcrrq");

    println!("init:  {}", pwd);

    pwd.next_valid();
    println!("part1: {}", pwd);

    pwd.next_valid();
    println!("part2: {}", pwd);
}
