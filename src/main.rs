use std::io;

// Function to convert Fahrenheit to Celsius
fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

// Function to convert Celsius to Fahrenheit
fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

// Function to provide scale suggestions based on the Celsius value
fn suggest_celsius_context(c: f64) {
    if c < 0.0 {
        println!("It's freezing!");
    } else if c >= 0.0 && c <= 15.0 {
        println!("It's cool.");
    } else if c > 15.0 && c <= 30.0 {
        println!("It's warm.");
    } else {
        println!("It's hot!");
    }
}

// Function to provide scale suggestions based on the Fahrenheit value
fn suggest_fahrenheit_context(f: f64) {
    if f < 32.0 {
        println!("It's freezing!");
    } else if f >= 32.0 && f <= 59.0 {
        println!("It's cool.");
    } else if f > 59.0 && f <= 86.0 {
        println!("It's warm.");
    } else {
        println!("It's hot!");
    }
}

fn main() {
    // Initialize an empty vector to store conversion history
    // Each entry is a tuple of the input temperature as a string and the converted temperature as a string
    let mut history: Vec<(String, String)> = Vec::new();

    loop {
        // Display options to user
        println!("Temperature Converter");
        println!("1: Fahrenheit to Celsius");
        println!("2: Celsius to Fahrenheit");
        println!("3: View Conversion History");
        println!("4: Quit");

        // Get user input for conversion type
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        // Check for exit condition
        if choice.trim() == "4" {
            break;
        }

        // View conversion history
        if choice.trim() == "3" {
            println!("Conversion History:");
            for (input, converted) in &history {
                println!("{} -> {}", input, converted);
            }
            continue;
        }

        // Get temperature value from user
        println!("Enter the temperature value:");
        let mut temp_str = String::new();
        io::stdin().read_line(&mut temp_str).unwrap();

        // Validate user input
        let temp: f64 = match temp_str.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a numerical value.");
                continue;
            }
        };

        // Perform conversion and provide scale suggestions
        if choice.trim() == "1" {
            let converted = fahrenheit_to_celsius(temp);
            let rounded_converted = (converted * 10.0).round() / 10.0; // Round to one decimal place
            println!("Converted temperature: {:.1}°C", rounded_converted);
            suggest_celsius_context(rounded_converted);
            history.push((format!("{}°F", temp), format!("{:.1}°C", rounded_converted)));
        } else if choice.trim() == "2" {
            let converted = celsius_to_fahrenheit(temp);
            let rounded_converted = (converted * 10.0).round() / 10.0; // Round to one decimal place
            println!("Converted temperature: {:.1}°F", rounded_converted);
            suggest_fahrenheit_context(rounded_converted);
            history.push((format!("{}°C", temp), format!("{:.1}°F", rounded_converted)));
        } else {
            println!("Invalid choice. Please select 1, 2, 3, or 4.");
        }
    }
}
