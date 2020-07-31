extern crate piston_window;

use piston_window::*;

type Colour = [f32; 4];

const RED: Colour = [1.0, 0.0, 0.0, 1.0];
const GREEN: Colour = [0.0, 1.0, 0.0, 1.0];
const BLUE: Colour = [0.0, 0.0, 1.0, 1.0];
const WHITE: Colour = [1.0; 4];
const BLACK: Colour = [0.0, 0.0, 0.0, 1.0];

const WINDOW_SIZE: i32 = 512;
const PIXEL_SIZE: f64 = 32.0;
const WORLD_SIZE: i32 = WINDOW_SIZE / PIXEL_SIZE as i32;

#[derive(Default)]
struct Metrics {
    block_pixels: usize,
    board_x: usize,
    board_y: usize,
}

impl Metrics {
    fn resolution(&self) -> [u32; 2] {
        [
            (self.board_x * self.block_pixels) as u32,
            (self.board_y * self.block_pixels) as u32,
        ]
    }
}

enum State {
    Walking,
    Standing,
}

enum Shape {
    Rectangle { width: f64, height: f64 },
    Circle { radius: f64 },
}

struct Player {
    shape: Shape,
    size: (f64, f64),
    location: (f64, f64), // (x, y)
    color: Colour,
}

impl Player {
    fn render(&self, metrics: &Metrics, window: &mut PistonWindow, event: &Event) {
        // println!("Entered render function");
        window.draw_2d(event, |context, graphics, _| {
            // println!("entering draw2d");
            clear(GREEN, graphics);
            match self.shape {
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
            };
        });
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let metrics = Metrics {
        block_pixels: 20,
        board_x: 8,
        board_y: 20,
    };

    let mut player = Player {
        shape: Shape::Circle { radius: 50.0 },
        size: (50.0, 50.0),
        location: (0.0, 0.0),
        color: RED,
    };

    while let Some(e) = window.next() {
        if let Some(_) = e.render_args() {
            player.render(&metrics, &mut window, &e);
        }

        if let Some(args) = e.press_args() {
            if let Some(movement) = on_press(&args) {
                println!("Key registered as arrow key, movement: {:?}", movement);
                println!(
                    "movement successfully unwrapped, current play location: {:?}",
                    player.location
                );
                let x = player.location.0 + (movement.0 * player.size.0);
                let y = player.location.1 + (movement.1 * player.size.1);
                player.location = (x, y);
                println!(
                    "movement changed location. new location: {:?}",
                    player.location
                );
            }
        }
    }
}

fn on_press(args: &Button) -> Option<(f64, f64)> {
    println!("Entered on_press function");
    match args {
        Button::Keyboard(args) => on_key(args),
        _ => None,
    }
}

fn on_key(key: &Key) -> Option<(f64, f64)> {
    println!("Entered on_key function");
    match key {
        Key::Right => Some((1.0, 0.0)),
        Key::Left => Some((-1.0, 0.0)),
        Key::Up => Some((0.0, -1.0)),
        Key::Down => Some((0.0, 1.0)),
        _ => None,
    }
}
