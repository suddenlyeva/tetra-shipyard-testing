mod game;
mod test;
use test::*;
use game::*;

fn main() {
    Game::new("Test", 1280, 720).launch(test_state());
}