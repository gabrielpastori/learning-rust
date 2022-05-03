use std::io;

pub enum Option {
    Add,
    Show,
    ShowAll,
    Quit,
}


pub fn display_options() {
    println!(
        "Type the option you want:
    1 - Add a employee
    2 - Retrieve employees by department
    3 - Retrieve all people in the company by department (alphabetically sorted)
    4 - Exit"
    );
}

pub fn read_option() -> Option {
    loop {
        let selected_option = read_str();
        
        let selected_option = match selected_option.as_str() {
            "1" => Option::Add,
            "2" => Option::Show,
            "3" => Option::ShowAll,
            "4" => Option::Quit,
            _ => {
                println!("Type a valid option!");
                display_options();
                continue;
            }
        };

        return selected_option;
        
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

