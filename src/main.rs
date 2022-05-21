use std::time::{Duration};
use std::thread::sleep;

pub mod game_logic;
pub mod entity;
pub mod item;

fn main() {
    let mut game = game_logic::Game::new(10, 10);

    game.update_vector_at_pos(5, 5, 'b');


    loop {
        item::get_weapon_from_string("abc 123".to_string());
        game.update_at_rand_pos(' ');
        game.move_player('d');
        game.print_board();
        sleep(Duration::new(1,0));
        clearscreen::clear().expect("Failed to clear screen");
    }
}
