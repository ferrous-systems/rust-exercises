/// A geometric 2D shape that has an area.
pub trait HasArea {
    fn area(&self) -> f32;
}

/// Represents a square, a shape with four equal sides.
pub struct Square {
    /// The length of one side of the square
    side: u32,
}

impl Square {
    /// Construct a new [`Square`] with the given length for each side.
    pub fn new(side: u32) -> Self {
        Square { side }
    }

    /// Calculate the area of the [`Square`]
    fn area(&self) -> u32 {
        self.side * self.side
    }

    /// Multiplies the length of `side` by a factor to increase the size of the given [`Square`]
    pub fn scale(&mut self, factor: u32) {
        self.side *= factor;
    }

    /// Takes ownership of the given [`Square`] and returns the underlying value
    pub fn destroy(self) -> u32 {
        self.side
    }
}

impl HasArea for Square {
    fn area(&self) -> f32 {
        Square::area(self) as f32
    }
}

/// Represents a circle, with a given radius.
pub struct Circle {
    radius: f32,
}

impl Circle {
    /// Construct a new [`Circle`] with the given radius.
    pub fn new(radius: f32) -> Self {
        Circle { radius }
    }

    /// Calculate the area of the given [`Circle`]
    pub fn area(&self) -> f32 {
        self.radius * self.radius * std::f32::consts::PI
    }

    /// Multiplies the radius by a factor to increase/decrease the size of the given [`Circle`]
    pub fn scale(&mut self, factor: f32) {
        self.radius *= factor;
    }

    /// Takes ownership of the given [`Circle`] and returns the underlying value
    pub fn destroy(self) -> f32 {
        self.radius
    }
}

impl HasArea for Circle {
    fn area(&self) -> f32 {
        Circle::area(self)
    }
}

pub enum Shape {
    Square(Square),
    Circle(Circle),
}

impl HasArea for Shape {
    fn area(&self) -> f32 {
        match self {
            Shape::Square(sq) => HasArea::area(sq),
            Shape::Circle(ci) => HasArea::area(ci),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn squares() {
        let test_square = Square::new(5);
        assert_eq!(test_square.area(), 25);
        let shape = Shape::Square(test_square);
        assert_eq!(shape.area(), 25.0);
    }

    #[test]
    fn circle() {
        let test_circle = Circle::new(4.0);
        assert!((test_circle.area() - 50.265484).abs() < 0.001);
        let shape = Shape::Circle(test_circle);
        assert!((shape.area() - 50.265484).abs() < 0.001);
    }

    #[test]
    fn square_scale() {
        let mut test_square = Square::new(4);
        test_square.scale(2);
        assert_eq!(test_square.area(), 64);
    }

    #[test]
    fn circle_scale() {
        let mut test_circle = Circle::new(4.0);
        test_circle.scale(2.0);
        assert!((test_circle.area() - 201.06194).abs() < 0.001);
    }

    #[test]
    fn square_destroy() {
        let test_square = Square::new(4u32);
        assert_eq!(test_square.destroy(), 4u32);
    }

    #[test]
    fn circle_destroy() {
        let test_circle = Circle::new(4.0);
        assert_eq!(test_circle.destroy(), 4.0);
    }
}
