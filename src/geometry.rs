#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Rectangle {
            x,
            y,
            width,
            height,
        }
    }

    pub fn contains(&self, x: f64, y: f64) -> bool {
        x >= self.x &&
        x <= self.x + self.width &&
        y >= self.y &&
        y <= self.y + self.height
    }

    pub fn new_nw(&self) -> Rectangle {
        // x.
        // ..
        Rectangle::new(
            self.x,
            self.y,
            self.width / 2.0,
            self.height / 2.0
        )
    }

    pub fn new_ne(&self) -> Rectangle {
        // .x
        // ..
        Rectangle::new(
            self.x + self.width / 2.0,
            self.y,
            self.width / 2.0,
            self.height / 2.0,
        )
    }

    pub fn new_sw(&self) -> Rectangle {
        // ..
        // x.
        Rectangle::new(
            self.x,
            self.y + self.height / 2.0,
            self.width / 2.0,
            self.height / 2.0,
        )
    }

    pub fn new_se(&self) -> Rectangle {
        // ..
        // .x
        Rectangle::new(
            self.x + self.width / 2.0,
            self.y + self.height / 2.0,
            self.width / 2.0,
            self.height / 2.0,
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point2D<T: std::fmt::Debug> {
    pub x: f64,
    pub y: f64,
    pub data: T,
}
