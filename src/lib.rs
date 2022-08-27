#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use crate::Rectangle;

    #[test]
    fn can_hold() {
        // arrange
        let rect = Rectangle {
            width: 100,
            height: 100,
        };
        let other = Rectangle {
            width: 50,
            height: 50,
        };

        // act
        let result = rect.can_hold(&other);

        // assert
        assert!(result);
    }
}
