extern crate piston_window;
extern crate rand;

use piston_window::*;
use rand::Rng;
use std::cell::RefCell;

type Colour = [f32; 4];

const RED: Colour = [1.0, 0.0, 0.0, 1.0];
const GREEN: Colour = [0.0, 1.0, 0.0, 1.0];
const BLUE: Colour = [0.0, 0.0, 1.0, 1.0];
const WHITE: Colour = [1.0; 4];
const BLACK: Colour = [0.0, 0.0, 0.0, 1.0];

const WINDOW_SIZE: i32 = 512;
const PIXEL_SIZE: f64 = 32.0;
const WORLD_SIZE: i32 = WINDOW_SIZE / PIXEL_SIZE as i32;

const WINDOW_WIDTH: f64 = 1500.0;
const WINDOW_HEIGHT: f64 = 900.0;

struct Metrics {
    width: f64,
    height: f64,
}

impl Metrics {
    // ...
}

// Represents shape to be rendered on the board
// Cases are rectangle and circle, with measurement parameters attached for those shapes
#[derive(Clone)]
enum Shape {
    Rectangle { width: f64, height: f64 },
    Circle { radius: f64 },
}

// Player struct
// Parameters:
// shape: shape of character, can currently be circle or rectangle
// size: size of character (width, height)
// location: coordinates of character (x, y)
// color: color of character
#[derive(Clone)]
struct Player<'a> {
    shape: Shape,
    size: (f64, f64),
    location: (f64, f64),
    color: Colour,
    metrics: &'a Metrics,
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

impl<'a> Player<'a> {
    fn render(&self, context: Context, graphics: &mut G2d) {
        match self.shape {
            // Checks shape of player and adds it to board accordingly
            // Currently only options are Circle and Square players
            Shape::Circle { radius } => {
                ellipse(
                    self.color,
                    ellipse::circle(self.location.0, self.location.1, radius),
                    context.transform,
                    graphics,
                );
            }
            Shape::Rectangle { width, height } => {
                rectangle(
                    self.color,
                    [self.location.0, self.location.1, width, height],
                    context.transform,
                    graphics,
                );
            }
        }
    }
}

enum PlayerState {
    Walking,
    Standing,
}

enum SceneryType {
    Tree {
        base_width: f64,
        base_height: f64,
        top_radius: f64,
    },
}

struct Scenery {
    type_: SceneryType,
    location: (f64, f64),
}

#[derive(Copy, Clone)]
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn collides_with_rectangle(self, rect: Rectangle) -> bool {
        // http://www.jeffreythompson.org/collision-detection/circle-rect.php
        let test_x = if self.x < rect.x {
            rect.x
        } else if self.x > rect.x + rect.width {
            rect.x + rect.width
        } else {
            self.x
        };
        let test_y = if self.y < rect.y {
            rect.y
        } else if self.y > rect.y + rect.height {
            rect.y + rect.height
        } else {
            self.y
        };
        let dist_x = self.x - test_x;
        let dist_y = self.y - test_y;
        let distance = ((dist_x * dist_x) + (dist_y * dist_y)).sqrt();
        distance <= self.radius
    }

    fn collides_with_circle(self, circle: Circle) -> bool {
        // http://www.jeffreythompson.org/collision-detection/circle-circle.php
        let dist_x = self.x - circle.x;
        let dist_y = self.y - circle.y;
        let distance = ((dist_x * dist_x) + (dist_y * dist_y)).sqrt();
        distance <= (self.radius + circle.radius)
    }
}

#[derive(Copy, Clone)]
struct Rectangle {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

impl Scenery {
    fn collides(&self, player: &Player<'_>) -> bool {
        match self.type_ {
            SceneryType::Tree {
                base_width,
                base_height,
                top_radius,
            } => match player.shape {
                Shape::Circle { radius } => {
                    let player_circle = Circle {
                        x: player.location.0,
                        y: player.location.1,
                        radius,
                    };
                    let top_circle = Circle {
                        x: self.location.0 + (base_width / 2.0),
                        y: self.location.1 - top_radius + 5.0,
                        radius: top_radius,
                    };
                    let base_rect = Rectangle {
                        x: self.location.0,
                        y: self.location.1,
                        width: base_width,
                        height: base_height,
                    };
                    player_circle.collides_with_rectangle(base_rect)
                        || player_circle.collides_with_circle(top_circle)
                }
                Shape::Rectangle { .. } => {
                    // Temporary placeholder until we add a rectangle player
                    false
                }
            },
        }
    }

    fn render(&self, context: Context, graphics: &mut G2d) {
        match self.type_ {
            SceneryType::Tree {
                base_width,
                base_height,
                top_radius,
            } => {
                rectangle(
                    BLACK,
                    [self.location.0, self.location.1, base_width, base_height],
                    context.transform,
                    graphics,
                );
                let top_x = self.location.0 + (base_width / 2.0);
                let top_y = self.location.1 - top_radius + 5.0;
                ellipse(
                    BLUE,
                    ellipse::circle(top_x, top_y, top_radius),
                    context.transform,
                    graphics,
                );
            }
        }
    }
}

enum GameState {
    Start,
}

struct Game<'a> {
    user_player: Player<'a>,
    other_players: Vec<Player<'a>>,
    scenery: RefCell<Vec<Scenery>>,
    metrics: &'a Metrics,
}

impl<'a> Game<'a> {
    fn display_items(state: GameState, metrics: &'a Metrics) -> Self {
        match state {
            GameState::Start => {
                // Currently just have one player for setup purposes, eventually there should be more and this can be moved
                let player = Player {
                    shape: Shape::Circle { radius: 50.0 },
                    size: (50.0, 50.0),
                    location: (50.0, 50.0),
                    color: RED,
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
    fn render(&self, window: &mut PistonWindow, event: &Event) {
        window.draw_2d(event, |context, graphics, _| {
            clear(GREEN, graphics);
            self.user_player.render(context, graphics);
            for player in &self.other_players {
                player.render(context, graphics);
            }
            for scene in &*self.scenery.borrow() {
                scene.render(context, graphics);
            }
        });
    }

    fn move_player(&self, player: &Player<'_>, movement: (f64, f64)) -> Option<(f64, f64)> {
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
    fn on_press(&self, args: &Button) -> Option<(f64, f64)> {
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
            Key::T => {
                &self.handle_letter(key);
                None
            }
            _ => None,
        }
    }

    fn handle_letter(&self, key: &Key) {
        match key {
            Key::T => {
                //plant tree in random location on board
                let mut rng = rand::thread_rng();
                let ranX = rng.gen_range(100.0, WINDOW_WIDTH - 100.0);
                let ranY = rng.gen_range(100.0, WINDOW_HEIGHT - 100.0);
                let sceneType = SceneryType::Tree {
                    base_width: 45.0,
                    base_height: 120.0,
                    top_radius: 60.0,
                };
                &self.add_scenery((ranX, ranY), sceneType);
            }
            _ => {}
        }
    }

    fn add_scenery(&self, location: (f64, f64), sceneryType: SceneryType) {
        let scenery = Scenery {
            type_: sceneryType,
            location: location,
        };
        self.scenery.borrow_mut().push(scenery);
    }
}

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
