use rand::Rng;


pub struct Game {
    game_board: Vec<Vec<char>>,
    player_pos: (usize,usize),
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
            player_pos: (0,0),
        }
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

    pub fn update_player_pos(&mut self, new_y: usize, new_x: usize) {
        self.player_pos.0 = new_y;
        self.player_pos.1 = new_x;
    }

    pub fn update_vector_at_pos(&mut self, y_pos: usize, x_pos: usize, new_char: char) {
        self.game_board[y_pos][x_pos] = new_char;
    }

    pub fn update_at_rand_pos(&mut self, char_to_change_to: char) {
        let mut rng = rand::thread_rng();
        let rand_y = rng.gen_range(0, 10);
        let rand_x = rng.gen_range(0, 10);

        self.game_board[rand_y][rand_x] = char_to_change_to;
    }

    pub fn move_player(&mut self, direction: char) {
        let (old_pos_y, old_pos_x) = self.get_player_pos();

        match direction {
            'w' => { 
                let new_pos_y: usize = old_pos_y - 1;
                let new_pos_x: usize = old_pos_x;
                self.update_player_pos(new_pos_y, new_pos_x);

                println!("Move up");
            }

            'd' => {
                let new_pos_y: usize = old_pos_y;
                let new_pos_x: usize = old_pos_x + 1;
                self.update_player_pos(new_pos_y, new_pos_x);

                println!("Move right");
            }

            'a' => {
                let new_pos_y: usize = old_pos_y;
                let new_pos_x: usize = old_pos_x - 1;
                self.update_player_pos(new_pos_y, new_pos_x);

                println!("Move left");
            }

            's' => {
                let new_pos_y: usize = old_pos_y + 1;
                let new_pos_x: usize = old_pos_x;
                self.update_player_pos(new_pos_y, new_pos_x);

                println!("Move down");
            }

            _ => {
                println!("Do nothing");
            }
        }
    }
}

