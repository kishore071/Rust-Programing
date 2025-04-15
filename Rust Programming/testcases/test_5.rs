// Problem 4:

/*
A palindrome is a word, verse, or sentence that reads the same backward or forward,
such as 'Able was I ere I saw Elba,' or a number like 1881.

Write a function named is_palindrome() that checks whether a given string is a palindrome or not.
The function should take a string as input and return a boolean value indicating whether the string is a palindrome or not.
*/

fn main() {
    let input = String::from("1881");
    println!(
        "It is {:?} that the given string is palindrome",
        palindrome(input)
    );
}

fn palindrome(input: String) -> bool {
    /* Your Code here */
    let palin: Vec<char> = input.chars().collect();
    //let mut first=0;
    let last = palin.len();
    //println!("{last}");
    for i in 0..last / 2 {
        if palin[i] != palin[last - i - 1] {
            return false;
        }
    }
    true
}
