fn main() {
    let mut x: i32 = 5;
    let mut i: i32 = 0;
    let mut y: i32 = 1;
    while x != 1 {
        //i = y;
        //i = i + y;
        let m: i32 = y + i;
        i = y;
        y = m;
        x = x - 1;
    }
    println!("Fibonnaci value for {} is {}", x, i);
}
