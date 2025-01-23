//
// Integration tests
//

use aoc::GridU;

const R: &str = concat!(
    "###### \n",
    "###### \n",
    "#     #\n",
    "#     #\n",
    "#     #\n",
    "#     #\n",
    "###### \n",
    "###### \n",
    "#   #  \n",
    "#   #  \n",
    "#    # \n",
    "#    # \n",
    "#     #\n",
    "#     #\n",
);

const G: &str = concat!(
    "                       \n",
    "         GGGGGGGGGGGGG \n",
    "      GGG::::::::::::G \n",
    "    GG:::::::::::::::G \n",
    "   G:::::GGGGGGGG::::G \n",
    "  G:::::G       GGGGGG \n",
    " G:::::G               \n",
    " G:::::G               \n",
    " G:::::G    GGGGGGGGGG \n",
    " G:::::G    G::::::::G \n",
    " G:::::G    GGGGG::::G \n",
    " G:::::G        G::::G \n",
    "  G:::::G       G::::G \n",
    "   G:::::GGGGGGGG::::G \n",
    "    GG:::::::::::::::G \n",
    "      GGG::::::GGG:::G \n",
    "         GGGGGG   GGGG \n",
    "                       \n",
);

#[test]
fn gridu_rotation() {
    let grid = GridU::<u8>::parse(R);

    println!("{grid}");

    println!("clockwise");
    println!("{}", grid.rotate_clockwise());
    println!("clockwise x2");
    println!("{}", grid.rotate_clockwise().rotate_clockwise());

    println!("counterclockwise");
    println!("{}", grid.rotate_counterclockwise());
    println!("counterclockwise x2");
    println!(
        "{}",
        grid.rotate_counterclockwise().rotate_counterclockwise()
    );

    assert_eq!(
        grid.to_string(),
        grid.rotate_clockwise()
            .rotate_clockwise()
            .rotate_clockwise()
            .rotate_clockwise()
            .to_string()
    );

    assert_eq!(
        grid.to_string(),
        grid.rotate_counterclockwise()
            .rotate_counterclockwise()
            .rotate_counterclockwise()
            .rotate_counterclockwise()
            .to_string()
    );

    assert_eq!(
        grid.rotate_counterclockwise()
            .rotate_counterclockwise()
            .to_string(),
        grid.rotate_clockwise().rotate_clockwise().to_string()
    );
}

#[test]
fn gridu_inversion() {
    let grid = GridU::<u8>::parse(R);

    println!("{grid}");

    println!("flip_horizontal");
    println!("{}", grid.flip_horizontal());

    println!("flip_vertical");
    println!("{}", grid.flip_vertical());

    println!("flip both");
    println!("{}", grid.flip_vertical().flip_horizontal());

    assert_eq!(
        grid.to_string(),
        grid.flip_vertical().flip_vertical().to_string()
    );

    assert_eq!(
        grid.to_string(),
        grid.flip_horizontal().flip_horizontal().to_string()
    );

    assert_eq!(
        grid.flip_horizontal().flip_vertical().to_string(),
        grid.flip_vertical().flip_horizontal().to_string()
    );

    assert_eq!(
        grid.to_string(),
        grid.flip_horizontal()
            .flip_vertical()
            .rotate_clockwise()
            .rotate_clockwise()
            .to_string()
    );
}

#[test]
fn gridu_dispositions() {
    let grid = GridU::<u8>::parse(G);

    for r in grid.iter_pos() {
        println!("{r}");
    }
}
