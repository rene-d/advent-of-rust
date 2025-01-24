use aoc::Coord;

#[test]
fn coord_add() {
    let a = Coord::new(1, 2);
    let b = Coord::new(-1, 2);
    let c = a + b;

    assert_eq!(c.x, 0);
    assert_eq!(c.y, 4);
}

#[test]
fn coord_add_assign() {
    let mut a = Coord::new(1, 2);
    let b = Coord::new(-1, 2);

    a += b;

    assert_eq!(a.x, 0);
    assert_eq!(a.y, 4);
}

#[test]
fn coord_mul_coord_i32() {
    let a = Coord::new(1, -2);
    let c = a * 10;

    assert_eq!(c.x, 10);
    assert_eq!(c.y, -20);
}

#[test]
fn coord_mul_i32_coord() {
    let a = Coord::new(1, -2);
    let c = 10 * a;

    assert_eq!(c.x, 10);
    assert_eq!(c.y, -20);
}

#[test]
fn coord_mul_assign() {
    let mut a = Coord::new(1, -2);
    a *= 10;

    assert_eq!(a.x, 10);
    assert_eq!(a.y, -20);
}

#[test]
fn coord_sub() {
    let a = Coord::new(1, 2);
    let b = Coord::new(-1, 2);
    let c = a - b;

    assert_eq!(c.x, 2);
    assert_eq!(c.y, 0);
}

#[test]
fn coord_sub_assign() {
    let mut a = Coord::new(1, 2);
    let b = Coord::new(-1, 2);

    a -= b;

    assert_eq!(a.x, 2);
    assert_eq!(a.y, 0);
}

#[test]
fn coord_clockwise() {
    let mut a = Coord::RIGHT;

    a = a.clockwise();
    assert_eq!(a, Coord::DOWN);

    a = a.clockwise();
    assert_eq!(a, Coord::LEFT);

    a = a.clockwise();
    assert_eq!(a, Coord::UP);

    a = a.clockwise();
    assert_eq!(a, Coord::RIGHT);
}

#[test]
fn coord_counter_clockwise() {
    let mut a = Coord::RIGHT;

    a = a.counter_clockwise();
    assert_eq!(a, Coord::UP);

    a = a.counter_clockwise();
    assert_eq!(a, Coord::LEFT);

    a = a.counter_clockwise();
    assert_eq!(a, Coord::DOWN);

    a = a.counter_clockwise();
    assert_eq!(a, Coord::RIGHT);
}

#[test]
fn coord_from() {
    let a = Coord::from('^');
    assert_eq!(a, Coord::UP);

    let a = Coord::from(b'D');
    assert_eq!(a, Coord::DOWN);
}
