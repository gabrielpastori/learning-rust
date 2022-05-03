mod company;
mod option;
use crate::option::Option;
use crate::company::human_resources::Employee;
use std::io;

fn main() {
    let mut employee_department = Employee::new();

    loop {
        option::display_options();
        let option = option::read_option();

        match &option {
            Option::Add => {
                println!("Type the name of the employee");
                let name = read_str();
                println!("Type the name of the department");
                let department = read_str();
                employee_department.add_employee_department(name, department);
            }
            Option::Show => {
                println!("Type the name of the department");
                let department = read_str();
                employee_department.show_employees_by_department(&department);
            }
            Option::ShowAll => {
                employee_department.show_all_employees();
            }
            Option::Quit => {
                break;
            }
        }
    }
}

fn read_str() -> String {
    let mut string_buffer = String::new();
    io::stdin()
        .read_line(&mut string_buffer)
        .expect("Failed to read line!");

    string_buffer = string_buffer.trim().to_string();

    string_buffer
}
