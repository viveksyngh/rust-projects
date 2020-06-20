fn main() {
    let temp_f = 100.0;
    println!("Temperature in Celsius: {}", fahrenheit_to_celsius(temp_f));
    
    let temp_f = 5.0;
    println!("Temperature in fahrenheit: {}", celsius_to_fahrenheit(temp_f));
}

fn fahrenheit_to_celsius(temp_f: f64) -> f64 {
    let ratio = 5.0/9.0;
    let float_temp: f64 = temp_f - 32.0;
    return  float_temp * ratio
} 

fn celsius_to_fahrenheit(temp_c: f64) -> f64 {
    let ratio = 9.0/5.0;
    return temp_c * ratio + 32.0
}