// Problem 1:
/* You are tasked with implementing a library management system using Rust.
Your goal is to design a program that can handle books and magazines.
To fulfill the requirements, follow the steps below:

Create a structure called Item with the following fields:
id: An integer representing the unique identifier of the item.
title: A string representing the title of the item.
year: An integer representing the publication year of the item.
type: an enumeration type. The details are coming below.

Create an enumeration called ItemType with two variants:
Book: Represents a book.
Magazine: Represents a magazine.

Implement a function called display_item_info() that takes an Item as input
and displays its information. The function should output
the item's ID, title, publication year, and type (book or magazine).
*/
enum ItemType {
    Book,
    Magazine,
}
struct Item {
    id: i64,
    title: String,
    year: u32,
    item_type: ItemType,
}
impl Item {
    fn display_item_info(&self) {
        let item = match self.item_type {
            ItemType::Book => "Book",
            ItemType::Magazine => "Magazine",
        };
        println!(
            "The book u want is '{}' and its id is {} with publication year {} with type is '{}'",
            self.title, self.id, self.year, item,
        );
    }
}
fn main() {
    let book: Vec<Item> = vec![
        Item {
            id: 1001,
            title: String::from("The Rust Book"),
            year: 2021,
            item_type: ItemType::Book,
        },
        Item {
            id: 1002,
            title: String::from("Rust Monthly"),
            year: 2023,
            item_type: ItemType::Magazine,
        },
    ];
    //println!("{}", "Hello");
    for item in &book {
        item.display_item_info();
    }
}
