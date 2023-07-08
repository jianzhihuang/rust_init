use std::error::Error;
use std::io::{self, Write};
use std::time::Duration;

use crossterm::{cursor, execute, terminal};
use rand::Rng;
use termion::event::Key;
use termion::input::TermRead;

const BOARD_WIDTH: i32 = 20;
const BOARD_HEIGHT: i32 = 10;
const FRAME_RATE: u64 = 250;
const SNAKE_START_LENGTH: i32 = 3;

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

struct Snake {
    direction: Direction,
    body: Vec<Point>,
}

fn spawn_food(food: &mut Point, snake: &[Point]) {
    let mut rng = rand::thread_rng();
    loop {
        food.x = rng.gen_range(0..BOARD_WIDTH);
        food.y = rng.gen_range(0..BOARD_HEIGHT);
        if !snake.iter().any(|p| p.x == food.x && p.y == food.y) {
            break;
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Set up terminal
    terminal::enable_raw_mode()?;
    execute!(io::stdout(), terminal::EnterAlternateScreen)?;
    execute!(io::stdout(), cursor::Hide)?;

    // Set up game state
    let mut running = true;
    let mut snake = Snake {
        direction: Direction::Right,
        body: vec![
            Point { x: 3, y: 2 },
            Point { x: 2, y: 2 },
            Point { x: 1, y: 2 },
        ],
    };
    let mut food = Point { x: 7, y: 7 };
    let mut last_update = std::time::Instant::now();

    // Game loop
    while running {
        // Handle input
        if let Some(key) = io::stdin().keys().next() {
            match key? {
                Key::Char('q') => {
                    running = false;
                }
                Key::Left => {
                    if snake.direction != Direction::Right {
                        snake.direction = Direction::Left;
                    }
                }
                Key::Right => {
                    if snake.direction != Direction::Left {
                        snake.direction = Direction::Right;
                    }
                }
                Key::Up => {
                    if snake.direction != Direction::Down {
                        snake.direction = Direction::Up;
                    }
                }
                Key::Down => {
                    if snake.direction != Direction::Up {
                        snake.direction = Direction::Down;
                    }
                }
                _ => {}
            }
        }

        // Update game state
        if last_update.elapsed().as_millis() >= u128::from(FRAME_RATE) {
            // Move snake
            let head = snake.body.last().unwrap().clone();
            let new_head = match snake.direction {
                Direction::Up => Point {
                    x: head.x,
                    y: head.y - 1,
                },
                Direction::Down => Point {
                    x: head.x,
                    y: head.y + 1,
                },
                Direction::Left => Point {
                    x: head.x - 1,
                    y: head.y,
                },
                Direction::Right => Point {
                    x: head.x + 1,
                    y: head.y,
                },
            };
            snake.body.push(new_head);
            snake.body.remove(0);

            // Check for collisions
            if new_head.x < 0
                || new_head.x >= BOARD_WIDTH
                || new_head.y < 0
                || new_head.y >= BOARD_HEIGHT
            {
                running = false;
            }
            for segment in &snake.body[..snake.body.len() - 1] {
                if new_head.x == segment.x && new_head.y == segment.y {
                    running = false;
                }
            }
            if let Some(head) = snake.body.last().cloned() {
                if head == food {
                    snake.body.push(new_head);
                    spawn_food(&mut food, &snake.body);
                }
            }
                snake.body.insert(
                    0,
                    Point {
                        x: food.x,
                        y: food.y,
                    },
                );
                spawn_food(&mut food, &snake.body);
            }

            // Render game state
            execute!(io::stdout(), cursor::MoveTo(0, 0))?;
            for y in 0..BOARD_HEIGHT {
                for x in 0..BOARD_WIDTH {
                    let c = if x == food.x && y == food.y {
                        'o'
                    } else if snake.body.iter().any(|p| p.x == x && p.y == y) {
                        'x'
                    } else {
                        ' '
                    };
                    print!("{}", c);
                }
                println!();
            }
            println!("Score: {}", snake.body.len() - SNAKE_START_LENGTH as usize);
            last_update = std::time::Instant::now();
        }
    }

    // Clean up terminal
    terminal::disable_raw_mode()?;
    execute!(io::stdout(), terminal::LeaveAlternateScreen)?;
    execute!(io::stdout(), cursor::Show)?;
    Ok(())
}