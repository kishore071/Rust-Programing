#[allow(dead_code)]
mod car {
    pub struct Car {
        owner: String,
        year: u32,
        fuel_level: f32,
        price: f32,
    }

    impl Car {
        pub fn new(owner: String, year: u32, fuel_level: f32, price: f32) -> Car {
            println!("Car Created Successfull");
            Car {
                owner,
                year,
                fuel_level,
                price,
            }
        }

        pub fn car_name(&self) -> String {
            println!("Car Name is retrieved");
            format!("Car Name is {}", &self.owner)
        }
    }
}

use car::Car;

fn main() {
    let my_car: Car = Car::new(String::from("Kallao"), 2010, 0.0, 5.0);
    println!("{}", my_car.car_name());
}
