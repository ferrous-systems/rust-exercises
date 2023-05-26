use num;

/// A geometric 2D shape that has an area.
pub trait HasArea<T> {
    fn area(&self) -> T;
}

/// Represents a square, a shape with four equal sides.
pub struct Square<T> {
    /// The length of one side of the square
    side: T,
}

impl<T> Square<T> {
    /// Construct a new [`Square`] with the given length for each side.
    pub fn new(side: T) -> Self {
        Square { side }
    }

    /// Calculate the area of the [`Square`]
    fn area(&self) -> T
    where
        T: num::Num + Copy,
    {
        self.side * self.side
    }

    /// Multiplies the length of `side` by a factor to increase/decrease the size of the given [`Square`]
    pub fn scale(&mut self, factor: T)
    where
        T: num::Num + std::ops::MulAssign,
    {
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
    fn area(&self) -> T {
        Square::area(self)
    }
}

/// Represents a circle, with a given radius.
pub struct Circle<T> {
    radius: T,
}

impl<T> Circle<T> {
    /// Construct a new [`Circle`] with the given radius.
    pub fn new(radius: T) -> Self {
        Circle { radius }
    }

    /// Calculate the area of the [`Circle`]
    fn area(&self) -> T
    where
        T: num::Num + Copy + From<f32>,
    {
        self.radius * self.radius * std::f32::consts::PI.into()
    }

    /// Multiplies the radius by a factor to increase/decrease the size of the given [`Circle`]
    pub fn scale(&mut self, factor: T)
    where
        T: num::Num + std::ops::MulAssign,
    {
        self.radius *= factor;
    }

    /// Takes ownership of the given [`Circle`] and returns the underlying value
    pub fn destroy(self) -> T {
        self.radius
    }
}

impl<T> HasArea<T> for Circle<T>
where
    T: num::Num + Copy + From<f32>,
{
    fn area(&self) -> T {
        Circle::area(self)
    }
}

pub enum Shape<T> {
    Square(Square<T>),
    Circle(Circle<T>),
}

impl<T> HasArea<T> for Shape<T>
where
    T: num::Num + Copy + From<f32>,
{
    fn area(&self) -> T {
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
        let test_square = Square::new(5f64);
        assert_eq!(test_square.area(), 25f64);
        let shape = Shape::Square(test_square);
        assert_eq!(shape.area(), 25f64);
    }

    #[test]
    fn circle() {
        let test_circle = Circle::new(4.0f64);
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
        let mut test_circle = Circle::new(4.0f64);
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
