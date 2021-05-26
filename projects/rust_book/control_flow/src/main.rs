fn main() {
    let n = 20;
    println!("\nThe fibonacci number at position {} is {}\n\n", n, fibonacci(n));

    let temp = 130.0;
    println!("{} degrees C is {} degrees F", temp, temperature_convert_c_to_f(temp));
    println!("{} degrees C is {} degrees K", temp, temperature_convert_c_to_k(temp));
    println!("{} degrees F is {} degrees C", temp, temperature_convert_f_to_c(temp));
    println!("{} degrees F is {} degrees K", temp, temperature_convert_f_to_k(temp));
    println!("{} degrees K is {} degrees C", temp, temperature_convert_k_to_c(temp));
    println!("{} degrees K is {} degrees F", temp, temperature_convert_k_to_f(temp));
}

fn fibonacci(x : i64) -> i64{
    let mut final_num = 0;
    let mut two_ago = 0;
    let mut one_ago = 0;
    for count in 0..x + 1{
        let current_number = if count < 2 { count } else{ one_ago + two_ago };
        final_num = current_number;
        two_ago = one_ago;
        one_ago = current_number;
        print!("{} ", current_number);
    }
    final_num
}

fn temperature_convert_c_to_f(celsius : f64) -> f64{
    let fahrenheit = (celsius * (9.0/5.0)) + 32.0;
    fahrenheit
}

fn temperature_convert_c_to_k(celsius : f64) -> f64{
    let kelvin = celsius + 273.15;
    kelvin
}

fn temperature_convert_f_to_c(fahrenheit : f64) -> f64{
    let celsius = (fahrenheit - 32.0) / (9.0/5.0);
    celsius
}

fn temperature_convert_f_to_k(fahrenheit : f64) -> f64{
    let kelvin = temperature_convert_c_to_k(temperature_convert_f_to_c(fahrenheit));
    kelvin
}

fn temperature_convert_k_to_c(kelvin : f64) -> f64{
    let celsius = kelvin - 273.15;
    celsius
}

fn temperature_convert_k_to_f(kelvin : f64) -> f64{
    let fahrenheit = temperature_convert_c_to_f(temperature_convert_k_to_c(kelvin));
    fahrenheit
}