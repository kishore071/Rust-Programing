use std::io;
fn main() {
    let mut value: String = String::new();
    io::stdin()
        .read_line(&mut value)
        .expect("Error in Reading Value");
    let mut val: Vec<String> = value
        .trim()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    for name in val {
        print!("{} ", reverse_str(name));
    }
}
fn reverse_str(input: String) -> String {
    let mut input_vec: Vec<char> = input.chars().collect();
    let mut j: usize = input_vec.len() - 1;
    for i in 0..(input_vec.len() / 2) {
        let temp = input_vec[i];
        input_vec[i] = input_vec[j];
        input_vec[j] = temp;
        j -= 1;
    }
    input_vec.into_iter().collect()
}
/*use std::io;

fn main() {
    let mut value: String = String::new();
    io::stdin()
        .read_line(&mut value)
        .expect("Error in Reading Value");

    // Split, reverse each word, and collect the reversed words back into a new string
    let reversed_value: String = value
        .trim() // Remove leading/trailing whitespace
        .split_whitespace() // Split based on whitespace
        .map(|word| reverse_str(word)) // Reverse each word
        .collect::<Vec<String>>()
        .join(" "); // Join the words back with spaces

    // Print the modified value
    println!("{}", reversed_value);
}

fn reverse_str(input: &str) -> String {
    let mut input_vec: Vec<char> = input.chars().collect();
    let mut j: usize = input_vec.len() - 1;
    for i in 0..(input_vec.len() / 2) {
        let temp = input_vec[i];
        input_vec[i] = input_vec[j];
        input_vec[j] = temp;
        j -= 1;
    }
    input_vec.into_iter().collect()
}
*/
