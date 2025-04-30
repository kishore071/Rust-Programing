trait Multiplication<Rhs, Output> {
    //type Output = i32;
    fn multiply(self, rhs: Rhs) -> Output;
}
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
impl Multiplication<Point, Point> for Point {
    fn multiply(self, rhs: Point) -> Point {
        Point {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}
impl Multiplication<i32, Point> for Point {
    fn multiply(self, rhs: i32) -> Point {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

fn main() {
    //println!("Hello, world!");
    let input_1: Point = Point { x: 10, y: 20 };
    let input_2: Point = Point { x: 3, y: 5 };
    println!("{:?}", input_1.multiply(input_2));
    println!("{:?}", Point { x: 10, y: 20 }.multiply(2));
}
