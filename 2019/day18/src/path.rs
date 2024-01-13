pub struct Path {
    pub destination: u8, // key: 'a'..'z'
    pub steps: usize,
    pub keys: u32,  // bit <n> = key 'a'+<n>
    pub doors: u32, // bit <n> = door 'A'+<n>
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut keys = String::new();
        let mut doors = String::new();

        for i in 0..26 {
            if self.keys & (1 << i) != 0 {
                keys.push((i + b'a') as char);
            }
            if self.doors & (1 << i) != 0 {
                doors.push((i + b'A') as char);
            }
        }

        write!(
            f,
            "[dest:{} keys:{}, doors:{}, steps:{}]",
            (self.destination) as char,
            keys,
            doors,
            self.steps
        )
    }
}
