use super::mazecell::MazeCell; // needed for trait

pub trait Multirobot {
    fn update(&mut self);
}

impl Multirobot for aoc::GridU<u8> {
    /// Update the maze for part 2: the main entrance becomes four robots
    fn update(&mut self) {
        let (x, y) = self
            .iter()
            .find_map(|(xy, c)| if c.is_entrance() { Some(xy) } else { None })
            .unwrap();

        self[(x, y)].set_wall();
        self[(x - 1, y)].set_wall();
        self[(x + 1, y)].set_wall();
        self[(x, y - 1)].set_wall();
        self[(x, y + 1)].set_wall();

        self[(x - 1, y - 1)].set_entrance();
        self[(x + 1, y - 1)].set_entrance();
        self[(x - 1, y + 1)].set_entrance();
        self[(x + 1, y + 1)].set_entrance();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn update() {
        let mut maze = aoc::GridU::<u8>::parse(
            "\
#######
#a.#Cd#
##...##
##.@.##
##...##
#cB#Ab#
#######
",
        );

        maze.update();

        assert_eq!(
            maze.to_string(),
            "\
#######
#a.#Cd#
##@#@##
#######
##@#@##
#cB#Ab#
#######
"
        );
    }
}
