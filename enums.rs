enum Direction { //simple enum example
    EAST,
    WEST,
    NORTH,
    SOUTH,
}
fn main() {
    println!("{}", direction(Direction::EAST))
}

fn direction(dir: Direction) -> String {
    match dir {
        Direction::EAST  => String::from("East"),
        Direction::WEST  => String::from("West"),
        Direction::NORTH => String::from("North"),
        Direction::SOUTH => String::from("South"),
    }
}