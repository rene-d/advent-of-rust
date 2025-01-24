pub struct HexSlice<'a>(&'a [u8]);

impl<'a> HexSlice<'a> {
    fn new<T>(data: &'a T) -> Self
    where
        T: ?Sized + AsRef<[u8]> + 'a,
    {
        HexSlice(data.as_ref())
    }
}

// You can choose to implement multiple traits, like Lower and UpperHex
impl std::fmt::Display for HexSlice<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for byte in self.0 {
            // Decide if you want to pad the value or have spaces inbetween, etc.
            write!(f, "{byte:02x}")?;
        }
        Ok(())
    }
}
pub trait HexDisplayExt {
    fn hex_display(&self) -> HexSlice<'_>;
}

impl<T> HexDisplayExt for T
where
    T: ?Sized + AsRef<[u8]>,
{
    fn hex_display(&self) -> HexSlice<'_> {
        HexSlice::new(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn display() {
        assert_eq!(b"Rosko".hex_display().to_string(), "526f736b6f");
    }
}
