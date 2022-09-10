use std::io;

fn fahrenheit_to_celcius(temperature: f32) -> f32 {
    (temperature - 32.0) * (5.0 / 9.0)
}

fn calc_fibonacci_number(length: usize) -> i32 {
    let mut sequence = Vec::new();
    for index in 0..=length {
        if index < 2 {
            sequence.push(1);
        } else if index >= 2 {
            sequence.push(sequence[index - 2] + sequence[index - 1]);
        }
    }
    return sequence[length - 1];
}

fn main() {
    println!("Chapter 3 Exercises");

    let temperature: f32 = loop {
        let mut user_input = String::new();
        println!("Enter a temperature in fahrenheit.");
        io::stdin()
            .read_line(&mut user_input)
            .expect("Could not read line!");
        match user_input.trim().parse() {
            Ok(num) => break num,
            Err(_) => continue,
        };
    };

    println!(
        "{} degrees fahrenheit is equal to {} degrees celcius",
        temperature,
        fahrenheit_to_celcius(temperature)
    );

    println!(
        "The 5th number in the fibonacci sequence is {}",
        calc_fibonacci_number(5)
    );
}
