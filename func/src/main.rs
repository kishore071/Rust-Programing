fn main() {
    //println!("Hello, world!");
    let overall_marks: i64 = 98;
    //Both achieve same result
    // let mut grade: char = 'n';
    // match overall_marks {
    //     90..=100 => grade = 'A',
    //     80..90 => grade = 'B',
    //     70..80 => grade = 'C',
    //     _ => grade = 'F',
    // }
    let grade: char = {
        match overall_marks {
            90..=100 => 'A',
            80..90 => 'B',
            70..80 => 'C',
            _ => 'F',
        }
    };
    println!("The Grade recieved is {}", grade);
}
