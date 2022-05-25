use std::time::{Duration};
use std::thread::sleep;

pub mod game_logic;
pub mod entity;
pub mod item;

fn main() {

    let mut game = game_logic::Game::create_board_from_file("src/Boards/Board1.txt".to_string());

    game.update_player_pos(2, 2);

    loop {
        game.print_board();

        let user_input = game_logic::get_user_input();
        game.move_player(user_input);

        sleep(Duration::new(1,0));
        clearscreen::clear().expect("Failed to clear screen");
    }
}
