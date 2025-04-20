//HashMap Use
use std::collections::HashMap;
fn main() {
    println!("Hello, world!");
    let mut person: HashMap<&str, i32> = HashMap::new();
    person.insert("kishore", 22);
    person.insert("kaala", 10);
    person.insert("OP", 24);
    println!("The age is {:?}", person.get("kishore").unwrap());
    if person.contains_key("OP") {
        println!("Value exists");
    } else {
        println!("not exits");
    }
    for (name, age) in &person {
        println!("The person {} has a age of {}", name, age);
    }
    let mut personal: HashMap<&str, &str> = HashMap::new();
    //personal.insert("Kaala", "Kishore");
    //personal.insert("Kaala", "OP");
    personal.entry("Kaala").or_insert("Kishore");
    personal.entry("Kaala").or_insert("OP");
    //personal.insert("Kishore", "Kumar");
    //personal.insert("Rajalakshmi", "Baskar");
    println!("personal value is {:?}", personal);
    for (first, last) in &personal {
        println!("Full Name is {} {}", first, last);
    }
}
