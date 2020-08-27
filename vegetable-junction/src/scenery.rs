use piston_window::{ellipse, rectangle, Context, G2d};

use crate::colors;
use crate::player::Player;
use crate::shapes::{Circle, Rectangle, Shape};

pub enum SceneryType {
    Tree {
        base_width: f64,
        base_height: f64,
        top_radius: f64,
    },
}

pub struct Scenery {
    pub type_: SceneryType,
    pub location: (f64, f64),
}

impl Scenery {
    pub fn collides(&self, player: &Player<'_>) -> bool {
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

    pub fn render(&self, context: Context, graphics: &mut G2d) {
        match self.type_ {
            SceneryType::Tree {
                base_width,
                base_height,
                top_radius,
            } => {
                rectangle(
                    colors::BLACK,
                    [self.location.0, self.location.1, base_width, base_height],
                    context.transform,
                    graphics,
                );
                let top_x = self.location.0 + (base_width / 2.0);
                let top_y = self.location.1 - top_radius + 5.0;
                ellipse(
                    colors::BLUE,
                    ellipse::circle(top_x, top_y, top_radius),
                    context.transform,
                    graphics,
                );
            }
        }
    }
}
