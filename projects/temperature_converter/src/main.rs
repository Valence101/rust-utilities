use std::io;

fn main() {
    println!("Welcome to the Temperature Converter!");

    loop {
        println!("Would you like to convert to Celsius or Fahrenheit? (c/f):");

        let mut c_or_f = String::new();
    
        io::stdin()
            .read_line(&mut c_or_f)
            .expect("Failed to read line");
    
        if c_or_f.trim() == "c" {
            let f: f64 = prompt_user_for_temperature("You have selected to convert to Celsius, please enter your temperature in Fahrenheit:".to_string());
            let c: f64 = convert_f_to_c(f);
            println!("{f} degrees fahrenheit = {c} degrees celcius");
        } else {
            let c: f64 = prompt_user_for_temperature("You have selected to convert to Fahrenheit, please enter your temperature in Celsius:".to_string());
            let f: f64 = convert_c_to_f(c);
            println!("{c} degrees celcius = {f} degrees fahrenheit");
        }
    }
}

fn convert_f_to_c(fahrenheit: f64) -> f64 {
    let c: f64 = (fahrenheit - 32.0) * (5.0/9.0);
    c
}

fn convert_c_to_f(celcius: f64) -> f64 {
    let f: f64 = (celcius * (9.0/5.0)) + 32.0;
    f
}

fn prompt_user_for_temperature(prompt: String) -> f64 {
    println!("{prompt}");
    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");
    let t: f64 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => prompt_user_for_temperature(prompt),
    };
    t
}
