fn main() {
    //Data type of integer 64 byte
    let mut x: i64 = 10;
    let y: f32 = x as f32 + 55.1;
    println!("Value of x is {}", x);
    x = x + 11;
    println!("Updated Value of x is {}", x);
    println!("Float Value of y is {}", y);
    let far: u8 = y as u8;
    let n: char = far as char;
    println!("{}", far);
    println!("Alphabet is {}", n);
    let num: u32 = n as u32;
    println!("The Covertion of the alphabet to num is {}", num);
}
