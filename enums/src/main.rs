enum Direction{
    North,
    East,
    South,
    West,
}

fn main(){
    let my_direction = Direction::East;
    let ans = move_around(my_direction);

    println!("Player Moves : {}", ans);
}

fn move_around(direction: Direction) -> String{
    match direction {
        Direction::North => String::from("North"),
        Direction::East => String::from("East"),
        Direction::West => String::from("West"),
        Direction::South => String::from("South"),
    }
}