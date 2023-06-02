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

    /// Calculate the area of the given [`Square`]
    pub fn area(&self) -> u32 {
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
        std::f32::consts::PI * self.radius * self.radius
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn squares() {
        let test_square = Square::new(5u32);
        assert_eq!(test_square.area(), 25u32);
    }

    #[test]
    fn circle() {
        let test_circle = Circle::new(4.0);
        assert_eq!(test_circle.area(), 50.265484);
    }

    #[test]
    fn circle_scale() {
        let mut test_circle = Circle::new(4.0);
        test_circle.scale(2.0);
        println!("{}", test_circle.radius);
        assert_eq!(test_circle.area(), 201.06194);
    }
    #[test]
    fn square_scale() {
        let mut test_square = Square::new(4);
        test_square.scale(2);
        println!("{}", test_square.side);
        assert_eq!(test_square.area(), 64);
    }
    #[test]
    fn circle_destroy() {
        let test_circle = Circle::new(4.0);
        assert_eq!(test_circle.destroy(), 4.0);
    }
    #[test]
    fn square_destroy() {
        let test_square = Square::new(4u32);
        assert_eq!(test_square.destroy(), 4u32);
    }
}
