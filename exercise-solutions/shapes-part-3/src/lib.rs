use num;
pub struct Square<T> {
    /// The length of one side of the square
    side: T,
}

impl<T> Square<T>
where
    T: num::Num + Copy + std::ops::MulAssign,
{
    /// Construct a new [`Square`] with the given length for each side.
    pub fn new(side: T) -> Square<T> {
        Square { side }
    }

    /// Multiplies the length of `side` by a factor to increase the size of the given [`Square`]
    pub fn scale(&mut self, factor: T) {
        self.side *= factor;
    }

    /// Takes ownership of the given [`Square`] and returns the underlying value
    pub fn destroy(self) -> T {
        self.side
    }
}

impl<T> HasArea<T> for Square<T>
where
    T: num::Num + Copy,
{
    /// Calculate the area of the given [`Square`]
    fn area(&self) -> T {
        self.side * self.side
    }
}

pub trait HasArea<T> {
    fn area(&self) -> T;
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

    /// Multiplies the radius by a factor to increase the size of the given [`Circle`]
    pub fn scale(&mut self, factor: f32) {
        self.radius *= factor;
    }

    /// Takes ownership of the given [`Circle`] and returns the underlying value
    pub fn destroy(self) -> f32 {
        self.radius
    }
}
impl<T> HasArea<T> for Circle
where
    T: num::Num + Copy,
    f32: core::ops::Mul<f32, Output = T>,
{
    /// Calculate the area of the given [`Square`]
    fn area(&self) -> T {
        // std::f32::consts::PI *
        self.radius * self.radius
    }
}

pub enum Shape<T> {
    Square(Square<T>),
    Circle(Circle),
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
    fn cicle() {
        let test_circle = Circle::new(4.0);
        assert_eq!(test_circle.area(), 50.265484);
    }

    #[test]
    fn cicle_scale() {
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
    fn cicle_destroy() {
        let test_circle = Circle::new(4.0);
        assert_eq!(test_circle.destroy(), 4.0);
    }
    #[test]
    fn square_destroy() {
        let test_square = Square::new(4u32);
        assert_eq!(test_square.destroy(), 4u32);
    }
}
