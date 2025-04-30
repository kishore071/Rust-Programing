//fibonnaci series
use std::io;
fn main() {
    //println!("Hello, world!");
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Invalid Number");
    let num = num.trim().parse::<u32>().unwrap();
    print!("{}", fibonnaci(num));
}
fn fibonnaci(val: u32) -> u32 {
    if val == 1 || val == 0 {
        if val == 1 { 1 } else { 0 }
    } else {
        let mut num_1 = 0;
        let mut num_2 = 1;
        let mut res = 0;
        for i in 1..val {
            //num_1+=num_2;
            res = num_1 + num_2;
            num_1 = num_2;
            num_2 = res;
        }
        res
    }
}
