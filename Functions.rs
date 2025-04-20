fn main() {
    let name: String = {
        if 2 / 3 == 1 {
            let num = "Kishore";
            format!("{}", num)
        } else {
            format!("Kumar")
        }
    };
    println!("Name is {}", name);
    multiply(2, 3);
    println!("Divided value of {}/{} is {}", 3, 2, divide(3, 2));
}
fn multiply(x: i64, y: i64) {
    println!("Multiplied value of {} x {} is {}", x, y, x * y);
}
fn divide(x: i64, y: i64) -> i64 {
    //return x/y ; //also works aswell
    x / y //without semicolon meant return statement
}
