use rand::Rng;
use std::io;
use std::fs::File;
use std::io::{BufReader, BufRead};

pub struct Game {
    game_board: Vec<Vec<char>>,
    board_height: usize,
    board_width: usize,
    player_pos: (usize,usize),
}

pub fn get_user_input() -> char {
    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    user_input.chars().nth(0).unwrap()
}


impl Game {
    pub fn new(height: usize, width: usize) -> Game {
        let mut outer_vector = Vec::new();

        for _ in 0..height {
            let inner_vector = vec!['0'; width];

            outer_vector.push(inner_vector);
        }

        Game {
            game_board: outer_vector,
            board_height: height,
            board_width: width,
            player_pos: (0,0),
        }
    }
    
    pub fn create_board_from_file(filename: String) -> Game {
        let file = File::open(filename).expect("Issue reading file");
        let buf  = BufReader::new(file);

        let lines_of_file: Vec<String> = buf.lines()
                                .map(|l| l.expect("Could not parse line"))
                                .collect();

        let mut outer_vector: Vec<Vec<char>> = Vec::new();

        for line in &lines_of_file {
            let mut inner_vector = vec!['0'; 0];

            for char in line.chars() {
                inner_vector.push(char);
            }

            outer_vector.push(inner_vector);
        } 

        Game {
            game_board: outer_vector,
            board_height: lines_of_file.len(),
            board_width: lines_of_file[0].len(),
            player_pos: (0,0),
        }
    }

    pub fn bounds_check(&self, new_pos_y: usize, new_pos_x: usize, y_inc: i16, x_inc: i16) -> bool {
        let y = (new_pos_y as i16) + y_inc;
        let x = (new_pos_x as i16) + x_inc;
    
        y >= 0 && y < self.board_height.try_into().unwrap() && x >= 0 && x < self.board_width.try_into().unwrap()
    }
    
    pub fn print_board(&self) {
        for index in 0..self.game_board.len() {
            for board_char in &self.game_board[index] {
                print!("{}", board_char);
            }
            println!();
        }
    }

    pub fn get_player_pos(&self) -> (usize, usize) {
        self.player_pos
    }

    pub fn get_char_at_pos(&self, y_pos: usize, x_pos: usize) -> char{
        self.game_board[y_pos][x_pos]
    }

    pub fn update_vector_at_pos(&mut self, y_pos: usize, x_pos: usize, new_char: char) {
        self.game_board[y_pos][x_pos] = new_char;
    }

    pub fn update_player_pos(&mut self, new_y: usize, new_x: usize) {
        self.update_vector_at_pos(self.player_pos.0, self.player_pos.1, ' ');

        self.player_pos.0 = new_y;
        self.player_pos.1 = new_x;

        self.update_vector_at_pos(new_y, new_x, 'p');
    }

    pub fn update_at_rand_pos(&mut self, char_to_change_to: char) {
        let mut rng = rand::thread_rng();
        let rand_y = rng.gen_range(0, self.board_height);
        let rand_x = rng.gen_range(0, self.board_width);

        self.game_board[rand_y][rand_x] = char_to_change_to;
    }

    pub fn move_player(&mut self, direction: char) {
        let (old_pos_y, old_pos_x) = self.get_player_pos();

        match direction {
            'w' => { 
                let bounds_flag = self.bounds_check(old_pos_y, old_pos_x, -1, 0);
                if bounds_flag { self.update_player_pos(old_pos_y - 1, old_pos_x) }

                println!("Move up");
            }

            'd' => {
                let bounds_flag = self.bounds_check(old_pos_y, old_pos_x, 0, 1);
                if bounds_flag { self.update_player_pos(old_pos_y, old_pos_x + 1) }

                println!("Move right");
            }

            'a' => {
                let bounds_flag = self.bounds_check(old_pos_y, old_pos_x, 0, -1);
                if bounds_flag { self.update_player_pos(old_pos_y, old_pos_x - 1) }

                println!("Move left");
            }

            's' => {
                let bounds_flag = self.bounds_check(old_pos_y, old_pos_x, 1, 0);
                if bounds_flag { self.update_player_pos(old_pos_y + 1, old_pos_x) }

                println!("Move down");
            }

            _ => {
                println!("Do nothing");
            }
        }
    }

}

