struct Car{
    owner:String,
    year:u32,
    fuel_level:f32,
    price:f32
}
fn main() {
    let mut my_car:Car=Car{
        owner: String::from{"ABC"},
        year: 2010,
        fuel_level: 0.0,
        price: 5.0
    };
    //println!("{}",my_car.owner);
}
