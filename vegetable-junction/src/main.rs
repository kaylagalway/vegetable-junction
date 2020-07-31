extern crate piston_window;

use piston_window::*;

type Colour = [f32; 4];

const RED: Colour = [1.0, 0.0, 0.0, 1.0];
const GREEN: Colour = [0.0, 1.0, 0.0, 1.0];

const WINDOW_SIZE: i32 = 512;
const PIXEL_SIZE: f64 = 32.0;
const WORLD_SIZE: i32 = WINDOW_SIZE / PIXEL_SIZE as i32;

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true).build().unwrap();
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [0.0, 0.0, 100.0, 100.0],
                      context.transform,
                      graphics);
        });
    }

    //This would handle keyboard arrow interaction I believe
    if let Some(k) = e.button_args() {
        if k.state == ButtonState::Press {
            /*
            match k.button {
                Button::Keyboard(Key::Up) => player.y -= PIXEL_SIZE as i32,
                Button::Keyboard(Key::Down) => player.y += PIXEL_SIZE as i32,
                Button::Keyboard(Key::Left) => player.x -= PIXEL_SIZE as i32,
                Button::Keyboard(Key::Right) => player.x += PIXEL_SIZE as i32,
                _ => (),
            }
            */
        }
    }
}
