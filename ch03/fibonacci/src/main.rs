use std::io;
fn main() {
    let mut n = String::new();
    let n_int: i32;

    loop {
        n.clear();
        println!("Type n:");
        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line");

        
        n_int = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid Entry!");
                continue;
            }
        };
        break;
    };

    let mut a = 1;
    let mut b = 1;
    let mut c: i32;

    for _ in 0..n_int-2 {
        c = a+b;
        a = b;
        b = c;
    };

    println!("The n-th term of Fibonnaci is {}", b);
}
