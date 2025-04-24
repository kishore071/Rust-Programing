mod shapes {
    pub struct Circle {
        radius: f32,
    }
    impl Circle {
        pub fn new(radius: f32) -> Self {
            Self { radius }
        }
        pub fn new_1(radius: f32) -> Result<Circle, String> {
            if radius > 0.0 {
                Ok(Circle { radius })
            } else {
                Err(String::from("radius should be positive"))
            }
        }
        pub fn contains(&self, other: &Circle) -> bool {
            self.radius > other.radius
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn larger_circle_contains() {
        let larger_circle = shapes::Circle::new(10.0);
        let smaller_circle = shapes::Circle::new(8.0);
        assert_eq!(
            larger_circle.contains(&smaller_circle),
            true,
            "Custom failure Message"
        );
        assert_ne!(larger_circle.contains(&smaller_circle), false);
        assert!(larger_circle.contains(&smaller_circle));
    }

    #[test]
    fn smaller_circle_contains() {
        let larger_circle = shapes::Circle::new(10.0);
        let smaller_circle = shapes::Circle::new(8.0);
        assert_eq!(!smaller_circle.contains(&larger_circle), true);
    }
    #[test]
    fn should_not_create_circle() -> Result<(), String> {
        let some_circle = shapes::Circle::new_1(-1.0)?;
        Ok(())
    }
    #[test]
    #[should_panic]
    fn should_not_create_circle_panic() {
        let some_circle = shapes::Circle::new_1(-10.0);
        //Ok(())
    }
}
