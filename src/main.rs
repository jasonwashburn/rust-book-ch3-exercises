use std::io;


fn fahrenheit_to_celcius(temperature: f32) -> f32 {
    (temperature - 32.0) * (5.0 / 9.0)
}

fn main() {
    println!("Chapter 3 Exercises");

    let temperature: f32 = loop {
        let mut user_input = String::new();
        println!("Enter a temperature in fahrenheit.");
        io::stdin().read_line(&mut user_input).expect("Could not read line!");
        match user_input.trim().parse() {
            Ok(num) => break num,
            Err(_) => continue,
        };
    };

    println!("{} degrees fahrenheit is equal to {} degrees celcius", temperature, fahrenheit_to_celcius(temperature));
}
