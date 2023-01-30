extern crate rand;
use macroquad::prelude::*;
use rand::random;
use std::process;

// Langton's Ant Struct
struct Ant {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Ant {

    // Turns the ant 90 degrees to the right.
    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
    }

    // Turns the ant 90 degrees to the left.
    fn turn_left(&mut self) {
            self.direction = match self.direction {
                Direction::North => Direction::West,
                Direction::East => Direction::North,
                Direction::South => Direction::East,
                Direction::West => Direction::South,
            };
        }

    // Moves the ant forward, turning appropriately depending on the color of the square.
    // Move Forward -> Turn Ant -> Flip Square Color
    fn move_forward(&mut self, grid: &mut Vec<Vec<i32>>) {

        // Move forward in the direction that the ant is facing.
        match self.direction {
            Direction::North => self.y -= 1,
            Direction::East => self.x += 1,
            Direction::South => self.y += 1,
            Direction::West => self.x -= 1,
        };

        // Wrap around the grid if out of bounds.
        if self.x >= grid.len() as i32 {
            self.x = 0;
        } else if self.x < 0 {
            self.x = (grid.len() - 1) as i32;
        }
        if self.y >= grid[0].len() as i32 {
            self.y = 0;
        } else if self.y < 0 {
            self.y = (grid[0].len() - 1) as i32;
        }

        // Turn right if on white square, turn left if on black square, then flip square color.
        if grid[self.x as usize][self.y as usize] == 0 {
            self.turn_right();
            grid[self.x as usize][self.y as usize] = 1;
        } else {
            self.turn_left();
            grid[self.x as usize][self.y as usize] = 0;
        }
    }
}

enum Direction {
    North,
    East,
    South,
    West,
}

fn window_conf() -> Conf {
        Conf {
            window_title: "Langton's Antfarm".to_owned(),
            fullscreen: true,
            ..Default::default()
        }
}

// Draw Grid
fn draw_grid(square_count_x: i32, square_count_y: i32, width: i32, height: i32, size: i32) {
    for n in 1..width {
       draw_line((n * size) as f32, 0 as f32, (n * size) as f32, height as f32, 1f32, BLACK);
    }

    for n in 1..height {
        draw_line(0 as f32, (n * size) as f32, width as f32, (n * size) as f32, 1f32, BLACK);
    }
}

fn color_squares(grid: &mut Vec<Vec<i32>>) {
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            if grid[x][y] == 0 {
                draw_rectangle((x * 10) as f32, (y * 10) as f32, 10f32, 10f32, WHITE);
            } else {
                draw_rectangle((x * 10) as f32, (y * 10) as f32, 10f32, 10f32, BLACK);
            }
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    // Initialize Grid Information.
    const SQUARE_SIZE: i32 = 10;
    let screen_width: i32 = screen_width() as i32;
    let screen_height: i32 = screen_height() as i32;
    let square_count_width: i32 = screen_width / SQUARE_SIZE;
    let square_count_height: i32 = screen_height / SQUARE_SIZE;

    // Initialize Board.
    let mut grid = vec![vec![0; square_count_height as usize]; square_count_width as usize];

    // Initialize 5 ants in random locations with random directions.
    let mut ants = Vec::new();
    for _ in 0..5 {
        ants.push(Ant {
            x: random::<i32>() % square_count_width,
            y: random::<i32>() % square_count_height,
            direction: match random::<i32>() % 4 {
                0 => Direction::North,
                1 => Direction::East,
                2 => Direction::South,
                3 => Direction::West,
                _ => Direction::North,
            }
        });
    r

    loop {
        clear_background(WHITE);

        // Move ants forward.
        for ant in &mut ants {
            ant.move_forward(&mut grid);
        }

        // Color squares.
        color_squares(&mut grid);

        // Draw grid lines.
        draw_grid(square_count_width, square_count_height, screen_width, screen_height, SQUARE_SIZE);

        next_frame().await
    }
}