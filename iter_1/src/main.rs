// use std::io;
// fn main() {
//     let mut name = String::new();
//     io::stdin().read_line(&mut name).expect("Error");
//     name = name.trim().to_string();
//     println!("Hello, {}!", name);
// }
#[derive(Debug)]
struct Employee {
    name: String,
    salary: u16,
}
struct Employee_Records {
    employee_db: Vec<Employee>,
}
impl Iterator for Employee_Records {
    type Item = Employee;
    fn next(&mut self) -> Option<Self::Item> {
        if self.employee_db.len() != 0 {
            let result = self.employee_db.remove(0);
            //self.employee_db.remove(0);
            Some(result)
        } else {
            None
        }
    }
}
impl Employee_Records {
    fn print_emp(&mut self) -> Result<Employee, String> {
        match self.next() {
            Some(val) => Ok(val),
            None => Err(String::from("No Data Found")),
        }
    }
}
fn main() {
    let mut emp_1: Employee = Employee {
        name: "Kaala".to_string(),
        salary: 20000,
    };
    let mut emp_2: Employee = Employee {
        name: "Kishore".to_string(),
        salary: 29000,
    };
    let mut emp_db = Employee_Records {
        employee_db: vec![emp_1, emp_2],
    };
    // println!("{}", emp_db.print_emp());
    // println!("{}", emp_db.print_emp());
    // println!("{}", emp_db.print_emp());
    // println!("{}", emp_db.next().unwrap());
    // println!("{}", emp_db.next().unwrap());
    // println!("{}", |emp_db.next()!=Some(None));
    for employee_details in emp_db {
        println!("{:?}", employee_details);
    }
}
