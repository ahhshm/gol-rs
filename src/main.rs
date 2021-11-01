use rand::Rng;
use std::thread;
use std::time::Duration;

// The reason that I chose usize is that indexing vectors in possible just with usize
// so we don't need to cast it to usize every time
const GAME_WIDTH: usize = 30;
const GAME_HEIGHT: usize = 20;

struct Game {
    width: usize,
    height: usize,
    world: Vec<Vec<u8>>,
}

impl Game {
    fn draw(&self) -> () {
        for line in &self.world {
            println!(
                "{}",
                line.iter()
                    .map(|l| {
                        if l == &1 {
                            return "* ".to_string();
                        } else {
                            return "  ".to_string();
                        };
                    })
                    .collect::<String>()
            );
        }
    }

    fn evolution(&mut self) {
        let operations: [[i8; 2]; 8] = [
            [0, 1],
            [0, -1],
            [1, 0],
            [-1, 0],
            [1, 1],
            [1, -1],
            [-1, 1],
            [-1, -1],
        ];

        let mut new = self.world.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let mut lives = 0;

                // count the number of live neighbors
                for op in operations {
                    let new_row = row as i8 + op[0];
                    let new_col = col as i8 + op[1];

                    match self.world.get(new_row as usize) {
                        Some(row) => match row.get(new_col as usize) {
                            Some(item) => {
                                if item == &1 {
                                    lives += 1;
                                }
                            }
                            None => {}
                        },
                        None => {}
                    };
                }

                // apply the rules based on the live neighbor's
                // rules:
                //     1. Any live cell with two or three live neighbors survives.
                //     2. Any dead cell with three live neighbors becomes a live cell.
                //     3. All other live cells die in the next generation. Similarly, all other dead cells stay dead.
                if lives == 3 || lives == 2 && self.world[row][col] == 1 {
                    new[row][col] = 1;
                } else {
                    new[row][col] = 0;
                };
            }
        }

        self.world = new;
    }
}

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
