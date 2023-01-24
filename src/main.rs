use macroquad::prelude::*;

struct Ant {
    x: usize,
    y: usize,
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
    fn move_forward(&mut self, board: &mut Vec<Vec<char>>) {

        // Move forward in the direction that the ant is facing.
        match self.direction {
            Direction::North => self.y -= 1,
            Direction::East => self.x += 1,
            Direction::South => self.y += 1,
            Direction::West => self.x -= 1,
        };

        // Turn right if on white square, turn left if on black square, then flip square color.
        if board[self.x][self.y] == 'o' {
            self.turn_right();
            board[self.x][self.y] = 'x';
        } else {
            self.turn_left();
            board[self.x][self.y] = 'o';
        }
        
    }
}

enum Direction {
    North,
    East,
    South,
    West,
}

fn print_board(board: &Vec<Vec<char>>) {
   for row in board {
        for cell in row {
            print!("{} ", cell);
        }
        println!();
    } 
}


fn test() {

    let mut board = vec![vec!['o'; 80]; 80];

    let mut ant = Ant {
        x: 40,
        y: 40,
        direction: Direction::North
    };

    for _i in 0..11149 {
        ant.move_forward(&mut board);
    }

    print_board(&board);
}

const PLAYER_SIZE: Vec2 = const_vec2!([150f32, 40f32]);

struct Player {
    rect: Rect,
}

impl Player {
    pub fn new() -> Self {
       Self {
           rect: Rect::new(
               screen_width() * 0.5f32 - PLAYER_SIZE.x * 0.5f32,
               screen_height() - 100f32,
               PLAYER_SIZE.x,
               PLAYER_SIZE.y,
           ),
       }
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, BLUE);
    }

}

// Side Quest to Make the Breakout Game to Get an Understanding of Macroquad
#[macroquad::main("BREAKOUT")]
async fn main() {

    let player = Player::new();

    loop {
        clear_background(WHITE);
        player.draw();
        next_frame().await
    }
}




















