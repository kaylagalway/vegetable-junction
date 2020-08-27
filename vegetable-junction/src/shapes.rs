// Represents shape to be rendered on the board
// Cases are rectangle and circle, with measurement parameters attached for those shapes
#[derive(Clone)]
pub enum Shape {
    Rectangle { width: f64, height: f64 },
    Circle { radius: f64 },
}

#[derive(Copy, Clone)]
pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
}

impl Circle {
    pub fn collides_with_rectangle(self, rect: Rectangle) -> bool {
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

    pub fn collides_with_circle(self, circle: Circle) -> bool {
        // http://www.jeffreythompson.org/collision-detection/circle-circle.php
        let dist_x = self.x - circle.x;
        let dist_y = self.y - circle.y;
        let distance = ((dist_x * dist_x) + (dist_y * dist_y)).sqrt();
        distance <= (self.radius + circle.radius)
    }
}

#[derive(Copy, Clone)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}
