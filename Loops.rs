fn main() {
    let mut x: i64 = 0;
    loop {
        if x <= 10 {
            println! {"Number is {}",x};
            x += 1;
        } else {
            break;
        }
    }
}
