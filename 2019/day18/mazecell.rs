pub trait MazeCell {
    fn is_wall(&self) -> bool;
    fn is_key(&self) -> bool;
    fn is_door(&self) -> bool;
    fn is_entrance(&self) -> bool;
    // fn is_robot(&self) -> bool;

    fn set_wall(&mut self);
    fn set_entrance(&mut self);
}

impl MazeCell for u8 {
    fn is_wall(&self) -> bool {
        self == &b'#'
    }

    fn is_key(&self) -> bool {
        self.is_ascii_lowercase()
        // (b'a'..=b'z').contains(self)
    }

    fn is_door(&self) -> bool {
        self.is_ascii_uppercase()
        // (b'A'..=b'Z').contains(self)
    }

    fn is_entrance(&self) -> bool {
        self == &b'@'
    }

    // fn is_robot(&self) -> bool {
    //     // valid after maze update in part 2
    //     (b'1'..=b'4').contains(self)
    // }

    fn set_wall(&mut self) {
        *self = b'#';
    }

    fn set_entrance(&mut self) {
        *self = b'@';
    }
}
