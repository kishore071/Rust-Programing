#[derive(Debug)]
struct Km {
    value: u32,
}
#[derive(Debug)]
struct Kmh {
    value: u32,
}
#[derive(Debug)]
struct Miles {
    value: u32,
}
#[derive(Debug)]
struct Mph {
    value: u32,
}
// impl Kmh {
//     fn distance_in_three_hours(&self) -> Km {
//         Km {
//             value: self.value * 3,
//         }
//     }
// }
// impl Mph {
//     fn distance_in_three_hours(&self) -> Miles {
//         Miles {
//             value: self.value * 3,
//         }
//     }
// }
// Associated Traits
trait DistanceThreeHours {
    type Distance;
    fn distance_in_three_hours(&self) -> Self::Distance;
}
impl DistanceThreeHours for Kmh {
    type Distance = Km;
    fn distance_in_three_hours(&self) -> Self::Distance {
        Self::Distance {
            value: self.value * 3,
        }
    }
}
impl DistanceThreeHours for Mph {
    type Distance = Miles;
    fn distance_in_three_hours(&self) -> Self::Distance {
        Self::Distance {
            value: self.value * 3,
        }
    }
}
fn main() {
    //println!("Hello, world!");
    let speed_kmh: Kmh = Kmh { value: 90 };
    let distance_km: Km = speed_kmh.distance_in_three_hours();
    println!(
        "At {:?},you will travel {:?} in 3 hours",
        speed_kmh, distance_km
    );
    let speed_Mph = Mph { value: 90 };
    let distance_Miles = speed_Mph.distance_in_three_hours();
    println!(
        "At {:?},you will travel {:?} in 3 hours",
        speed_Mph, distance_Miles
    );
}
