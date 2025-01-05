/// Greatest common divisor
pub fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        (a, b) = (b, a % b);
    }

    a
}

/// Least Common Multiple
pub fn lcm(a: i32, b: i32) -> i32 {
    a * (b / gcd(a, b))
}
