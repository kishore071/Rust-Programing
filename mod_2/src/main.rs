use mod_2::hostin::eat_at_restaurant;
use std::io;
fn main() {
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to recieve input");
    //.trim();
    name = format!("{}", name.trim());
    println!(
        "{}",
        eat_at_restaurant(String::from(format!("{}", &name[0..3])))
    );
}
