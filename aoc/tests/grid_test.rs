//
// Integration tests
//

use aoc::Coord;
use aoc::Grid;

#[test]
fn grid_parse_char() {
    let g = Grid::<char>::from(
        "\
#####
#a..#
#...#
#####
",
    );

    assert_eq!(g.width(), 5);
    assert_eq!(g.height(), 4);

    assert_eq!(g[Coord::new(1, 1)], 'a');
}

#[test]
fn grid_parse_u8() {
    let g = Grid::<u8>::from(
        "\
#####
#a..#
#...#
#####
",
    );

    assert_eq!(g.width(), 5);
    assert_eq!(g.height(), 4);

    assert_eq!(g[Coord::new(1, 1)], b'a');
}

#[test]
fn grid_set_get() {
    let mut grid = Grid::<char>::with_size(5, 5, ' ', '#');

    grid[(0, 0)] = 'A';
    grid[Coord::new(1, 0)] = 'B';

    assert_eq!(grid[Coord::new(0, 0)], 'A');
    assert_eq!(grid[(1, 0)], 'B');

    assert!(grid.is_in_grid(Coord::new(2, 2)).is_some());
    assert!(grid.is_in_grid(Coord::new(5, 0)).is_none());
    assert!(grid.is_in_grid(Coord::new(0, -1)).is_none());
}

#[test]
fn grid_iter_spec() {
    let grid: Grid<char> = Grid::<char>::with_size(5, 5, ' ', '#');

    assert_eq!(grid.iter_cells().count(), 5 * 5);

    // upper left corner has 2 orthogonal cells: (1,0) (0,1)
    assert_eq!(grid.iter_directions(Coord::ZERO).count(), 2);

    // upper left corner has 3 adjacent cells: (1,0) (1,1) (0,1)
    assert_eq!(grid.iter_neighbors(Coord::ZERO).count(), 3);

    // 812
    // 7 3
    // 654
    let grid = Grid::<char>::with_size(3, 3, ' ', '#');
    let mut square = Grid::<char>::with_size(3, 3, ' ', '#');
    for (c, p) in ('1'..).zip(grid.iter_neighbors(Coord::new(1, 1))) {
        square[p] = c;
    }
    assert_eq!(format!("{square}"), "812\n7 3\n654\n");

    //  N
    // W E
    //  S
    let grid = Grid::<char>::with_size(3, 3, ' ', '#');
    let mut plus = Grid::<char>::with_size(3, 3, ' ', '#');
    for (&c, (_, p)) in ['N', 'E', 'S', 'W']
        .iter()
        .zip(grid.iter_directions(Coord::new(1, 1)))
    {
        plus[p] = c;
    }
    assert_eq!(format!("{plus}"), " N \nW E\n S \n");
}

#[test]
fn grid_iter() {
    let grid = Grid::<char>::from("AB\nCD");
    for (pos, &c) in grid.iter().skip(2) {
        assert_eq!(pos, Coord::new(0, 1)); // enumeration starts at the second line since we skip 2 eleemnts
        assert_eq!(c, 'C');
        break;
    }

    let mut abcd = String::new();
    for (_, &c) in &grid {
        abcd.push(c);
    }
    assert_eq!(abcd, "ABCD");
}

#[test]
fn grid_iter_mut() {
    let mut grid = Grid::<char>::from("AB\nCD");

    for (_, c) in grid.iter_mut() {
        *c = c.to_ascii_lowercase();
    }

    let abcd: String = grid.iter().map(|(_, c)| c).collect();
    assert_eq!(abcd, "abcd");

    for (_, c) in &mut grid {
        *c = c.to_ascii_uppercase();
    }

    let abcd: String = grid.iter().map(|(_, c)| c).collect();
    assert_eq!(abcd, "ABCD");
}
