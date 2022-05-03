use std::collections::HashMap;

pub mod human_resources {

    #[derive(Default)]
    pub struct Employee {
        data: super::HashMap<String, Vec<String>>,
    }

    impl Employee {
        pub fn add_employee_department(&mut self, name: String, department: String) {
            self.data
                .entry(department)
                .or_insert_with(Vec::new)
                .push(name);
        }

        pub fn show_employees_by_department(&self, department: &str) {
            match self.data.get(department) {
                Some(employees) => {
                    println!("Department: {}", department);
                    println!("Employees:");
                    for employee in employees {
                        println!("- {}", employee);
                    }
                }
                None => println!("This department does not exists"),
            };
        }

        pub fn show_all_employees(&self) {
            let mut all_employees = Vec::<String>::new();
            for (department, employees) in &self.data {
                for employee in employees {
                    all_employees.push(format!("{} - {}", employee, department));
                }
            }
            all_employees.sort();
            println!("Employee - Departament:");
            for employee in &all_employees {
                println!("{}", employee);
            }
        }

        pub fn new() -> Self {
            Default::default()
        }
    }
}
