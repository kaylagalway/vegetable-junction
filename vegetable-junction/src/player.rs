use crate::colors::Colour;
use crate::metrics::Metrics;
use crate::shapes::Shape;
use piston_window::{ellipse, rectangle, Context, G2d};

// Player struct
// Parameters:
// shape: shape of character, can currently be circle or rectangle
// size: size of character (width, height)
// location: coordinates of character (x, y)
// color: color of character
#[derive(Clone)]
pub struct Player<'a> {
    pub shape: Shape,
    pub size: (f64, f64),
    pub location: (f64, f64),
    pub color: Colour,
    pub metrics: &'a Metrics,
}

impl<'a> Player<'a> {
    pub fn render(&self, context: Context, graphics: &mut G2d) {
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
