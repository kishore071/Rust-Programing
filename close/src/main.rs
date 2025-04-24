struct User {
    name: String,
    age: u8,
    salary: u32,
}
fn validate_user(name: &str, bannned_user_name: &str) -> bool {
    name.len() != 0 && name != bannned_user_name
}
fn validate_user_age(age: u8) -> bool {
    age >= 25
}
fn is_valid_user<V1, V2>(
    name: &str,
    age: u8,
    salary: u32,
    validate_user: V1,
    validate_user_age: V2,
    validate_pay: fn(u32) -> bool,
    bannned_user_name: &str,
) -> bool
where
    V1: Fn(&str, &str) -> bool,
    V2: Fn(u8) -> bool,
{
    validate_user(name, bannned_user_name) && validate_user_age(age) && validate_pay(salary)
}
fn main() {
    //println!("Hello, world!");
    let name = User {
        name: "Kaala".to_string(),
        age: 25,
        salary: 20000,
    };
    let validate_pay = |salary: u32| salary >= 15000;
    let bannned_user_name = String::from("Banned User");
    // let validate_user_name = |name: &str| name.len() != 0;
    // let validate_user_age = |age: u8| age >= 25;
    println!(
        "User Validity age is {}",
        is_valid_user(
            &name.name,
            name.age,
            name.salary,
            validate_user,
            validate_user_age,
            validate_pay,
            &bannned_user_name
        )
    );
}
