use game_of_life::Game;
use rand::Rng;
use std::thread;
use std::time::Duration;

const GAME_WIDTH: usize = 30;
const GAME_HEIGHT: usize = 20;

fn main() {
    let mut game = Game {
        width: GAME_WIDTH,
        height: GAME_HEIGHT,
        world: vec![vec![0; GAME_WIDTH]; GAME_HEIGHT],
    };

    // fill the vector that represents the game randomly with 0 and 1.
    for i in 0..game.height {
        for j in 0..game.width {
            let random = rand::thread_rng().gen_range(0..4);
            game.world[i][j] = if random == 0 { 1 } else { 0 }
        }
    }

    loop {
        // clear the screen
        print!("{}[2J", 27 as char);
        // draw the game
        game.draw();
        // sleep for 1 second
        thread::sleep(Duration::new(1, 0));

        game.evolution();
    }
}
