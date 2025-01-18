//! Utility functions

/// Join an array of copyable items
pub fn join<T: Copy>(a: &[T], sep: T) -> impl Iterator<Item = T> + '_ {
    let n = a.len() * 2 - 1;

    (0..n).map(move |i| if i % 2 == 0 { a[i / 2] } else { sep })
}

/// Join an array of copyable items with a final sepatator
pub fn join_with_final<T: Copy>(a: &[T], sep: T, end: T) -> impl Iterator<Item = T> + '_ {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_join() {
        let arr: Vec<_> = join(&[1, 2, 3, 4], 0).collect();
        assert_eq!(arr, [1, 0, 2, 0, 3, 0, 4]);
    }

    #[test]
    fn test_join_with_final() {
        let arr: String = join_with_final(&["a", "bb", "ccc"], ",", ".").collect();
        assert_eq!(arr, "a,bb,ccc.");
    }
}
