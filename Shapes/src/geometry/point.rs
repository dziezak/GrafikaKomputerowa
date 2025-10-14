#[derive(Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}


impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Point { x, y }
    }
    pub fn distance(&self, other: &Point) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    pub fn translate(&mut self, dx: f32, dy: f32) {
        self.x += dx;
        self.y += dy;
    }

    pub fn length(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn normalized(&self) -> Point {
        let len = self.length();
        if len == 0.0 {
            Point::new(0.0, 0.0)
        } else {
            Point::new(self.x / len, self.y / len)
        }
    }

    pub fn scale(&self, s: f32) -> Point {
        Point::new(self.x * s, self.y * s)
    }

    pub fn rot90(&self) -> Point {
        Point::new(-self.y, self.x)
    }
}

impl std::ops::Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point { x: self.x - other.x, y: self.y - other.y }
    }
}

impl std::ops::Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

impl std::ops::Mul<f32> for Point {
    type Output = Point;
    fn mul(self, rhs: f32) -> Point {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

