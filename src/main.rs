use std::io;

fn main() {
    let mut temp_f = String::new();

    println!("Please enter your temperature in Fahrenheit: ");

    io::stdin()
        .read_line(&mut temp_f)
        .expect("Please enter a number.");

    let temp_f: f64 = match temp_f.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0,
    };

    let temp_c: f64 = ({temp_f} - 32.0) * 0.556;
    println!("The temperature in centigrade is: {temp_c}");
}
