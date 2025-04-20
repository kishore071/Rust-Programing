pub use front_of_house::hostin;
mod front_of_house {
    pub mod hostin {
        fn add_to_waitlist(name: String) -> String {
            return format!("U are in waiting Line ,{}", name);
        }
        pub fn eat_at_restaurant(name: String) -> String {
            let name = crate::front_of_house::hostin::add_to_waitlist(String::from(name));
            name
        }
    }
}
// fn main() {
//     println!("{}", eat_at_restaurant(String::from("kaala")));
// }
