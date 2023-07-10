use game::{Game, size::Size};

mod game;

fn main() -> Result<(), String> {
    let game = Game::new("Foo", Size::new(800, 600));

    return game.run();
}