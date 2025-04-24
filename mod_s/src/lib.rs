pub use shapes::Circle;
mod shapes{
    pub struct Circle{
        pub radius:f32
    }
    impl Circle{
        pub fn new(radius:f32)->Self{
            Self{radius}
        }
        pub fn calculate_circumfernce(radius:f32)->f32{
            2.00*3.14*radius
        }
        pub fn print_circum(&self)->String{
            String::from(format!("{:.2}",crate::shapes::Circle::calculate_circumfernce(self.radius)))
        }
    }
}
