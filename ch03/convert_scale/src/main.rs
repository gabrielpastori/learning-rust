use std::io;

fn main() {
    
    let mut order;

    loop {
        order = String::new();

        println!("type F if you want to convert from Fahrenheit to Celsius, C if the opposite: ");

        io::stdin()
            .read_line(&mut order)
            .expect("Failed to read line");

        order = order.trim().to_string();

        if order == "F" || order == "C" {
            break;
        }

        println!("Invalid Entry!");
    };
    
    loop {
        let mut temperature = String::new();

        println!("type the temperature you want to convert:");

        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line");

        let mut float_temperature: f32 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Type a valid value!");
                continue;
            },
        };

        if order=="F" {
            float_temperature = (float_temperature - 32.0)*5.0/9.0;     
        } else{
            float_temperature = (float_temperature * 9.0/5.0) + 32.0;
        };
        
        println!("Converted temperature: {}", {float_temperature});
        
        break;
    };
    
    
}
