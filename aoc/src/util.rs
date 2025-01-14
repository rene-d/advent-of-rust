//! Utility functions

/// Join an array of copyable items
pub fn join<T: Copy>(a: &[T], sep: T) -> impl Iterator<Item = T> + '_ {
    let n = a.len() * 2 - 1;

    (0..n).map(move |i| if i % 2 == 0 { a[i / 2] } else { sep })
}

/// Join an array of copyable items with a final sepatator
pub fn join_last<T: Copy>(a: &[T], sep: T, end: T) -> impl Iterator<Item = T> + '_ {
    let n = a.len() * 2;

    (0..n).map(move |i| {
        if i % 2 == 0 {
            a[i / 2]
        } else if i == n - 1 {
            end
        } else {
            sep
        }
    })
}
