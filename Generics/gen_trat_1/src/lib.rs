pub use shapes::{Circle, Rectangle, ShapeProperty, ShapeType, ShapesTrait};

mod shapes {
    pub enum ShapeType {
        Circle(Circle),
        Rectangle(Rectangle),
    }
    pub trait ShapesTrait: Area {
        fn circumference(&self) -> f32 {
            0.0
        }
    }
    pub trait Area {
        fn area(&self) -> f32 {
            0.0
        }
    }
    pub struct Rectangle {
        width: f32,
        height: f32,
    }
    pub struct Circle {
        radius: f32,
    }

    impl Circle {
        pub fn new(radius: f32) -> Self {
            Self { radius }
        }
    }

    impl Rectangle {
        pub fn new(width: f32, height: f32) -> Self {
            Self { width, height }
        }
    }
    impl ShapesTrait for ShapeType {
        fn circumference(&self) -> f32 {
            match self {
                ShapeType::Circle(c) => c.circumference(),
                ShapeType::Rectangle(r) => r.circumference(),
            }
        }
    }
    impl Area for ShapeType {
        fn area(&self) -> f32 {
            match self {
                ShapeType::Circle(c) => c.area(),
                ShapeType::Rectangle(r) => r.area(),
            }
        }
    }
    impl Area for Circle {
        fn area(&self) -> f32 {
            3.14 * self.radius
        }
    }
    impl Area for Rectangle {
        fn area(&self) -> f32 {
            self.width * self.height
        }
    }
    impl ShapesTrait for Rectangle {
        fn circumference(&self) -> f32 {
            2 as f32 * (self.width + self.height)
        }
    }
    impl ShapesTrait for Circle {
        fn circumference(&self) -> f32 {
            2.00 * 3.14 * self.radius
        }
    }
    pub fn ShapeProperty<T: ShapesTrait>(object: &T)
    // where
    //     T: Circle(c),
    {
        println!("{:.2}", object.circumference());
        println!("{:.2}", object.area());
    }
}
