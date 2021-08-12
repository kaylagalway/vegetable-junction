mod colors;
mod game;
mod metrics;
mod player;
mod scenery;
mod shapes;

use game::Game;
use game::GameState;
use metrics::{Metrics, WINDOW_HEIGHT, WINDOW_WIDTH};
use piston_window::{PistonWindow, PressEvent, RenderEvent, WindowSettings};

fn main() {
    // Piston game window initialization
    let metrics = Metrics {
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
    };
    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", [metrics.width, metrics.height])
            .exit_on_esc(true)
            .build()
            .unwrap();

    //Setup new game
    let mut game = Game::display_items(GameState::Start, &metrics);

    // Runloop of constant events sent from piston game engine
    while let Some(e) = window.next() {
        // This will be consistently called on the event run loop
        if let Some(_) = e.render_args() {
            // Render consistently checks for player/game updates and renders the screen layout accordingly
            game.render(&mut window, &e);
        }

        // This checks for user interaction and presses that may adjust the graphics
        if let Some(args) = e.press_args() {
            if let Some(movement) = game.on_press(&args) {
                println!("Key registered as arrow key, movement: {:?}", movement);
                println!(
                    "movement successfully unwrapped, current play location: {:?}",
                    game.user_player.location
                );
                if let Some(new_location) = game.move_player(&game.user_player, movement) {
                    game.user_player.location = new_location;
                }
                println!(
                    "movement changed location. new location: {:?}",
                    game.user_player.location
                );
            }
        }
    }
}

askdhfjasdfaskldjflk