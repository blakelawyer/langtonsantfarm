use colored::Colorize;

struct Ant {
    x: usize,
    y: usize,
    direction: Direction,
}

impl Ant {

    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
    }

    fn turn_left(&mut self) {
            self.direction = match self.direction {
                Direction::North => Direction::West,
                Direction::East => Direction::North,
                Direction::South => Direction::East,
                Direction::West => Direction::South,
            };
        }

    // Moves the ant forward, turning appropriately depending on the color of the square.
    fn move_forward(&mut self, board: &mut Vec<Vec<char>>) {
        // Move Forward -> Turn Ant -> Flip Square Color

        // Move forward in the direction that the ant is facing.
        match self.direction {
            Direction::North => self.y -= 1,
            Direction::East => self.x += 1,
            Direction::South => self.y += 1,
            Direction::West => self.x -= 1,
        };

        // Turn right if on white square, turn left if on black square, then flip square color.
        if(board[self.x][self.y] == 'o') {
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
            if *cell == 'x' {
                print!("{} ", format!("x").red());
            } else {
                print!("{} ", cell);
            }
        }
        println!();
    } 
}


fn main() {

    let mut board = vec![vec!['o'; 80]; 80];

    let mut ant = Ant {
        x: 40,
        y: 40,
        direction: Direction::North
    };

    for i in 0..11149 {
        ant.move_forward(&mut board);
    }

    print_board(&board);
}




















