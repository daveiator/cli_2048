use cli_2048::{Grid, Direction};
use crossterm::{Result, event::{read,Event}};
use std::env;

fn main() -> Result<()>{
    let args: Vec<String> = env::args().collect();
    let mut grid = Grid::new(4, 4);
    if args.len() > 2 {
        grid = Grid::new(
            args[1].parse::<usize>().unwrap_or_else(|_| {
                println!("Invalid arguments!");
                std::process::exit(1);
            }), args[2].parse::<usize>().unwrap_or_else(|_| {
                println!("Invalid arguments!");
                std::process::exit(1);
            }));
    }
    
    println!("{grid}");

    loop {
        // `read()` blocks until an `Event` is available
        match read()? {
            Event::Key(event) => {
                let input = format!("{:?}", event.code);
                let input = input.to_string();
                let input = input.split("'").collect::<Vec<&str>>()[1];
                if input == "q" {
                    println!("Quitting...");
                    std::process::exit(0);
                }
                let direction = match input {
                    "a" => Direction::LEFT,
                    "d" => Direction::RIGHT,
                    "w" => Direction::UP,
                    "s" => Direction::DOWN,
                    _ => {
                        println!("Invalid input: {}", input);
                        println!("{grid}");
                        continue;
                    }
                };
                grid = match grid.slide(direction) {
                    Ok(grid) => grid,
                    Err(e) => {
                        match e {
                            "no more options"=> {
                                println!("Game over! No more options!");
                                std::process::exit(0);
                            }
                            _ => {
                                panic!("{}", e);
                            }
                        }
                    }
                };
                println!("{grid}");
            },
            _ => {},
        }
    }
}
