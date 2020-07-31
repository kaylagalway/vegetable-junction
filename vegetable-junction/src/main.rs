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
        [(self.board_x * self.block_pixels) as u32,
         (self.board_y * self.block_pixels) as u32]
    }
}

enum State {
    Walking,
    Standing,
}

struct Player {
    size: [f64; 2],
    location: [f64; 2], // (x, y)
    color: Colour,
}

impl Player {
    fn render(&self, metrics: &Metrics, window: &mut PistonWindow, event: &Event) {
        window.draw_2d(event, |context, graphics, _| {
            let mut draw = |color, rect: [f64; 4]| {
                Rectangle::new(color).draw(rect, &DrawState::default(), context.transform, graphics);
            };
            clear(GREEN, graphics);
            rectangle(RED, // red
                        [50.0, 50.0, 100.0, 100.0],
                        context.transform,
                        graphics);
        });
    }
}

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true).build().unwrap();

    let metrics = Metrics {
        block_pixels: 20,
        board_x: 8,
        board_y: 20,
    };

    let player = Player {
        size: [50.0; 2],
        location: [0.0; 2],
        color: RED,
    };

    while let Some(e) = window.next() {
        // game.progress();
        player.render(&metrics, &mut window, &e);

        if let Some(_) = e.render_args() {
            player.render(&metrics, &mut window, &e);
        }

        if let Some(args) = e.press_args() {
            on_press(&args);
            // game.on_press(&args);
        }
    }

    //This would handle keyboard arrow interaction I believe
    /*
    if let Some(k) = e.button_args() {
        if k.state == ButtonState::Press {

            match k.button {
                Button::Keyboard(Key::Up) => player.y -= PIXEL_SIZE as i32,
                Button::Keyboard(Key::Down) => player.y += PIXEL_SIZE as i32,
                Button::Keyboard(Key::Left) => player.x -= PIXEL_SIZE as i32,
                Button::Keyboard(Key::Right) => player.x += PIXEL_SIZE as i32,
                _ => (),
            }

        }
    }
    */
}

fn on_press(args: &Button, location: (f64, f64)) -> Option<(f64, f64)> {
    match args {
        Button::Keyboard(args) => on_key(args, location),
        // Button::Keyboard(key) => { self.on_key(key); }
        _ => None,
    }
}

fn on_key(key: &Key, location: (f64, f64)) -> Option<(f64, f64)> {
    match key {
        Key::Right => Some((1.0, 0.0)),
        Key::Left => Some((-1.0, 0.0)),
        Key::Up => Some((0.0, -1.0)),
        Key::Down => Some((0.0, 1.0)),
        _ => None,
    }
}


