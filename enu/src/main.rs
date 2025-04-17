// enum Weekday {
//     Monday,
//     Tuesday,
//     Wednesday,
//     Thrusday,
//     Friday,
//     Saturday,
//     Sunday,
// }
// fn main() {
//     //println!("Hello, world!");
//     let mut_day: String = String::from("hi");
//     println!("{}", mut_day);
//     let weekday: Vec<String> = vec![
//         "Monday".to_string(),
//         "Tuesday".to_string(),
//         "Wednesday".to_string(),
//         "Thrusday".to_string(),
//         "Friday".to_string(),
//         "Saturday".to_string(),
//         "Sunday".to_string(),
//     ];
//     let day = weekday[1].clone();
//     let day = Weekday::Saturday;
// }
enum TravelType {
    Car(f32),
    Train(f32),
    Bus(f32),
    Aeroplane(f32),
}
impl TravelType {
    fn travel_allowance(&self) -> f32 {
        let allowance = match self {
            TravelType::Car(miles) => miles * 2.0,
            TravelType::Train(miles) => miles * 3.0,
            TravelType::Bus(miles) => miles * 4.5,
            TravelType::Aeroplane(miles) => miles * 5.0,
        };
        allowance
    }
}
fn main() {
    let participant = TravelType::Train(60.0);
    println!("{:.3}", participant.travel_allowance() as i32);
}
