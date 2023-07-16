use game::{Game, size::Size};

mod game;

fn main() -> Result<(), String> {
    let game = Game::new("Sex 2", Size::new(800, 600));

    return game.run();
}