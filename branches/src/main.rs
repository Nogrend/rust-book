fn main() {
    temperature_convertor(100.0, "F", "C");
    temperature_convertor(100.0, "C", "F");

    let fib = fibonacci_to(10);
    println!("{:?}", fib);
}

fn temperature_convertor(temperature: f32, from: &str, to: &str) {
    let converted_temp = match from {
        "F" => convert_fahrenheit_to_celsius(temperature),
        "C" => convert_celsius_to_fahrenheit(temperature),
        _ => panic!("Invalid temperature unit"),
    };
    println!(
        "{} degrees {} is {} degrees {}",
        temperature, from, converted_temp, to
    );
}

fn convert_fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn convert_celsius_to_fahrenheit(celsius: f32) -> f32 {
    (celsius * 9.0 / 5.0) + 32.0
}

fn fibonacci_to(n: u32) -> Vec<u32> {
    let mut fib = vec![0, 1];
    for _ in 2..n {
        fib.push(fib[fib.len() - 2] + fib[fib.len() - 1]);
    }
    fib
}