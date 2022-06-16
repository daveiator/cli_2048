use cli_2048::{Grid, Direction};
use std::io;

fn main() {
    let mut grid = Grid::new(4, 4);
    println!("{grid}");

    loop {
        println!("Enter your move:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input == "q" {
            break;
        }
        let direction = match input {
            "a" => Direction::LEFT,
            "d" => Direction::RIGHT,
            "w" => Direction::UP,
            "s" => Direction::DOWN,
            _ => {
                println!("Invalid input");
                continue;
            }
        };
        grid = grid.slide(direction).unwrap();
        println!("{grid}");
    }
}
