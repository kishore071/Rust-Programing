#[warn(unused_imports)]
use gen_trat_1::{Circle, Rectangle, ShapeProperty, ShapeType, ShapesTrait};
fn main() {
    let circle = ShapeType::Circle(Circle::new(10.0));
    let rectangle = ShapeType::Rectangle(Rectangle::new(5.0, 7.0));
    println!(
        "Circumfernce for Circle and Rectangle is {:.2} and {:.2}",
        circle.circumference(),
        rectangle.circumference()
    );
    ShapeProperty(&circle);
    //println!("{}", ShapeProperty(circle));
}
