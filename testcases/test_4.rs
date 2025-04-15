fn main() {
    let mut name: String = String::from("");
    name.push_str("Hello Mr,");
    name.push('K');
    println!("{name}");
    let mut num = 0;
    for i in name.chars() {
        println!("{} and len is {}", i, num + 1);
        num += 1;
    }
}
