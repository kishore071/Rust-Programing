use mod_s::Circle;
fn main(){
     let circle_radius=Circle::new(20.0);
     println!("Circumference of the circle of radius 20 is {}",circle_radius.print_circum());
     //println!("Calulate  Circumference is {:.2}",Circle::calculate_circumfernce(circle_radius.radius));
}