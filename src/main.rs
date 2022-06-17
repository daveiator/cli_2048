use cli_2048::{Grid, Direction};
use crossterm::{execute, Result, event::{read,Event,KeyCode}, terminal};
use std::env;
use std::io::{stdout};

fn main() -> Result<()>{
    let args: Vec<String> = env::args().collect();

    let mut grid = Grid::new(4, 4);

    match args.len() {
        1 => {}
        3 => {
            //grid size
            grid = Grid::new(
                args[1].parse::<usize>().unwrap_or_else(|_| {
                    println!("Invalid arguments!");
                    std::process::exit(1);
                }), args[2].parse::<usize>().unwrap_or_else(|_| {
                    println!("Invalid arguments!");
                    std::process::exit(1);
                })
            );
        }
        _ => {
            println!("Invalid arguments!");
            std::process::exit(1);
        }
    }
    execute!(
        stdout(),
        terminal::SetTitle("2048"),
    ).unwrap();
    
    println!("{grid}");
    loop {
        match read()? {
            Event::Key(event) => {
                let input = event.code;
                // let input = input.to_string();
                //println!("{}", input);

                if input == KeyCode::Char('q') {
                    println!("Quitting...");
                    std::process::exit(0);
                }
                let direction = match input {
                    KeyCode::Char('a') | KeyCode::Left => Direction::LEFT,
                    KeyCode::Char('d') | KeyCode::Right => Direction::RIGHT,
                    KeyCode::Char('w') | KeyCode::Up => Direction::UP,
                    KeyCode::Char('s') | KeyCode::Down => Direction::DOWN,
                    _ => {
                        println!("Invalid input!");
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
