fn main() {
    type Student = (&'static str, i64);
    let student_data: Student = ("Kishore Kumar", 22);
    let students_datas: [Student; 10] = [student_data.clone(); 10];
    println!("Student Name and age is {:?}", student_data);
    for i in 0..10 {
        let (student_name, student_age) = students_datas[i];
        println!(
            "Student Name is {} and his age is {} ",
            student_name, student_age,
        );
        //println!("Students Name and age is {:?}", students_datas[i]);
    }
}
