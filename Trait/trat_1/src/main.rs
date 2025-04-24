use shapes::{Area, Circle, Circumference, Rectangle};
pub mod shapes {
    pub struct Circle {
        radius: f32,
    }
    pub struct Rectangle {
        height: f32,
        length: f32,
    }
    pub trait Circumference: Area {
        fn circumference(&self) -> f32 {
            0.0
        }
    }
    pub trait Area {
        fn area(&self) -> f32 {
            0.0
        }
    }
    impl Circumference for Circle {
        fn circumference(&self) -> f32 {
            2 as f32 * 3.14 * self.radius
        }
    }
    impl Circle {
        pub fn new(radius: f32) -> Self {
            Self { radius }
        }
    }
    impl Rectangle {
        pub fn new(height: f32, length: f32) -> Self {
            Self { height, length }
        }
    }
    impl Circumference for Rectangle {
        fn circumference(&self) -> f32 {
            2 as f32 * (self.height + self.length)
        }
    }
    impl Area for Circle {
        fn area(&self) -> f32 {
            3.14 * self.radius * self.radius
        }
    }
    impl Area for Rectangle {}
}
// fn circum_print<T: Circumference + Area>(object: &T) {
//     println!(
//         "Circumfernce and area of Circle is {:.2} and {:.2}",
//         object.circumference(),
//         object.area()
//     );
// }
// fn return_shape(val: &Vec<f32>) -> impl Circumference + Area {
//     //if val.len() == 1 {
//     Circle::new(val[0])
//     // } else {
//     //     Rectangle::new(val[0], val[1])
//     // }
// }
trait Shape: Circumference + Area {}
impl<T: Circumference + Area> Shape for T {}

fn circum_print(shape: &dyn Shape) {
    println!(
        "Circumference: {:.2}, Area: {:.2}",
        shape.circumference(),
        shape.area()
    );
}

fn return_shape(val: &Vec<f32>) -> Box<dyn Shape> {
    if val.len() == 1 {
        Box::new(Circle::new(val[0]))
    } else {
        Box::new(Rectangle::new(val[0], val[1]))
    }
}
fn main() {
    //println!("{:?}", [1; 10]);
    let test_1: Vec<f32> = [20.0].to_vec();
    let _circle = return_shape(&test_1); //Circle::new(20.0);
    let _rectangle: Rectangle = Rectangle::new(10.0, 20.0);
    circum_print(&*_circle);
}
