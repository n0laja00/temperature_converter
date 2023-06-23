use std::io;

fn main() {
    println!("Welcome to a temperature converter! We convert Celsius into Fahrenheit using Rust!");

    loop {
        println!("Please, insert the temperature in decimals!"); 
        let mut temperature_celsius = String::new(); 

        io::stdin()
            .read_line(&mut temperature_celsius)
            .expect("Failed to readline.");
        
        // Convert string to float
        let temperature_celsius: f32 = match temperature_celsius.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your temperature in Celsius is {temperature_celsius:.2}!");

        let temperature_fahrenheit = celsius_to_fahrenheit(temperature_celsius);

        println!("Your temperature in Fahrenheit is {temperature_fahrenheit:.2}!")
    }
}

// Function to convert celsius to Fahrenheit
fn celsius_to_fahrenheit(temp_c: f32) -> f32 {
    temp_c * (9.0/5.0) + 32.0
}
