use std::io;

fn main() {
    println!("Press 1 to convert from Fahrenheit to Celsius, otherwise press 2.");

    let mut use_case = String::new();

    io::stdin()
        .read_line(&mut use_case)
        .expect("Please enter either 1 or 2.");

    if use_case.trim() == "1" {
        convert_to_c();
    } else {
        convert_to_f();
    }
}

fn convert_to_c() {
    let mut temp_f = String::new();

    println!("Please enter your temperature in fahrenheit: ");

    io::stdin()
        .read_line(&mut temp_f)
        .expect("Please enter a number.");

    let temp_f: f32 = match temp_f.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0,
    };

    let temp_c: f32 = ({temp_f} - 32.0) * 0.556;
    println!("The temperature in centigrade is: {temp_c}");
}

fn convert_to_f() {
    let mut temp_c = String::new();

    println!("Please enter your temperature in celsius: ");

    io::stdin()
        .read_line(&mut temp_c)
        .expect("Please enter a number.");

    let temp_c: f32 = match temp_c.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0,
    };

    let temp_f: f32 = ({temp_c} * 1.8) + 32.0;
    println!("The temperature in fahrenheit is: {temp_f}");
}