/// Represents a square, a shape with four equal sides.
pub struct Square {
    /// The length of one side of the square
    side: u32
}

impl Square {
    /// Construct a new [`Square`] with the given length for each side.
    pub fn new(side: u32) -> Self {
        Square {
            side
        }
    }

    /// Calculate the area of the given [`Square`]
    pub fn area(&self) -> u32 {
        self.side * self.side
    }

    /// Calculate the perimeter of the given [`Square`]
    pub fn perimeter(&self) -> u32 {
        self.side * 4u32
    }
}

/// Represents a circle, with a given radius.
pub struct Circle {
    radius: f32
}

impl Circle {
    /// Construct a new [`Circle`] with the given radius.
    pub fn new(radius: f32) -> Self {
        Circle {
            radius
        }
    }

    /// Calculate the area of the given [`Circle`]
    pub fn area(&self) -> f32 {
        std::f32::consts::PI * self.radius * self.radius
    }

    /// Calculate the perimeter of the given [`Circle`]
    pub fn perimeter(&self) -> f32 {
        std::f32::consts::PI * self.radius * 2.0
    }

    pub fn scale(&mut self, factor: f32) {
        self.radius *= factor;
    }
}

/// Represents a Right Angled Triangle.
/// 
/// Specified with the length of the two sides adjacent to the right-angle.


/// Represents a square, a shape with four equal sides.
pub struct Square<T> {
    /// The length of one side of the square
    side: T
}

impl<T> Square<T> {
    /// Construct a new [`Square`] with the given length for each side.
    pub  fn new(side: T) -> Square<T> {
        Square {
            side
        }
    }

    /// Calculate the area of the given [`Square`]
    pub  fn area(&self) -> T where T: num::Num + Copy {
        self.side * self.side
    }

    /// Calculate the perimeter of the given [`Square`]
    pub  fn perimeter(self: &Square<T>) -> T where T: num::Num + From<u8> + Copy { 
        self.side * 4u8.into()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn squares() {
        let test_square = Square::new(5u32);
        assert_eq!(test_square.area(), 25u32);
        assert_eq!(test_square.perimeter(), 20u32);
    }
}

pub enum Shape {
    Square(Square),
    Circle(Circle),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn squares() {
        let test_square = Square::new(5);
        assert_eq!(test_square.area(), 25);
        assert_eq!(test_square.perimeter(), 20);
    }

    #[test]
    fn cicle() {
        let test_circle = Circle::new(4.0);
        assert_eq!(test_circle.area(), 50.265484);
        assert_eq!(test_circle.perimeter(), 25.132742);
    }

    #[test]
    fn cicle_scale() {
        let mut test_circle = Circle::new(4.0);
        test_circle.scale(2.0);
        println!("{}", test_circle.radius);
        assert_eq!(test_circle.area(), 201.06194);
        assert_eq!(test_circle.perimeter(), 50.265484);
    }
}

