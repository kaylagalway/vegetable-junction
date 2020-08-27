use std::cell::RefCell;

use piston_window::{clear, Button, Event, Key, PistonWindow};
use rand::Rng;

use crate::colors;
use crate::metrics::{Metrics, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::player::Player;
use crate::scenery::{Scenery, SceneryType};
use crate::shapes::Shape;

pub enum GameState {
    Start,
}

pub struct Game<'a> {
    pub user_player: Player<'a>,
    pub other_players: Vec<Player<'a>>,
    pub scenery: RefCell<Vec<Scenery>>,
    pub metrics: &'a Metrics,
}

impl<'a> Game<'a> {
    pub fn display_items(state: GameState, metrics: &'a Metrics) -> Self {
        match state {
            GameState::Start => {
                // Currently just have one player for setup purposes, eventually there should be more and this can be moved
                let player = Player {
                    shape: Shape::Circle { radius: 50.0 },
                    size: (50.0, 50.0),
                    location: (50.0, 50.0),
                    color: colors::RED,
                    metrics,
                };
                let tree = Scenery {
                    type_: SceneryType::Tree {
                        base_width: 45.0,
                        base_height: 120.0,
                        top_radius: 60.0,
                    },
                    location: (500.0, 600.0),
                };
                return Game {
                    user_player: player,
                    other_players: Vec::new(),
                    scenery: RefCell::new(vec![tree]),
                    metrics,
                };
            }
        }
    }

    // Render function checks arguments and events and changes display based on this
    pub fn render(&self, window: &mut PistonWindow, event: &Event) {
        window.draw_2d(event, |context, graphics, _| {
            clear(colors::GREEN, graphics);
            self.user_player.render(context, graphics);
            for player in &self.other_players {
                player.render(context, graphics);
            }
            for scene in &*self.scenery.borrow() {
                scene.render(context, graphics);
            }
        });
    }

    pub fn move_player(&self, player: &Player<'_>, movement: (f64, f64)) -> Option<(f64, f64)> {
        let (min_x, max_x) = match player.shape {
            Shape::Circle { radius } => (radius, self.metrics.width - radius),
            Shape::Rectangle { .. } => unimplemented!("Add later"),
        };
        let (min_y, max_y) = match player.shape {
            Shape::Circle { radius } => (radius, self.metrics.height - radius),
            Shape::Rectangle { .. } => unimplemented!("Add later"),
        };
        let x = clamp(
            player.location.0 + (movement.0 * player.size.0),
            min_x,
            max_x,
        );
        let y = clamp(
            player.location.1 + (movement.1 * player.size.1),
            min_y,
            max_y,
        );
        let future_player = Player {
            location: (x, y),
            ..player.clone()
        };
        for scenery in &*self.scenery.borrow() {
            if scenery.collides(&future_player) {
                return None;
            }
        }
        Some((x, y))
    }

    // Handles all press interactions and returns Tuple of (x, y) coordinate changes if character requires movement
    // In the future could return an enum of character reaction, if something different than key arrows was pressed
    pub fn on_press(&self, args: &Button) -> Option<(f64, f64)> {
        println!("Entered on_press function");
        match args {
            Button::Keyboard(args) => self.on_key(args),
            _ => None,
        }
    }

    // Specifically handles keyboard presses and returns tuple of (x, y) coordinate changes for character movement
    // Currently only handling arrow keys
    fn on_key(&self, key: &Key) -> Option<(f64, f64)> {
        println!("Entered on_key function");
        match key {
            Key::Right => Some((1.0, 0.0)),
            Key::Left => Some((-1.0, 0.0)),
            Key::Up => Some((0.0, -1.0)),
            Key::Down => Some((0.0, 1.0)),
            _ => {
                if (Key::A..=Key::Z).contains(key) {
                    self.handle_letter(key)
                }
                None
            }
        }
    }

    fn handle_letter(&self, key: &Key) {
        match key {
            Key::T => {
                //plant tree in random location on board
                let scene_type = SceneryType::Tree {
                    base_width: 45.0,
                    base_height: 120.0,
                    top_radius: 60.0,
                };
                &self.make_scenery(scene_type);
            }
            _ => {}
        }
    }

    fn make_scenery(&self, scenery_type: SceneryType) {
        let mut rng = rand::thread_rng();
        let ran_x = rng.gen_range(100.0, WINDOW_WIDTH - 100.0);
        let ran_y = rng.gen_range(100.0, WINDOW_HEIGHT - 100.0);
        &self.add_scenery((ran_x, ran_y), scenery_type);
    }

    fn add_scenery(&self, location: (f64, f64), scenery_type: SceneryType) {
        let scenery = Scenery {
            type_: scenery_type,
            location: location,
        };
        self.scenery.borrow_mut().push(scenery);
    }
}

fn clamp(value: f64, min: f64, max: f64) -> f64 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}
